use std::fmt::Display;

#[derive(Clone)]
pub struct Message{
    pub sender: u32,
    pub clock: u64,
    pub payload: String
}

impl Display for Message{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", &self.sender, &self.clock, &self.payload)
    }
}

impl Default for Message{
    fn default() -> Self {
        Self { sender: Default::default(), clock: Default::default(), payload: Default::default() }
    }
}

impl Message{
    pub fn new(sender: u32, clock: u64, payload: String) -> Self {
        Self { sender, clock, payload}
    }
    pub fn ping() -> Self {
        Self { sender: Default::default(), clock: Default::default(), payload: "ping".to_string() }
    }
    pub fn pong() -> Self {
        Self { sender: Default::default(), clock: Default::default(), payload: "pong".to_string() }
    }
    /// Start or restart node operation
    pub fn start() -> Self {
        Self { sender: Default::default(), clock: Default::default(), payload: "start".to_string() }
    }
    /// Tell a node to lose connection
    pub fn disconnect() -> Self {
        Self { sender: Default::default(), clock: Default::default(), payload: "disconnect".to_string() }
    }
    /// Tell a node to shutdown without termination message
    pub fn kill() -> Self {
        Self { sender: Default::default(), clock: Default::default(), payload: "kill".to_string() }
    }
    /// Message to let the network know that a node is shutting down, to more easily simulate disconnects and intentional shutdowns separately
    pub fn terminating(sender: u32) -> Self {
        Self { sender, clock: Default::default(), payload: "terminating".to_string() }
    }
}
