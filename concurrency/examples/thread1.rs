use anyhow::{anyhow, Result};

use std::{thread, sync::mpsc};
use std::sync::mpsc::Sender;
use std::time::Duration;

const NUM_PRODUCERS: usize = 4;

#[derive(Debug) ]
struct Msg {
     idx: usize,
    value: i32,
}
fn main() -> Result<()> {

    // 创建 producer
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || {producer(i, tx)});
    }
    // 创建 consumer
    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("{:?}", msg);
        }
    });

    consumer.join().map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    Ok(())
}

fn producer(index: usize, tx: Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<i32>();
        tx.send(Msg::new(index, value))?;
        let sleep = rand::random::<u8>() as u64 * 10 ;
        thread::sleep(Duration::from_millis(sleep));
    }
}

impl Msg {
    fn new(idx: usize, value: i32) -> Msg {
        Self {idx, value}
    }

}
