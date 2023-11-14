use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc;

enum Message {}

async fn handle_connection(_send_channel: mpsc::Sender<Message>, socket_addr: SocketAddr) {
    tracing::info!("Connection on {} starting", socket_addr);
    tracing::info!("Connection on {} closing", socket_addr);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    let rt = tokio::runtime::Handle::current();

    let (sender, recv) = mpsc::channel::<Message>();

    std::thread::spawn(move || loop {
        match listener.accept() {
            Ok((_, socket_addr)) => {
                let sender = sender.clone();
                rt.spawn(async move { handle_connection(sender, socket_addr).await });
            }
            Err(err) => tracing::error!("{}", err),
        };
    });

    for _message in recv {
        // Todo
    }
}
