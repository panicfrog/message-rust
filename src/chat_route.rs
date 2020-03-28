use crate::chat_server;
use crate::chat_model::{ChatMessage, ChatMessageType};
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<chat_server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let session = WsChatSession {
        id: 0,
        hb: Instant::now(),
        addr: srv.get_ref().clone(),
    };
    ws::start(session, &req, stream)
}

struct WsChatSession {
    id: usize,
    hb: Instant,
    addr: Addr<chat_server::ChatServer>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        // TODO wait for auth, if unauth close connecting
        let addr = ctx.address();
        self.addr
            .send(chat_server::Connect {
                addr: addr.recipient(),
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
        self.addr.do_send(chat_server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<chat_server::Message> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: chat_server::Message, ctx: &mut Self::Context)
    {
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
                            self.addr.do_send(chat_server::P2PMessage {
                                id: self.id,
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
                            self.addr.do_send(chat_server::RoomMessage {
                                id: self.id,
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
                            self.addr.do_send(chat_server::BoardcastMessage {
                                id: self.id,
                                msg: msg.content.unwrap_or_default(),
                            });
                            if let Some(message_id) = msg.message_id {
                                let ack =
                                    serde_json::to_string(&ChatMessage::ack(message_id)).unwrap();
                                ctx.text(ack);
                            }
                        }
                        ChatMessageType::Join(room) => {
                            self.addr.do_send(chat_server::Join {
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
