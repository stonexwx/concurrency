use anyhow::{anyhow, Result};

const THREAD_COUNT: usize = 10;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: usize,
    value: usize,
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    for i in 0..THREAD_COUNT {
        let tx = tx.clone();
        std::thread::spawn(move || worker(tx, i));
    }

    drop(tx);

    let consumer = std::thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        println!("Consumer exit");
    });

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join err:{:?}", e))?;

    Ok(())
}

fn worker(tx: std::sync::mpsc::Sender<Msg>, id: usize) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(id, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        std::thread::sleep(std::time::Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 10 == 0 {
            println!("Thread {} exit", id);
            return Ok(());
        }
    }
}
