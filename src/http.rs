use serde::Serialize;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();
fn runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("failed to create Tokio Runtime"))
}

// get接口
pub fn op_get() {
    runtime().block_on(async {
        let url = "http://localhost:3000/";
        println!("Get: {}", url);

        //let client = reqwest::Client::new();
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("请求超时");

        match client.get(url).send().await {
            Ok(response) => {
                let status = response.status();
                match response.text().await {
                    Ok(data) => {
                        if status.is_success() {
                            println!("HTTP 响应内容: {}", data);
                        } else {
                            println!("HTTP 响应错误: {}", status);
                        }
                    }
                    Err(err) => println!("HTTP 读取响应失败: {}", err),
                }
            }
            Err(err) => println!("HTTP 请求失败: {}", err),
        }
    });
}

// post接口
#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

pub fn op_post() {
    runtime().block_on(async {
        let url = "http://localhost:3000/create_dog";
        println!("Post: {}", url);

        let user = User {
            name: "Tom".to_string(),
            age: 20,
        };

        //let client = reqwest::Client::new();
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("请求超时");

        match client.post(url).json(&user).send().await {
            Ok(response) => {
                let status = response.status();

                let header = response.headers();
                let content_type = match header.get("content-type") {
                    Some(hv) => hv.to_str().unwrap_or_default(),
                    None => "",
                };
                println!("content-type:{:?}{:?}", header, content_type);

                match response.text().await {
                    Ok(data) => {
                        if status.is_success() {
                            println!("HTTP 响应内容: {}", data);
                        } else {
                            println!("HTTP 响应错误: {}", status);
                        }
                    }
                    Err(err) => println!("HTTP 读取响应失败: {}", err),
                }
            }
            Err(err) => println!("HTTP 请求失败: {}", err),
        }
    });
}

pub fn op_post_json_object() {
    runtime().block_on(async {
        let url = "http://localhost:3000/create_dog";
        println!("Post: {}", url);

        let user = serde_json::json!({
            "name": "Tom",
            "age": 20,
        });

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("请求超时");

        match client.post(url).json(&user).send().await {
            Ok(response) => {
                let status = response.status();
                match response.text().await {
                    Ok(data) => {
                        if status.is_success() {
                            println!("HTTP 响应内容: {}", data);
                        } else {
                            println!("HTTP 响应错误: {}", status);
                        }
                    }
                    Err(err) => println!("HTTP 读取响应失败: {}", err),
                }
            }
            Err(err) => println!("HTTP 请求失败: {}", err),
        }
    });
}

pub fn op_post_json_string() {
    runtime().block_on(async {
        let url = "http://localhost:3000/create_dog";
        println!("Post: {}", url);

        let user = r#"{"name":"Tom","age":20}"#;
        //let user = "{\"name\":\"Tom\",\"age\":20}";

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("请求超时");

        match client
            .post(url)
            .header("Content-Type", "application/json")
            .body(user)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                match response.text().await {
                    Ok(data) => {
                        if status.is_success() {
                            println!("HTTP 响应内容: {}", data);
                        } else {
                            println!("HTTP 响应错误: {}", status);
                        }
                    }
                    Err(err) => println!("HTTP 读取响应失败: {}", err),
                }
            }
            Err(err) => println!("HTTP 请求失败: {}", err),
        }
    });
}
