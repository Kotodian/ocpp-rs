use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use queues::*;

use super::{
    enums::{Action, Payload},
    rpc::Call,
};

// Basic information about sent message.
#[derive(Clone, Debug)]
pub struct SentMessage {
    pub id: Option<String>,
    pub timestamp: Option<u64>,
}

#[derive(Clone)]
pub struct ActionPayload {
    pub action: Action,
    pub payload: Payload,
}

// Sent OCPP messages hash map: message id => stringified message.
static MESSAGES: Lazy<Mutex<HashMap<String, ActionPayload>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// Pending messages queue.
static QUEUE: Lazy<Mutex<Queue<Call>>> = Lazy::new(|| Mutex::new(queue![]));
// Last sent message.
static LAST_SENT_MESSAGE: Lazy<Mutex<SentMessage>> = Lazy::new(|| {
    Mutex::new(SentMessage {
        id: None,
        timestamp: None,
    })
});

pub fn set_message(key: String, value: ActionPayload) {
    MESSAGES.lock().unwrap().insert(key, value);
}

pub fn get_message(key: &str) -> Option<ActionPayload> {
    match MESSAGES.lock().unwrap().get(key) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn queue_size() -> usize {
    QUEUE.lock().unwrap().size()
}

pub fn queue_add(s: Call) {
    match QUEUE.lock().unwrap().add(s) {
        Err(e) => println!("{:?}", e),
        _ => (),
    };
}

pub fn queue_pop() -> Option<Call> {
    match QUEUE.lock().unwrap().remove() {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}

pub fn set_last_sent_message(id: String, timestamp: u64) {
    LAST_SENT_MESSAGE.lock().unwrap().id = Some(id);
    LAST_SENT_MESSAGE.lock().unwrap().timestamp = Some(timestamp);
}

pub fn get_last_sent_message() -> SentMessage {
    LAST_SENT_MESSAGE.lock().unwrap().clone()
}
