use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use queues::*;



// Basic information about sent message.
#[derive(Clone, Debug)]
pub struct SentMessage {
    pub id: Option<String>,
    pub timestamp: Option<u64>,
}

lazy_static! {
    // Sent OCPP messages hash map: message id => stringified message.
    static ref MESSAGES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    
    // Pending messages queue.
    static ref QUEUE: Mutex<Queue<String>> = Mutex::new(queue![]);
    // Last sent message.
    static ref LAST_SENT_MESSAGE: Mutex<SentMessage> = Mutex::new(SentMessage { id: None, timestamp: None });
}

pub fn set_message(key: String, value: String) {
    MESSAGES.lock().unwrap().insert(key, value);
}

pub fn get_message(key: &str) -> String {
    match MESSAGES.lock().unwrap().get(key) {
        Some(value) => value.to_string(),
        None => String::from(""),
    }
}



pub fn queue_size() -> usize {
    QUEUE.lock().unwrap().size()
}

pub fn queue_add(s: String) {
    match QUEUE.lock().unwrap().add(s) {
        Err(e) => println!("{:?}", e),
        _ => (),
    };
}

pub fn queue_pop() -> String {
    match QUEUE.lock().unwrap().remove() {
        Ok(res) => res,
        Err(_) => String::from(""),
    }
}

pub fn set_last_sent_message(id: String, timestamp: u64) {
    LAST_SENT_MESSAGE.lock().unwrap().id = Some(id);
    LAST_SENT_MESSAGE.lock().unwrap().timestamp = Some(timestamp);
}

pub fn get_last_sent_message() -> SentMessage {
    LAST_SENT_MESSAGE.lock().unwrap().clone()
}
