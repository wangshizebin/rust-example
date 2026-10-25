use std::collections::hash_map;

pub fn op_string() {
    let mut s = String::from("this is the content");
    let t = s.split(' ');
    for v in t {
        println!("{:?}{:?}", std::any::type_name_of_val(&v), v);
    }
    println!("{:?}{:?}", std::any::type_name_of_val(&s), s);

    match s.find("is") {
        Some(p) => {
            s = String::from(&s[p..]);
            println!("String left:{s}");
        }
        None => {
            println!("not found");
            return;
        }
    }

    s = s.replace("is", "");
    s.push_str("\r\n\t");
    s = String::from(s.trim());
    println!("{s}{}", s.len());

    s.insert(0, ' ');
    s.insert_str(0, "is");
    println!("{s}{}", s.len());
    println!("starts_with:{}", s.starts_with("is"));
    println!("ends_with:{}", s.ends_with("content"));

    let s = "word ".repeat(3);
    println!("{:?}", s);
    match "123".parse::<i32>() {
        Ok(n) => {
            println!("parsed number:{:?}", n);
        }
        Err(e) => {
            println!("parsed error:{:?}", e);
        }
    }

    let x = s.as_bytes();
    println!("{:?}{:?}", std::any::type_name_of_val(&x), x);
    let s = format!("=={}==", "base");
    println!("{:?}{:?}", std::any::type_name_of_val(&s), s);
    println!("");
}

pub fn op_hashmap() {
    let mut x = hash_map::HashMap::<i32, String>::new();
    x.insert(0, "c".to_string());
    x.insert(1, "b".to_string());
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));

    let empty = "".to_string();
    let v = x.get(&0).unwrap_or(&empty);
    println!("{:?}{:?}", v, std::any::type_name_of_val(&v));

    x.insert(0, "a".to_string());
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));

    x.remove(&0);
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));
    println!("len: {:?}", x.len());

    let v = &x.entry(100).or_default();
    println!("{:?}{:?}", v, std::any::type_name_of_val(&v));

    x.clear();
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));
    println!("len: {:?}", x.len());

    x.insert(0, "c".to_string());
    x.insert(1, "b".to_string());
    for (k, v) in x.iter() {
        println!("{:?}-{:?}", k, v);
    }
    for (_, v) in x.iter_mut() {
        *v = "x".to_string();
    }
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));
    println!("");
}

pub fn op_vec() {
    let mut x = vec!["1", "2", "3"];
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));
    for v in x.iter() {
        println!("{:?}{:?}", v, std::any::type_name_of_val(&v));
    }
    for (i, v) in x.iter().enumerate() {
        println!("{:?}{:?}", i, v);
    }

    for v in x.iter_mut() {
        *v = "x";
    }
    x[0] = "1";
    x[1] = "2";
    x[2] = "3";
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));
    println!("len: {:?}", x.len());

    let mut x = x.repeat(2);
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));
    for _i in 0..10 {
        x.pop();
    }
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));

    x.push("1");
    x.push("2");
    x.push("3");

    let x = x[..2].to_vec();
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));

    let mut x = Vec::<i32>::new();
    x.push(100);
    println!("{:?}{:?}", x, std::any::type_name_of_val(&x));
}

pub fn op_trans() {
    let x: u8 = 0x31;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));
    let x = x as u32;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));
    let x = x as u8;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));
    let x = x as char;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));
    let x = String::from(x);
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));

    let x: f32 = 12.5;
    let x = x as i32;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));
    let x: f32 = 12.5;
    let x = x.round() as i32;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));
    let x = x as f32;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));
    let x = std::mem::size_of_val(&x) as i64;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));

    type I32 = i32;
    let x: I32 = 100;
    println!("{:?} {:?}", x, std::any::type_name_of_val(&x));
}

pub fn op_from_into() {
    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    let num = Number::from(30);
    println!("Number: {:?}", num.value);
    let num: Number = 30.into();
    println!("Number {:?}", num.value);

    match "198".parse::<i32>() {
        Ok(v) => {
            println!("{:?}", v);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    if let Ok(x) = "198x".parse::<i32>() {
        println!("{:?}", x);
    } else {
        println!("err");
    }

    let result = "198x".parse::<i32>();
    if let Ok(x) = result {
        println!("{:?}", x);
    } else if let Err(e) = result {
        println!("{:?}", e);
    }
}
