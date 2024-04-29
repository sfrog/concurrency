use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Message {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Message>();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        println!("Consumer exiting");
        42
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    println!("Secret: {}", secret);

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Message>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Message::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("Producer {} exiting", idx);
            break;
        }
    }
    Ok(())
}

impl Message {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
