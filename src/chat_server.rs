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
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomMessage {
    pub id: usize,
    pub msg: String,
    pub room: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct P2PMessage {
    pub id: usize,
    pub msg: String,
    pub other_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BoardcastMessage {
    pub id: usize,
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

pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
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
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message {
                            text: message.to_owned(),
                        });
                    }
                }
            }
        }
    }

    fn send_boardcast(&self, message: &str, skip_id: usize) {
        for (id, addr) in &self.sessions {
            if *id != skip_id {
                let _ = addr.do_send(Message {
                    text: message.to_owned(),
                });
            }
        }
    }

    fn send_p2p_message(&self, id: &usize, message: &str) {
        if let Some(addr) = self.sessions.get(id) {
            let _ = addr.do_send(Message {
                text: message.to_owned(),
            });
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
        self.sessions.insert(id, msg.addr);
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

        // for room in rooms {
        //     self.send_message(&room, "Someone disconnect", 0);
        // }
    }
}

impl Handler<RoomMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: RoomMessage, _: &mut Self::Context) {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}

impl Handler<P2PMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: P2PMessage, _: &mut Self::Context) {
        self.send_p2p_message(&msg.other_id, msg.msg.as_str());
    }
}

impl Handler<BoardcastMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: BoardcastMessage, _: &mut Self::Context) {
        self.send_boardcast(&msg.msg.as_str(), msg.id);
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
