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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        let _handle = task::spawn(connection_loop(stream)); // 1
    }

    Ok(())
}

async fn connection_loop(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&stream); // 2
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    println!("name = {}", name);

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
    }
    Ok(())
}

fn main() -> Result<()> {
    let fut = accept_loop("127.0.0.1:8080");

    task::block_on(fut)
}
