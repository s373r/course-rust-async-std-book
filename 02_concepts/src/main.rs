// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 2.2. Tasks

fn main() {
    // https://book.async.rs/concepts/tasks.html
    println!("--- 2.2. Tasks ---");
    {
        use async_std::{fs::File, io, prelude::*, task};

        async fn read_file(path: &str) -> io::Result<String> {
            let mut file = File::open(path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            Ok(contents)
        }

        let reader_task = task::spawn(async {
            let result = read_file("data.csv").await;
            match result {
                Ok(s) => println!("{}", s),
                Err(e) => println!("Error reading file: {:?}", e),
            }
        });
        println!("Started task!");
        task::block_on(reader_task);
        println!("Stopped task!");
    }
}
