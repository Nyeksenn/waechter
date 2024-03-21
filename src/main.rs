mod error;

use error::CommError;
use smol::prelude::*;
use smol::{Unblock, block_on};
use smol::io::{BufReader, BufWriter};
use smol::future::try_zip;
use smol::channel::{unbounded, Receiver, Sender};


async fn accept(tx: Sender<String>) -> Result<(), CommError> {
    let stdin = BufReader::new(Unblock::new(std::io::stdin()));

    let mut input_lines = stdin.lines();
    while let Some(line) = input_lines.next().await {
        let msg = line?;
        tx.send(msg).await?;
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
    match block_on(run()) {
        Ok(_) => println!("Exiting."),
        Err(_) => eprintln!("Something went wrong."),
    }
}
