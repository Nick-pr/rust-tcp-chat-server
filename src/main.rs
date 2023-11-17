use std::net::SocketAddr;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
enum Event {
    Message(Box<[u8]>, SocketAddr),
}

async fn handle_connection(
    mut stream: TcpStream,
    socket_addr: SocketAddr,
    sender: broadcast::Sender<Event>,
    mut receiver: broadcast::Receiver<Event>,
) {
    tracing::info!("Connection on {} starting", socket_addr);

    let (stream_reader, mut stream_writer) = stream.split();
    let mut buf = String::new();

    let mut stream_reader = BufReader::new(stream_reader);

    loop {
        tokio::select! {
            _ =  stream_reader.read_line(&mut buf) => {
                if buf.trim() == "/end" {
                    break
                }
                sender.send(Event::Message(buf.as_bytes().into(), socket_addr)).unwrap();
                buf.clear();
            }
            event = receiver.recv() => {
                match event.unwrap() {
                    Event::Message(m,author_addr) => {
                        if author_addr == socket_addr {
                            continue
                        };
                        stream_writer.write(&m).await.unwrap();
                        stream_writer.flush().await.unwrap();
                    }
                }
            }
        }
    }
    tracing::info!("Connection on {} closing", socket_addr);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Thread to accept connections and push to channel
    let handle = std::thread::spawn(|| async {
        let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
        tracing::info!("Listening on {}", listener.local_addr().unwrap());
        let tokio_rt = tokio::runtime::Handle::current();
        let (sender, _) = broadcast::channel::<Event>(10);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let sender = sender.clone();
                    let receiver = sender.subscribe();
                    tokio_rt.spawn(async move {
                        handle_connection(stream, addr, sender, receiver).await
                    });
                }
                Err(err) => tracing::error!("{}", err),
            };
        }
    });

    handle.join().unwrap().await;
}
