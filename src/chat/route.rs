use super::server;
use crate::cache::token::verification_value;
use crate::chat::model::{ChatMessage, ChatMessageType};
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use r2d2_redis::RedisConnectionManager;
use serde::Deserialize;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

type RedisPool = r2d2_redis::r2d2::Pool<RedisConnectionManager>;

#[derive(Deserialize)]
pub struct WebsocketInfo {
    pub token: String,
}

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
    info: web::Query<WebsocketInfo>,
    redis_pool: web::Data<RedisPool>,
) -> Result<HttpResponse, Error> {
    let redis_conn = &mut redis_pool
        .get()
        .expect("countn't get redis connection from pool");
    let v = verification_value::<crate::api::user::User>(info.token.clone(), redis_conn);
    match v {
        Ok(id) => {
            let session = WsChatSession {
                id: 0,
                hb: Instant::now(),
                addr: srv.get_ref().clone(),
                token: info.token.clone(),
                user: id,
            };
            ws::start(session, &req, stream)
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("token invalid")),
    }
}

struct WsChatSession {
    id: usize,
    hb: Instant,
    addr: Addr<server::ChatServer>,
    token: String,
    user: String,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        let name = self.user.clone();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
                name,
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<server::Message> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.text)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let msg: std::result::Result<ChatMessage, serde_json::Error> =
                    serde_json::from_str(text.as_str());
                match msg {
                    Ok(msg) => match msg.style {
                        ChatMessageType::OneToOne(id) => {
                            self.addr.do_send(server::StrP2PMessage {
                                id: self.user.clone(),
                                msg: msg.content.unwrap_or_default(),
                                other_id: id,
                            });
                            if let Some(message_id) = msg.message_id {
                                let ack =
                                    serde_json::to_string(&ChatMessage::ack(message_id)).unwrap();
                                ctx.text(ack);
                            }
                        }
                        ChatMessageType::RoomMessage(room) => {
                            self.addr.do_send(server::StrRoomMessage {
                                id: self.user.clone(),
                                msg: msg.content.unwrap_or_default(),
                                room: room,
                            });
                            if let Some(message_id) = msg.message_id {
                                let ack =
                                    serde_json::to_string(&ChatMessage::ack(message_id)).unwrap();
                                ctx.text(ack);
                            }
                        }
                        ChatMessageType::Broadcast => {
                            self.addr.do_send(server::StrBoardcastMessage {
                                id: self.user.clone(),
                                msg: msg.content.unwrap_or_default(),
                            });
                            if let Some(message_id) = msg.message_id {
                                let ack =
                                    serde_json::to_string(&ChatMessage::ack(message_id)).unwrap();
                                ctx.text(ack);
                            }
                        }
                        ChatMessageType::Join(room) => {
                            self.addr.do_send(server::Join {
                                id: self.id,
                                name: room,
                            });
                            if let Some(message_id) = msg.message_id {
                                let ack =
                                    serde_json::to_string(&ChatMessage::ack(message_id)).unwrap();
                                ctx.text(ack);
                            }
                        }
                        _ => (),
                    },
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary message"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}
