use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
struct Dog {
    name: String,
    age: u32,
}

pub fn op() {
    let body_str = "{
        \"name\":\"wangzebin\",
        \"age\":20
        }";
    let body: String = String::from(body_str);
    let dog: Dog = serde_json::from_str(&body).unwrap_or_default();
    println!(
        "{:?}{:?}{:?}\n",
        body,
        std::any::type_name_of_val(&body),
        dog
    );

    let body: &[u8] = body_str.as_bytes();
    let dog: Dog = serde_json::from_slice(&body).unwrap_or_default();
    println!(
        "{:?}{:?}{:?}\n",
        body,
        std::any::type_name_of_val(&body),
        dog
    );

    let body: Vec<u8> = body_str.as_bytes().to_vec();
    let dog: Dog = serde_json::from_slice(&body).unwrap_or_default();
    println!(
        "{:?}{:?}{:?}\n",
        body,
        std::any::type_name_of_val(&body),
        dog
    );

    let result = serde_json::to_string(&dog).unwrap_or_default();
    println!(
        "result:{:?}{:?}\n",
        result,
        std::any::type_name_of_val(&result)
    );

    let result: Vec<u8> = serde_json::to_vec(&dog).unwrap_or_default();
    println!(
        "result:{:?}{:?}\n",
        result,
        std::any::type_name_of_val(&result)
    );

    let result: &[u8] = &serde_json::to_vec(&dog).unwrap_or_default();
    println!(
        "result:{:?}{:?}\n",
        result,
        std::any::type_name_of_val(&result)
    );

    use std::convert::TryInto;
    let result: &[u8] = &[1, 2, 3, 4];
    let x: [u8; 4] = result.try_into().unwrap();
    println!("x:{:?}{:?}\n", x, std::any::type_name_of_val(&x));
    let x: [u8; _] = [1, 2, 3, 4];
    println!("x:{:?}{:?}\n", x, std::any::type_name_of_val(&x));
}
