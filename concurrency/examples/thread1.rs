use anyhow::Result;

use std::{thread, sync::mpsc};

fn main() -> Result<()> {

    println!("Starting thread1");
    let (tx, rx) = mpsc::channel();
    let producer = thread::spawn(move || {
        tx.send("hello").unwrap();
    })
    Ok(())
}