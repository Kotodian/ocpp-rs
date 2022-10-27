use std::{collections::HashMap, fmt::write};

use futures_util::StreamExt;
use log::{debug, info, trace};
use ocpp_json::v2_0_1::{data_types::ChargingStationType, payloads::BootNotificationRequest, enums::ReasonEnumType};
use ocpp_rpc::ocpp2_0_1::json::{enums::Action, rpc::Call};
use rand::SeedableRng;
use tokio_tungstenite::connect_async;
use url::Url;
use uuid::Uuid;

use crate::actor::{WsReaderActorHandle, WsWriterActorHandle};

pub async fn run_ocpp_simulator(sn: String, link_url: String) {
    let ws_url = Url::parse(&format!("{}/{}", link_url, sn)).expect("url parse error");
    let mut msg_id_hashmap = HashMap::<String, Action>::new();
    let mut heartbeat_duration: i64 = 30;
    let mut metervalue_flag: bool = false;
    let mut metervalue_duration: i64 = 60;
    let mut rng = rand::rngs::StdRng::from_entropy();

    debug!("Establish Websocket Connection");
    let (socket, _) = connect_async(ws_url).await.expect("Failed to connect");
    let (send_socket, read_socket) = socket.split();
    info!("WS Established");

    let write_actor = WsWriterActorHandle::new(send_socket);
    let ocpp_writer = write_actor.clone();
    let hearbeat_writer = write_actor.clone();

    let read_actor = WsReaderActorHandle::new(read_socket);
    let ocpp_reader = read_actor.clone();

    let writer = ocpp_writer.send_to_channel;
    let mut reader = ocpp_reader.recieve_from_channel;

    debug!("Powering Charger");

    trace!("Create BootNotification Request Msg");
    let boot_notification_payload = BootNotificationRequest {
        charging_station: ChargingStationType {
            serial_number: Some(sn),
            model: "TestCS".to_owned(),
            vendor_name: "TestCS".to_owned(),
            firmware_version: Some("v0.0.1".to_owned()),
            modm: None,
        },
        reason: ocpp_json::v2_0_1::enums::BootReasonEnumType::PowerUp,
    };
    
}
