use std::env;
use std::fmt;

fn main() {
    // 结构体显示样式
    //#[derive(Debug)]
    struct Ex<'a> {
        // 如果字段使用了借用，必须指定生命周期
        // 结构体定义中要指明Ex<'a>，标明结构体生命周期在'a之内，受其约束
        id: &'a str,
    }
    impl fmt::Debug for Ex<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("struct Ex debug")
                .field("field id", &self.id)
                .finish()
        }
    }
    impl fmt::Display for Ex<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("struct Ex display")
                .field("field id", &self.id)
                .finish()
        }
    }
    let x = Ex { id: "100" };

    println!("{}\n{:?}", x, x);

    // 获得运行的二进制程序所在路径
    let path = env::current_exe().unwrap();
    println!("{:?}", path.to_str().unwrap_or_default());

    let args: Vec<String> = env::args().collect();

    // 第一个参数: 程序路径
    println!("我的路径是 {}。", args[0]);

    // 其余参数: 命令行参数。
    // 例如：
    // $ ./args arg1 arg2
    // $ cargo run -- hello1 hello2
    println!("我获得了 {:?} 个参数：{:?}。", args.len() - 1, &args[1..]);
    println!("ok");

    let x = env::current_dir().unwrap_or_default();
    println!("{:?}", x);

    match env::var("HOME") {
        Ok(value) => println!("HOME = {}", value),
        Err(err) => println!("Error: {}", err),
    }

    // 目录分隔符 path separator
    use std::path::MAIN_SEPARATOR;
    println!("{}", MAIN_SEPARATOR);

    use std::path::PathBuf;
    let path = PathBuf::from("target").join("debug").join("bar");
    println!("{}", path.to_str().unwrap_or_default());

    // 命令行 command
    use std::process::Command;
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("执行进程失败：{}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("执行成功，输出为：{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("执行失败，错误为：{}", s);
    }
}
