#![feature(await_macro, async_await, futures_api)]

/*
[dependencies]

futures = "0.1.25"
tokio = { version = "0.1.15", features = ["async-await-preview"] }

*/

use tokio::prelude::*;

use futures::sync::mpsc::unbounded;
use futures::sync::mpsc::UnboundedSender as Sender;
use futures::sync::mpsc::UnboundedReceiver as Receiver;


use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

type T = u8;

async fn f1() {
    let (_, mut rx): (Sender<T>, Receiver<T>) = unbounded();
    let (_, rx_in): (Sender<(T, Sender<T>)>, Receiver<(T, Sender<T>)>) = unbounded();

    let resps: Arc<Mutex<HashMap<T, Sender<T>>>> = Arc::new(Mutex::new(HashMap::new()));

    let resp_one = resps.clone();

    tokio::spawn_async(async move {
        while let Some(Ok(id)) = await!(rx.next()) {
            let cond = {
                let resp_one = resp_one.lock().unwrap();
                resp_one.contains_key(&id)
            };
            if cond {
                let mut tx = {
                    let mut resp_one = resp_one.lock().unwrap();
                    resp_one.get(&id).unwrap().clone()
                };
                await!(tx.send_async(id));
            }
        };
    });
}

fn main() {
    println!("Hello, world!");
    tokio::run_async(async {
        await!(f1());
    });
}
