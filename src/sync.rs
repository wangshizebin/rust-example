use dashmap::DashMap;
use futures::future::join_all;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::runtime::Runtime;
use tokio::time::{Duration, sleep};

// pub async fn op_async() {
//     let task = tokio::spawn(async {
//         for i in 0..5 {
//             sleep(Duration::from_secs(1)).await;
//             println!("hello {}", i);
//         }
//     });
//     task.await.unwrap();
//     println!("task is over");
// }
// #[tokio::main]
// async fn main() {
//     sync::op_async().await;
// }

static RUNTIME: OnceLock<Runtime> = OnceLock::new();
fn runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("failed to create Tokio Runtime"))
}

pub fn op_async() {
    const COUNT: i32 = 5;
    let dataset: Arc<DashMap<i32, String>> = Arc::new(DashMap::new());
    dataset.insert(100, "end".to_string());

    let mut tasks = Vec::new();
    runtime().block_on(async {
        for i in 0..COUNT {
            let dataset_clone = Arc::clone(&dataset);
            let task = tokio::spawn(async move {
                sleep(Duration::from_secs(1)).await;
                let v = format!("hello {}", i,);
                println!("{}", v);
                dataset_clone.insert(i, v);
            });
            tasks.push(task);
        }
        join_all(tasks).await;
    });

    println!("{:?}", dataset);
}

pub fn op_mutex() {
    use std::sync::{Arc, Mutex /*RwLock*/};
    use std::thread;
    //let m = Arc::new(RwLock::new(String::from("start")));
    let s = Arc::new(Mutex::new(String::from("start")));
    println!("Initial Value: {:?}", s.lock());

    let s_clone = Arc::clone(&s);
    let task = thread::spawn(move || {
        println!("thread running...");
        match s_clone.lock() {
            Ok(mut v) => {
                *v = String::from("end");
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    });
    let _ = task.join();
    println!("Final Value: {:?}", s.lock());
}
