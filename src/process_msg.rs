use crate::message::Message;

/// Processor for more modular processing of messages

pub enum MessageProcessor {
    PRINT,
    PINGPONG
}

pub fn print_processor(message: &Message){
    println!("{message}")
}