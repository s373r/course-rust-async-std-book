// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 3. Tutorial: Implementing a chat
//         ^ https://book.async.rs/tutorial/index.html
//    3.1. Specification and Getting started
//         ^ https://book.async.rs/tutorial/specification.html
//    3.2. Writing an Accept Loop
//         ^ https://book.async.rs/tutorial/accept_loop.html
//    3.3. Receiving Messages
//         ^ https://book.async.rs/tutorial/receiving_messages.html
//    3.4. Sending Messages
//         ^ https://book.async.rs/tutorial/sending_messages.html
//    3.5. Connecting Readers and Writers
//         ^ https://book.async.rs/tutorial/connecting_readers_and_writers.html
//    3.6. All Together
//         ^ https://book.async.rs/tutorial/all_together.html
//    3.7. Clean Shutdown
//         ^ https://book.async.rs/tutorial/clean_shutdown.html
//    3.8. Handling Disconnection
//         ^ https://book.async.rs/tutorial/handling_disconnection.html

use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use futures::channel::mpsc;
use futures::sink::SinkExt;
use std::{
    collections::hash_map::{Entry, HashMap},
    sync::Arc,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn broker_loop(mut events: Receiver<Event>) -> Result<()> {
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();

    while let Some(event) = events.next().await {
        match event {
            Event::Message { from, to, msg } => {
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);

                        peer.send(msg).await?
                    }
                }
            }
            Event::NewPeer { name, stream } => match peers.entry(name) {
                Entry::Occupied(..) => (),
                Entry::Vacant(entry) => {
                    let (client_sender, client_receiver) = mpsc::unbounded();
                    entry.insert(client_sender);

                    spawn_and_log_error(connection_writer_loop(client_receiver, stream));
                }
            },
        }
    }

    Ok(())
}

async fn connection_writer_loop(
    mut messages: Receiver<String>,
    stream: Arc<TcpStream>,
) -> Result<()> {
    let mut stream = &*stream;

    while let Some(msg) = messages.next().await {
        stream.write_all(msg.as_bytes()).await?;
    }

    Ok(())
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let (broker_sender, broker_receiver) = mpsc::unbounded();
    let _broker_handle = task::spawn(broker_loop(broker_receiver));
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        spawn_and_log_error(connection_loop(broker_sender.clone(), stream));
    }

    Ok(())
}

async fn connection_loop(mut broker: Sender<Event>, stream: TcpStream) -> Result<()> {
    let stream = Arc::new(stream);
    let reader = BufReader::new(&*stream);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };

    broker
        .send(Event::NewPeer {
            name: name.clone(),
            stream: Arc::clone(&stream),
        })
        .await
        .unwrap();

    while let Some(line) = lines.next().await {
        let line = line?;
        let (dest, msg) = match line.find(':') {
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1..].trim()),
        };
        let dest: Vec<String> = dest
            .split(',')
            .map(|name| name.trim().to_string())
            .collect();
        let msg: String = msg.to_string();

        broker
            .send(Event::Message {
                from: name.clone(),
                to: dest,
                msg,
            })
            .await
            .unwrap();
    }

    Ok(())
}

fn main() -> Result<()> {
    let fut = accept_loop("127.0.0.1:8080");

    task::block_on(fut)
}
