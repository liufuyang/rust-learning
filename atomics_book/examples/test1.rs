use tokio::io;
use tokio::sync::mpsc;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1);
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    tokio::spawn(async move {
            if let Err(_) = tx1.send(1).await {
                println!("receiver dropped");
                return;
            }
    });

    tokio::spawn(async move {
        if let Err(_) = tx2.send(2).await {
            println!("receiver dropped");
            return;
        }
    });

    drop(tx);

    loop {
        tokio::select! {
            Some(i) = rx.recv() => println!("got = {}", i),
            _ = sigterm() => {println!("got sigterm"); break},
            _ = sigint() => {println!("got sigint"); break},
        }
    }
}

pub async fn sigterm() -> io::Result<()> {
    signal(SignalKind::terminate())?.recv().await;
    Ok(())
}
pub async fn sigint() -> io::Result<()> {
    signal(SignalKind::interrupt())?.recv().await;
    Ok(())
}


