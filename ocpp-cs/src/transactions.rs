use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

static TRANSACTIONS: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn set_transaction(key: String, value: String) {
    TRANSACTIONS.lock().unwrap().insert(key, value);
}

pub fn get_transaction(key: &str) -> String {
    match TRANSACTIONS.lock().unwrap().get(key) {
        Some(value) => value.to_string(),
        None => String::from(""),
    }
}

pub fn delete_transaction(key: &str) {
    TRANSACTIONS.lock().unwrap().remove(key);
}
