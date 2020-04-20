use super::model::{ChatMessage, ChatMessageType};
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap};
use log::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message {
    pub text: String,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StrRoomMessage {
    pub id: String,
    pub ids: Vec<String>,
    pub msg: String,
    pub room: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct P2PMessage {
    pub id: String,
    pub msg: String,
    pub other_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BoardcastMessage {
    pub id: String,
    pub msg: String,
}

struct Session(Recipient<Message>, String);

pub struct ChatServer {
    sessions: HashMap<usize, Session>,
    rng: ThreadRng,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl ChatServer {
    fn send_messages(&self, users: &Vec<String>, message: &str, skip_id: String) {
                for (_, sess) in &self.sessions {
                    if (users.contains(&sess.1) && sess.1 != skip_id) {
                        let _ = sess.0.do_send(Message {
                            text: message.to_owned(),
                        });
                    }
                }
        }


    fn send_boardcart(&self, message: &str, skip_id: String) {
        for (_, sess) in &self.sessions {
            if sess.1 != skip_id {
                let _ = sess.0.do_send(Message {
                    text: message.to_owned(),
                });
            }
        }
    }

    fn send_p2p_message(&self, id: String, message: &str) {
        for (_, sess) in &self.sessions {
            if sess.1 == id {
                let _ = sess.0.do_send(Message {
                    text: message.to_owned(),
                });
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        log::debug!("{} is connected", msg.name);
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, Session(msg.addr, msg.name));
        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        let _ = self.sessions.remove(&msg.id);
    }
}

impl Handler<StrRoomMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: StrRoomMessage, _: &mut Self::Context) {
        let skip_id = msg.id.clone();
        let send_msg = ChatMessage {
            from: Some(msg.id),
            style: ChatMessageType::RoomMessage(msg.room),
            content: Some(msg.msg),
            message_id: None,
        };
        let send_str = serde_json::to_string(&send_msg).unwrap();
        self.send_messages(&msg.ids, send_str.as_str(), skip_id);
    }

}

impl Handler<P2PMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: P2PMessage, _: &mut Self::Context) {
        let other_id = msg.other_id.clone();
        let send_msg = ChatMessage {
            from: Some(msg.id),
            style: ChatMessageType::OneToOne(msg.other_id),
            content: Some(msg.msg),
            message_id: None,
        };
        let send_str = serde_json::to_string(&send_msg).unwrap();
        self.send_p2p_message(other_id, send_str.as_str());
    }
}

impl Handler<BoardcastMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: BoardcastMessage, _: &mut Self::Context) {
        let send_msg = ChatMessage {
            from: Some(msg.id),
            style: ChatMessageType::Broadcast,
            content: Some(msg.msg),
            message_id: None,
        };
        let send_str = serde_json::to_string(&send_msg).unwrap();
        self.send_boardcart(&send_str.as_str(), send_msg.from.unwrap());
    }
}