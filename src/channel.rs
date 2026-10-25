use futures::future::join_all;
use std::time::Duration;

use tokio::{
    sync::{mpsc, watch},
    task::JoinHandle,
    time::sleep,
};

#[tokio::main]
async fn main() {
    let (tx_exit, rx_exit) = watch::channel(false);
    let (tx_msg, mut rx_msg) = mpsc::channel(16);

    let mut producers = Vec::<JoinHandle<()>>::new();

    // 第1个发送进程
    let mut rx_exit_clone = rx_exit.clone();
    let tx_msg_clone = tx_msg.clone();
    let producer = tokio::spawn(async move {
        let mut val = 0;
        loop {
            tokio::select! {
                res = rx_exit_clone.changed() => {
                    if res.is_err() {
                        println!("task {} watch channel closed, exit, error: {:?}", 0, res);
                        break;
                    } else if *rx_exit_clone.borrow() {
                        println!("task {} receive exit signal, quit gracefully", 0);
                        break;
                    }
                }

                _ = async {
                    sleep(Duration::from_secs(2)).await;
                    if let Err(err) = tx_msg_clone.send(val).await {
                        println!("an error occurred: {:?}", err);
                        return;
                    }
                    println!("sent in queue 0: {}", val);
                    val += 1;
                } => {}
            }
        }
    });
    producers.push(producer);

    // 第2个发送进程
    let mut rx_exit_clone = rx_exit.clone();
    let tx_msg_clone = tx_msg.clone();
    let producer = tokio::spawn(async move {
        let mut val = 0;
        loop {
            tokio::select! {
                res = rx_exit_clone.changed() => {
                    if res.is_err() {
                        println!("task {} watch channel closed, exit, error: {:?}", 1, res);
                        break;
                    } else if *rx_exit_clone.borrow() {
                        println!("task {} receive exit signal, quit gracefully", 1);
                        break;
                    }
                }

                _ = async {
                    sleep(Duration::from_secs(3)).await;
                    if let Err(err) = tx_msg_clone.send(val).await {
                        println!("an error occurred: {:?}", err);
                        return;
                    }
                    println!("sent in queue 1: {}", val);
                    val += 1;
                } => {}
            }
        }
    });
    producers.push(producer);

    // 发送退出信号
    // 必须 tx_exit.clone()，否则 tx_exit 所有权被移入线程，一旦结束会导致 rx_exit 出错
    let tx_exit_clone = tx_exit.clone();
    let terminator = tokio::spawn(async move {
        sleep(Duration::from_secs(10)).await;
        tx_exit_clone.send(true).unwrap_or_default();
    });
    producers.push(terminator);

    // rx_msg 所有权被移入线程。
    // 一旦 tx_msg 被 drop，就会导致 rx_msg.recv() 收到 None
    let consumer = tokio::spawn(async move {
        loop {
            if let Some(v) = rx_msg.recv().await {
                println!("received: {}", v);
            } else {
                println!("consumer is over");
                break;
            }
        }
    });

    join_all(producers).await;
    drop(tx_msg);
    consumer.await.unwrap_or_default();
}
