use chrono::Utc;
use ocpp_json::v2_0_1::{
    data_types::ChargingStationType,
    enums::{BootReasonEnumType, ConnectorStatusEnumType},
    payloads::{BootNotificationRequest, BootNotificationResponse, HeartbeatRequest},
};
use ocpp_rpc::json::{
    enums::{Action, MessageType, Payload},
    queue::{
        self, get_last_sent_message, queue_add, queue_pop, queue_size, set_last_sent_message,
        set_message, ActionPayload,
    },
    rpc::Call,
};
use std::env;
use uuid::Uuid;
use ws::{util::Token, Error, ErrorKind, Handler, Message, Request, Sender};

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
const HEARTBEAT: Token = Token(1);
const QUEUE_FETCH: Token = Token(2);

const QUEUE_FETCH_INTERVAL: u64 = 50;
const QUEUE_MESSAGE_EXPIREATION: u64 = 10;

pub struct Client {
    pub out: Sender,
}

impl Client {}

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

        let serial_number: Option<String> = match env::var("SERIAL_NUMBER") {
            Ok(data) => Some(data),
            _ => None,
        };

        // send BootNotification first
        let msg_id = &Uuid::new_v4().to_string();
        let payload = BootNotificationRequest {
            charging_station: ChargingStationType {
                serial_number,
                model,
                vendor_name,
                firmware_version: None,
                modem: None,
            },
            reason: BootReasonEnumType::PowerUp,
        }
        .into();
        let action = Action::BootNotification;
        let call = Call::new(msg_id.to_string(), action, payload);

        queue::set_message(
            msg_id.to_string(),
            ActionPayload {
                action: call.action.clone(),
                payload: call.payload.clone(),
            },
        );
        queue::queue_add(call);

        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let ocpp_message: MessageType = match serde_json::from_str(msg.as_text()?) {
            Ok(result) => result,
            Err(e) => panic!("Error during parsing: {:?}", e),
        };

        match ocpp_message {
            MessageType::Call(_, _, action, payload) => block!({}),
            MessageType::CallResult(_, msg_id, payload) => block!({
                let msg = queue::get_message(msg_id.as_str());
                if msg.is_none() {
                    // todo
                } else {
                    let msg = msg.unwrap();
                    match msg.action {
                        Action::Authorize => todo!(),
                        Action::BootNotification => {
                            let payload: BootNotificationResponse = serde_json::from_value(payload)
                                .expect("expect boot notification response");
                            // todo status notification

                            unsafe {
                                HEARTBEAT_INTERVAL = payload.interval * 1000;
                                self.out.timeout(HEARTBEAT_INTERVAL, HEARTBEAT)?;
                            }
                        }
                        Action::CancelReservation => todo!(),
                        Action::CertificateSigned => todo!(),
                        Action::ChangeAvailability => todo!(),
                        Action::ClearCache => todo!(),
                        Action::ClearChargingProfile => todo!(),
                        Action::ClearDisplayMessage => todo!(),
                        Action::ClearedChargingLimit => todo!(),
                        Action::ClearVariableMonitoring => todo!(),
                        Action::CostUpdated => todo!(),
                        Action::CustomerInformation => todo!(),
                        Action::DataTransfer => todo!(),
                        Action::DeleteCertificate => todo!(),
                        Action::FirmwareStatusNotification => todo!(),
                        Action::Get15118EVCertificate => todo!(),
                        Action::GetBaseReport => todo!(),
                        Action::GetCertificateStatus => todo!(),
                        Action::GetChargingProfile => todo!(),
                        Action::GetCompositeSchedule => todo!(),
                        Action::GetDisplayMessage => todo!(),
                        Action::GetInstalledCertificateIds => todo!(),
                        Action::GetLocalListVersion => todo!(),
                        Action::GetLog => todo!(),
                        Action::GetMonitoringReport => todo!(),
                        Action::GetReport => todo!(),
                        Action::GetTransactionStatus => todo!(),
                        Action::GetVariables => todo!(),
                        Action::Heartbeat => todo!(),
                        Action::InstallCertificate => todo!(),
                        Action::LogStatusNotification => todo!(),
                        Action::MeterValues => todo!(),
                        Action::NotifyChargingLimit => todo!(),
                        Action::NotifyCustomerInformation => todo!(),
                        Action::NotifyDisplayMessages => todo!(),
                        Action::NotifyEVChargingNeeds => todo!(),
                        Action::NotifyEVChargingSchedule => todo!(),
                        Action::NotifyEvent => todo!(),
                        Action::NotifyMonitoringReport => todo!(),
                        Action::NotifyReport => todo!(),
                        Action::PublishFirmware => todo!(),
                        Action::PublishFirmwareStatusNotification => todo!(),
                        Action::ReportChargingProfiles => todo!(),
                        Action::RequestStartTransaction => todo!(),
                        Action::RequestStopTransaction => todo!(),
                        Action::ReservationStatusUpdate => todo!(),
                        Action::ReserveNow => todo!(),
                        Action::Reset => todo!(),
                        Action::SecurityEventNotification => todo!(),
                        Action::SendLocalList => todo!(),
                        Action::SetChargingProfile => todo!(),
                        Action::SetDisplayMessage => todo!(),
                        Action::SetMonitoringBase => todo!(),
                        Action::SetMonitoringLevel => todo!(),
                        Action::SetNetworkProfile => todo!(),
                        Action::SetVariableMonitoring => todo!(),
                        Action::SetVariables => todo!(),
                        Action::SignCertificate => todo!(),
                        Action::StatusNotification => todo!(),
                        Action::TransactionEvent => todo!(),
                        Action::TriggerMessage => todo!(),
                        Action::UnlockConnector => todo!(),
                        Action::UnpublishFirmware => todo!(),
                        Action::UpdateFirmware => todo!(),
                    }
                }
            }),
            MessageType::CallError(_, msg_id, code, description, _) => block!({ todo!() }),
        };

        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        // todo log
        self.out.shutdown().unwrap();
    }

    fn on_error(&mut self, err: ws::Error) {
        // todo log
        self.out.shutdown().unwrap();
    }

    fn on_timeout(&mut self, event: Token) -> ws::Result<()> {
        match event {
            HEARTBEAT => {
                let msg_id = &Uuid::new_v4().to_string();
                let payload = HeartbeatRequest {}.into();

                let call = Call::new(msg_id.to_string(), Action::Heartbeat, payload);
                set_message(
                    msg_id.to_string(),
                    ActionPayload {
                        action: call.action.clone(),
                        payload: call.payload.clone(),
                    },
                );
                queue_add(call);
                unsafe {
                    self.out.timeout(HEARTBEAT_INTERVAL, HEARTBEAT)?;
                }

                Ok(())
            }
            QUEUE_FETCH => {
                let current_timestamp = Utc::now().timestamp() as u64;
                let last_sent_msg = get_last_sent_message();
                let last_sent_msg_exist = last_sent_msg.id != None;
                let last_sent_msg_expired = match last_sent_msg.timestamp {
                    Some(timestamp) => timestamp + QUEUE_MESSAGE_EXPIREATION > current_timestamp,
                    None => true,
                };

                if queue_size() > 0 && (!last_sent_msg_exist || last_sent_msg_expired) {
                    let call = queue_pop();

                    match call {
                        Some(call) => {
                            let msg = serde_json::to_string(&call).expect("serialize the call");
                            self.out.send(msg)?;
                            // todo log
                            set_last_sent_message(call.message_id.to_string(), current_timestamp);
                        }
                        None => {}
                    };
                }

                self.out.timeout(QUEUE_FETCH_INTERVAL, QUEUE_FETCH)?;

                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::Internal,
                "Invalid timeout token encountered!",
            )),
        }
    }
}
