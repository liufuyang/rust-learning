use std::thread;

use crossbeam::crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use tokio::runtime::Builder;
use tokio::time::delay_for;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Runtime for IO related tasks, give few cores for it
    let mut rt_fetching = Builder::new()
        .core_threads(2)
        .thread_name("async-io-thread-pool")
        .threaded_scheduler()
        .enable_time()
        .build()?;

    // Runtime for CPU intensive tasks, give more cores for it
    let mut rt_analyzing = Builder::new()
        .core_threads(10)
        .thread_name("cpu-intensive-thread-pool")
        .threaded_scheduler()
        .enable_time()
        .build()?;

    // Channels to pass on fetching (IO task) results
    let (chan_pages_s, chan_pages_r) = unbounded();
    // Channels to pass on analyzing (CPU task) results
    let (chan_data_s, chan_data_r) = bounded(16);

    rt_fetching.block_on(async {
        // simulating multiple IO fetch tasks, each one assign a worker, lots of workers go fetching
        for i in 1..=100 {
            tokio::spawn(get_content(i.to_string(), chan_pages_s.clone()));
        }
    });

    rt_analyzing.block_on(async {
        // simulating 10 workers doing CPU intensive tasks
        for _ in 0..10 {
            tokio::spawn(analyze_content(chan_pages_r.clone(), chan_data_s.clone()));
        }
    });

    println!("Hello, world!");
    drop(chan_pages_s);
    drop(chan_data_s); // close sender from current thread, so to allow collecting on receiver to continue (when other senders in other threads are dropped)

    let result_list: Vec<usize> = chan_data_r.iter().collect();
    println!("number of result_list: {}", result_list.len());
    let sum: usize = result_list.iter().sum::<usize>();
    println!("sum of result_list: {}", sum);

    Ok(())
}

async fn get_content(n: String, sender: Sender<String>) {
    // TODO: later on use some .await call to get real pages and then send content to sender

    // We can also simulate some CPU delay here
    // Resulting in each worker is delayed by 1s, 2 thread, 100 workers
    // with 100 data, should be finished within 1 seconds.
    delay_for(std::time::Duration::from_secs(1)).await;

    // If you try below with blocking the actual thread, the worker thread will be blocked
    // resulting only 2 tasks can be done for each second, as we have only 2 thread.
    // With 100 data, this will cost 50 seconds to finish this step
    // thread::sleep(std::time::Duration::from_secs(1));

    // Which means, in order for this async IO task works well, all internal
    // call here in this function should be async (with .await) and not making it blocking!

    println!(
        "[{}]: Async fetch and send page content {}...",
        thread::current().name().unwrap_or("?"),
        n
    );
    sender.send(n).expect("get_content send ok");
}

async fn analyze_content(receiver: Receiver<String>, sender: Sender<usize>) {
    while let Ok(page) = receiver.recv() {
        // TODO: later on switch this to real CPU intensive tasks
        // for now, only switching a string to usize as some analyze function
        let data: usize = page.parse().unwrap_or(0);

        // We can also simulate some CPU delay here
        // Resulting in each thread is delayed by 1s, 10 workers, 10 block thread,
        // with 100 data, should be finished within 10 seconds.
        delay_for(std::time::Duration::from_secs(1)).await;

        // As we have 10 workers and 10 thread, this would be the same effect above
        // thread::sleep(std::time::Duration::from_secs(1));

        // Thus, by design this method call can have CPU blocking call.

        println!(
            "[{}]: Async analysis and send data content {}...",
            thread::current().name().unwrap_or("?"),
            data
        );
        sender.send(data).expect("analyze_content send ok");
    }
}
