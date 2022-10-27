use std::env;

use ws::{util::Token, Handler, Request, Sender};

macro_rules! block {
    ($xs:block) => {
        loop {
            let _ = $xs;
            break;
        }
    };
}

static mut HEARTBEAT_INTERVAL: u64 = 0;

// Timeout events
const HEARBEAT: Token = Token(1);
const QUEUE_FETCH: Token = Token(2);

const QUEUE_FETCH_INTERVAL: u64 = 50;
const QUEUE_MESSAGE_EXPIREATION: u64 = 10;

pub struct Client {
    pub out: Sender,
}

impl Handler for Client {
    fn build_request(&mut self, url: &url::Url) -> ws::Result<ws::Request> {
        let mut req = Request::from_url(url).unwrap();
        req.add_protocol("ocpp2.0.1");
        Ok(req)
    }

    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.out.timeout(QUEUE_FETCH_INTERVAL, QUEUE_FETCH)?;

        let model: String = match env::var("MODEL") {
            Ok(var) => {
                if var == "" {
                    "Model".to_string()
                } else {
                    var
                }
            }
            _ => "Model".to_string(),
        };

        let vendor_name: String = match env::var("VENDOR_NAME") {
            Ok(var) => {
                if var == "" {
                    "VendorName".to_string()
                } else {
                    var
                }
            }
            _ => "VendorName".to_string(),
        };

        Ok(())
    }
}
