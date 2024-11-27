use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::{message::Message, node::Node};

pub struct Network{
    nodes: HashMap<u32, Sender<Message>>,
    delay: u64,
    clock: u64,
    recv: Receiver<Message>
}

impl Network {
    pub fn new(nodes: Option<HashMap<u32, Sender<Message>>>, delay: u64, clock: u64, recv: Receiver<Message>) -> Self{
        Network { nodes: nodes.unwrap_or(HashMap::new()), delay, clock, recv }
    }

    pub fn add_node(&mut self, node: &Node){
        self.nodes.insert(node.identity, node.own_sender.clone());
    }

    pub async fn start_node(&mut self, node_id: u32){
        self.nodes.get_mut(&node_id)
        .unwrap()
        //.expect("Network.start_node: there should be a node here")
        .send(Message::start()).await.unwrap();
    }

    pub async fn start_all_nodes(&mut self){
        for (_node_id, node_sender) in self.nodes.iter_mut(){
            node_sender.send(Message::start()).await.unwrap();
        }
    }

    pub async fn broadcast(&mut self, delay: Option<u64>, sender: u32, message: Message){
        let msg_delay = delay.unwrap_or(self.delay);
        println!("Node {sender} with {msg_delay} sent: {message}");
        for (node_id, node_sender) in self.nodes.iter_mut(){
            if *node_id == sender{
                continue
            }
            node_sender.send(message.clone()).await.unwrap();
        }
    }

    pub async fn convey(&mut self, delay: u64, target: u32, message: &Message){
        println!("trying to reach {target}");
        self.nodes.get_mut(&target)
        .expect(&format!("failed to get node {}", target))
        .send(message.clone()).await.unwrap();
    }

    pub async fn run(mut self){
        loop{
            let received = self.recv.recv().await.expect("receiver closed");
            match received.payload.as_ref(){
                "terminating" => {self.nodes.remove(&received.sender);},
                _ => self.broadcast(None, received.sender, received).await
            }
        }
    } 
}


impl Drop for Network{
    fn drop(&mut self) {
        println!("Network terminated");
    }
}