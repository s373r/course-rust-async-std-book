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
    net::{TcpListener, ToSocketAddrs}, // 3
    prelude::*,                        // 1
    task,                              // 2
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        // TODO
    }

    Ok(())
}

// main
fn main() -> Result<()> {
    let fut = accept_loop("127.0.0.1:8080");

    task::block_on(fut)
}
