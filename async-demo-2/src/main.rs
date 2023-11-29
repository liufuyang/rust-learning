use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;

use crossbeam::crossbeam_channel::{bounded, Receiver, Sender};
use rayon::prelude::*;
use reqwest;
use tokio::runtime::Builder;

type Map = HashMap<String, u32>;
type CityNumber = String;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Runtime for IO related tasks, give few cores for it
    let mut rt_fetching = Builder::new()
        .core_threads(4)
        .thread_name("async-io-thread-pool")
        .threaded_scheduler()
        .enable_time()
        .enable_io()
        .build()?;
    // Runtime for CPU intensive tasks, give more cores for it
    let mut rt_analyzing = Builder::new()
        .core_threads(12)
        .thread_name("cpu-intensive-thread-pool")
        .threaded_scheduler()
        .enable_time()
        .build()?;

    // Channels to pass on fetching (IO task) results
    let (chan_pages_s, chan_pages_r) = bounded(500);
    // Channels to pass on analyzing (CPU task) results
    let (chan_data_s, chan_data_r) = bounded(32);

    rt_fetching.block_on(async {
        // reading all the cities from file cities.txt and each one start a task to get the page content
        let f = File::open("async-demo-2/cities.txt").expect("Cannot open cities.txt");
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line.unwrap();
            let tokens: Vec<&str> = line.split(",").collect();
            let n = tokens[0];
            let city = tokens[1].replace(" ", "_");
            tokio::spawn(get_content(n.to_owned(), city, chan_pages_s.clone()));
        }
    });

    rt_analyzing.block_on(async {
        // using 10 workers doing CPU intensive tasks
        for _ in 0..12 {
            tokio::spawn(analyze_content(chan_pages_r.clone(), chan_data_s.clone()));
        }
    });

    drop(chan_pages_s);
    drop(chan_data_s); // close sender from current thread, so to allow collecting on receiver to continue (when other senders in other threads are dropped)

    let maps: Vec<Map> = chan_data_r.iter().collect();
    println!("number of maps: {}", maps.len());
    let map: Map = maps.into_par_iter().reduce_with(merge_maps).unwrap();
    println!("number of unique words: {}", map.len());

    Ok(())
}

async fn get_content(n: String, city: String, sender: Sender<(CityNumber, String)>) {
    let url = format!("https://en.wikipedia.org/wiki/{}", city);
    let body = reqwest::get(url.as_str())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!(
        "[{}] - {} - Page got for city: {}, character size {}",
        thread::current().name().unwrap_or("?"),
        n,
        city,
        body.len()
    );
    sender.send((n, body)).expect("get_content send error");
}

async fn analyze_content(receiver: Receiver<(CityNumber, String)>, sender: Sender<Map>) {
    while let Ok((n, body)) = receiver.recv() {
        let map = word_count(body.as_str());

        println!(
            "[{}]: Async analysis and send data content {}...",
            thread::current().name().unwrap_or("?"),
            n
        );
        sender.send(map).expect("analyze_content send error");
    }
}

fn merge_maps(mut a: Map, b: Map) -> Map {
    for (word, count) in b {
        *a.entry(word).or_insert(0) += count
    }
    a
}

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), count)
}

fn count(mut map: HashMap<String, u32>, word: String) -> HashMap<String, u32> {
    {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
}
