use super::model::{ChatMessage, ChatMessageType};
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};

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
    pub msg: String,
    pub room: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StrP2PMessage {
    pub id: String,
    pub msg: String,
    pub other_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StrBoardcastMessage {
    pub id: String,
    pub msg: String,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    pub name: String,
}

struct Session(Recipient<Message>, String);

pub struct ChatServer {
    sessions: HashMap<usize, Session>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        let rooms = HashMap::new();
        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
        }
    }
}

impl ChatServer {
    fn send_message(&self, room: &str, message: &str, skip_id: String) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if let Some(sess) = self.sessions.get(id) {
                    if sess.1 != skip_id {
                        let _ = sess.0.do_send(Message {
                            text: message.to_owned(),
                        });
                    }
                }
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
        println!("Someone joined");
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, Session(msg.addr, msg.name));
        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        println!("Someone disconnet");
        let mut rooms: Vec<String> = Vec::new();
        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned())
                }
            }
        }
    }
}

impl Handler<StrRoomMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: StrRoomMessage, _: &mut Self::Context) {
        let room = msg.room.clone();
        let skip_id = msg.id.clone();
        let send_msg = ChatMessage {
            from: Some(msg.id),
            style: ChatMessageType::RoomMessage(msg.room),
            content: Some(msg.msg),
            message_id: None,
        };
        let send_str = serde_json::to_string(&send_msg).unwrap();
        self.send_message(&room, send_str.as_str(), skip_id);
    }
}

impl Handler<StrP2PMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: StrP2PMessage, _: &mut Self::Context) {
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

impl Handler<StrBoardcastMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: StrBoardcastMessage, _: &mut Self::Context) {
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

impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Self::Context) -> Self::Result {
        let mut rooms = Vec::new();
        for key in self.rooms.keys() {
            rooms.push(key.to_owned());
        }
        MessageResult(rooms)
    }
}

impl Handler<Join> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Join, _: &mut Self::Context) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        // TODO: not remove the sesson in rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }

        if self.rooms.get_mut(&name).is_none() {
            self.rooms.insert(name.clone(), HashSet::new());
        }
        // self.send_message(&name, "Someone connect", id);
        self.rooms.get_mut(&name).unwrap().insert(id);
    }
}
