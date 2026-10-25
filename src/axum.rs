// 1. 外部依赖
use axum::{
    Router,
    extract::Query,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

// 2. 数据结构
#[derive(Debug, Deserialize)]
struct CatQuery {
    id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Dog {
    name: String,
    age: u32,
}

// 3. 请求处理函数
async fn hello() -> String {
    String::from("hello world")
}

async fn dog() -> String {
    String::from("dog")
}

async fn cat(Query(params): Query<CatQuery>) -> String {
    let id = params.id.unwrap_or_default();
    let id = id.trim();

    if id.is_empty() {
        return String::from("no param");
    }

    format!("id = {}", id)
}

async fn create_dog(body: String) -> String {
    let dog: Dog = serde_json::from_str(&body).unwrap_or_default();
    let name = dog.name.trim();

    if name.is_empty() {
        return String::from("no data");
    }

    format!("Created dog: {}, age: {}", dog.name, dog.age)
}

// 4. 程序入口
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/dog", get(dog))
        .route("/create_dog", post(create_dog))
        .route("/cat", get(cat));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
