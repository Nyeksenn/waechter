/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod error;

use error::CommError;
use smol::prelude::*;
use smol::{Unblock, block_on, spawn};
use smol::io::{BufReader, BufWriter};
use smol::future::try_zip;
use smol::channel::{unbounded, Receiver, Sender};
use std::path::Path;
use notify::{Config, RecommendedWatcher, Watcher, Event, Error};


async fn accept(tx: Sender<String>) -> Result<(), CommError> {
    let stdin = BufReader::new(Unblock::new(std::io::stdin()));

    let mut input_lines = stdin.lines();
    while let Some(line) = input_lines.next().await {
        let tx = tx.clone();
        let in_str = line?;
        let cmd: Vec<&str> = in_str.split(':').collect();
        let path_str = cmd[1].to_string();
        if cmd.len() == 2 && ["add", "remove"].contains(&cmd[0]) {
            spawn(async move {
                let path = Path::new(path_str.as_str());
                let mut watcher = RecommendedWatcher::new(
                    move |res: Result<Event, Error>| {
                        block_on(async  {
                            if let Ok(ev) = res {
                                match ev.kind {
                                    notify::EventKind::Create(_) => tx.send(format!("add:{}", ev.paths[0].display())).await.unwrap(),
                                    notify::EventKind::Modify(_) => tx.send(format!("change:{}", ev.paths[0].display())).await.unwrap(),
                                    notify::EventKind::Remove(_) => tx.send(format!("delete:{}", ev.paths[0].display())).await.unwrap(),
                                    _ => todo!()
                                }
                            }
                        })
                    },
                    Config::default(),
                ).unwrap();
                let _ = watcher.watch(path, notify::RecursiveMode::NonRecursive);
            }).detach();
        }
    }

    Ok(())
}

async fn notify(rx: Receiver<String>) -> Result<(), CommError> {
    let mut stdout = BufWriter::new(Unblock::new(std::io::stdout()));

    while let Ok(msg) = rx.recv().await {
        stdout.write_all(msg.as_bytes()).await?;
        stdout.write(b"\n").await?;
        stdout.flush().await?;
    }
    Ok(())
}

async fn run() -> Result<(), CommError> {
    let (tx, rx) = unbounded::<String>();
    try_zip(accept(tx), notify(rx)).await?;
    Ok(())
}

fn main() {
    match block_on(spawn(run())) {
        Ok(_) => println!("Exiting."),
        Err(_) => eprintln!("Something went wrong."),
    }
}
