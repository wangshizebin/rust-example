use tokio::sync::watch;
use tokio::task::JoinSet;
use tokio::time::{Duration, Instant, sleep};

#[tokio::main]
async fn main() {
    let (tx, rx) = watch::channel(false);
    let mut tasks = JoinSet::new();

    for i in 0..10 {
        let mut rx_exit = rx.clone();
        tasks.spawn(async move {
            loop {
                tokio::select! {
                    _ = sleep(Duration::from_secs(1)) => {
                        println!("task {} | time: {:?}", i, Instant::now());
                    }

                    res = rx_exit.changed() => {
                        if res.is_err() {
                            println!("task {} watch channel closed, exit", i);
                            break;
                        }
                        if *rx_exit.borrow() {
                            println!("task {} receive exit signal, quit gracefully", i);
                            break;
                        }
                    }

                }
            }
        });
    }

    let tx_exit = tx.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(10)).await;
        if let Err(_) = tx_exit.send(true) {
            println!("warning: no receivers alive, cannot send exit signal");
        }
    });

    // 等待全部子任务完成
    let mut i = 0;
    while let Some(res) = tasks.join_next().await {
        i += 1;
        match res {
            Ok(_) => println!("{i} 子任务正常结束"),
            Err(e) => eprintln!("子任务异常：{}", e),
        }
    }
    println!("all tasks exit, main done");
}
