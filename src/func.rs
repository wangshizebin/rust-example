pub fn op() {
    //closures
    let out = 9i32;
    let closure_fn = |i: i32| {
        println!("closure_fn: inner");
        i + out
    };
    let x = closure_fn(100);
    println!("{:?}", x);

    let one = || 1i32;
    println!("{}", one());

    //closures move
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    //println!("vec 中有 {} 个元素", haystack.len());
    println!("{}", contains(&1));
    println!("{}", contains(&4));
}
