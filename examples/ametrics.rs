use std::thread;

use anyhow::Result;

use concurrency::AmapMetrics;
use rand::Rng as _;

fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "class0", "class1", "class2", "class3", "page1", "page2", "page3", "page4", "page5",
    ]);

    for i in 0..4 {
        task_worker(i, metrics.clone())?;
    }

    for _ in 0..4 {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(5));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));
            metrics.increment(format!("class{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..256);
            metrics.increment(format!("page{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}
