use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{debug, trace};
use tokio::{
    net::TcpStream,
    sync::{mpsc, watch},
};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WebsocketWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WebsocketReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

struct WsWriterActor {
    write_to_ws: WebsocketWriter,
    read_from_channel: mpsc::Receiver<Message>,
}

impl WsWriterActor {
    fn new(write_to_ws: WebsocketWriter, read_from_channel: mpsc::Receiver<Message>) -> Self {
        WsWriterActor {
            write_to_ws,
            read_from_channel,
        }
    }

    async fn handle_msg(&mut self, msg: Message) {
        debug!("Outgoing msg: {}", msg);
        self.write_to_ws.send(msg).await.expect("send to ws");
    }
}

#[derive(Clone)]
pub struct WsWriterActorHandle {
    pub send_to_channel: mpsc::Sender<Message>,
}

impl WsWriterActorHandle {
    pub fn new(ws_writer: WebsocketWriter) -> Self {
        let (sender, receiver) = mpsc::channel(16);
        let actor = WsWriterActor::new(ws_writer, receiver);
        tokio::task::spawn(forward_to_ws(actor));
        Self {
            send_to_channel: sender,
        }
    }
}

async fn forward_to_ws(mut actor: WsWriterActor) {
    while let Some(msg) = actor.read_from_channel.recv().await {
        trace!("new msg in the channel forward to msg_handle");
        actor.handle_msg(msg).await;
    }
}

struct WsReaderActor {
    read_from_ws: WebsocketReader,
    write_to_channel: watch::Sender<String>,
}
impl WsReaderActor {
    fn new(read_from_ws: WebsocketReader, write_to_channel: watch::Sender<String>) -> Self {
        WsReaderActor {
            read_from_ws,
            write_to_channel,
        }
    }

    async fn handle_msg(&mut self, msg: String) {
        trace!("Forwarding msg {}", msg);
        self.write_to_channel
            .send(msg)
            .expect("Write to Watch Channel");
    }
}

#[derive(Clone)]
pub struct WsReaderActorHandle {
    pub recieve_from_channel: watch::Receiver<String>,
}

impl WsReaderActorHandle {
    pub fn new(ws_reader: WebsocketReader) -> Self {
        let (sender, receiver) = watch::channel(String::from("start"));
        let actor = WsReaderActor::new(ws_reader, sender);
        // spawn actor
        tokio::task::spawn(forward_to_channel(actor));
        Self {
            recieve_from_channel: receiver,
        }
    }
}

async fn forward_to_channel(mut actor: WsReaderActor) {
    while let Ok(msg) = actor
        .read_from_ws
        .next()
        .await
        .expect("read from websocket")
    {
        debug!("Incomming msg : {}", msg);
        actor.handle_msg(msg.to_string()).await;
    }
}
