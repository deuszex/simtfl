use std::collections::HashMap;

use message::Message;
use network::Network;
use node::Node;
use process_msg::MessageProcessor;

pub mod node;
pub mod network;
pub mod message;
pub mod process_msg;
pub mod utils;

#[tokio::main]
async fn main() {
    let (sendr, recv) = tokio::sync::mpsc::channel::<Message>(10);
    let mut initial_nodes = HashMap::new();
    for i in 0..10{
        let node = Node::new(i, 0, MessageProcessor::PINGPONG, sendr.clone());
        initial_nodes.insert(i, node.own_sender.clone());
        tokio::spawn(node.run());
    }
    let mut network = Network::new(Some(initial_nodes), 1, 0, recv);
    let printer = Node::new(11, 10, MessageProcessor::PRINT,sendr.clone());
    network.add_node(&printer);
    tokio::spawn(printer.run());
    
    network.broadcast(None, 1, Message::default()).await;
    tokio::spawn(network.run());

}
