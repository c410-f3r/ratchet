use bytes::BytesMut;
use ratchet::{Error, Message, NoExtProxy, PayloadType, ProtocolRegistry, WebSocketConfig};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept(stream));
    }
}

async fn accept(stream: TcpStream) {
    if let Err(e) = run(stream).await {
        println!("{:?}", e);
    }
}

async fn run(stream: TcpStream) -> Result<(), Error> {
    let mut websocket = ratchet::accept(
        stream,
        WebSocketConfig::default(),
        NoExtProxy,
        ProtocolRegistry::default(),
    )
    .await
    .unwrap()
    .upgrade()
    .await?
    .socket;

    let mut buf = BytesMut::new();

    loop {
        match websocket.read(&mut buf).await? {
            Message::Text => {
                let _s = String::from_utf8(buf.to_vec())?;
                websocket.write(&mut buf, PayloadType::Text).await?;
                buf.clear();
            }
            Message::Binary => {
                websocket.write(&mut buf, PayloadType::Binary).await?;
                buf.clear();
            }
            Message::Ping | Message::Pong => {}
            Message::Close(_) => return Ok(()),
        }
    }
}