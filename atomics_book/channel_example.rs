use atomics_book::channel::simple_channel::Channel as SimpleChannel;
use std::thread;

fn main() {
    let simple_channel = SimpleChannel::new();
    thread::scope(|s| {
        s.spawn(|| {
            simple_channel.send(100);
        });

        s.spawn(|| {
            let v = simple_channel.receive();
            println!("A - received - v:{}", v);
            simple_channel.send(v + 1);
        });

        s.spawn(|| {
            let v = simple_channel.receive();
            println!("B - received - v:{}", v);
            simple_channel.send(v + 1);
        });
    });

    let v = simple_channel.receive();
    println!("main - received - v:{}", v);
}
