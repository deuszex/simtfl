use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{message:: Message, process_msg::{print_processor, MessageProcessor}, utils::hash};

pub enum NodeState{
    /// This node doesn't run at all
    STOPPED,
    /// Node is running on the network, and processing messages
    RUNNING,
    /// Node is running, but has lost "connection", so it's unaffected by messages until it "reconnects"
    DISCONNECTED
}

pub struct Node{
    pub identity: u32,
    pub from_network: Receiver<Message>,
    pub own_sender: Sender<Message>,
    pub clock: u64,
    pub state: NodeState,
    pub message_processor: MessageProcessor,
    pub handled: HashMap<[u8;32], Message>,
    pub to_network: Option<Sender<Message>>
}

impl Default for Node{
    fn default() -> Self {
        let (own_sender, from_network) = tokio::sync::mpsc::channel::<Message>(10);
        Self { identity: 0, from_network, own_sender, clock: 0, state: NodeState::STOPPED, message_processor: MessageProcessor::PRINT, handled: HashMap::default(), to_network: None}
    }
}

impl Node{
    pub fn new(identity: u32, clock: u64, message_processor: MessageProcessor, sendr: Sender<Message>)-> Self {
        let (own_sender, from_network) = tokio::sync::mpsc::channel::<Message>(10);
        Self { identity, from_network, own_sender, clock, state: NodeState::STOPPED, message_processor, handled: HashMap::new(), to_network: Some(sendr) }
    }

    pub fn network_join(mut self, to_network: Sender<Message>){
        self.to_network = Some(to_network);
    }

    pub fn start_node(&mut self){
        self.state = NodeState::RUNNING
    }

    pub fn disconnect_node(&mut self){
        self.state = NodeState::DISCONNECTED
    }

    pub async fn receive(&mut self, message: &Message){
        println!("Node {} received message", self.identity);
        self.process(message).await;
    }

    pub async fn process(&mut self, message: &Message){
        match self.message_processor {
            MessageProcessor::PRINT => print_processor(&message),
            MessageProcessor::PINGPONG => self.to_network.as_mut().expect("NodeNotOnNetwork").send(Message::pong()).await.expect("channel corrupted"),
        }
        self.handled.insert(hash(&message.to_string().as_bytes()), message.clone());
    }

    pub async fn send_message(&mut self, message_body: String){
        self.to_network.as_mut().expect("NodeNotOnNetwork").send(Message::new(self.identity, self.clock, message_body)).await.expect("channel corrupted")
    }

    pub async fn run(mut self){
        loop{
            let received = self.from_network.recv().await.expect("receiver closed");
            self.process(&received).await;
        }
    } 
}

impl Drop for Node{
    fn drop(&mut self) {
        println!("Node {} terminated", self.identity);
    }
}