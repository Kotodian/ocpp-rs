use std::sync::Mutex;

use lazy_static::lazy_static;

// Connector struct.
#[derive(Clone, Debug)]
pub struct Connector {
    pub status: &'static str,
    pub operational: bool,
}

lazy_static! {
    static ref EVSES: Mutex<[[Connector; 1]; 1]> = Mutex::new([[Connector {
        status: "Inoperative",
        operational: true
    }]]);
}
// Array of EVSE each item of which contains an array of connectors.

pub fn set_connector_status(evse_index: usize, connector_index: usize, value: &'static str) {
    EVSES.lock().unwrap()[evse_index][connector_index].status = value;
}
// NOTE Unused.
// pub fn set_connector_operational_status(evse_index: usize, connector_index: usize, value: bool) {
//     EVSES.lock().unwrap()[evse_index][connector_index].operational = value;
// }

pub fn get_connector(evse_index: usize, connector_index: usize) -> Connector {
    EVSES.lock().unwrap()[evse_index][connector_index].clone()
}
