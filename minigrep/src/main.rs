use std::env;
use std::fs;
use std::process;

use minigrep::Config;
// 可以为了工程,把所有结构体、函数、impl块定义在lib.rs中
// 结构体封装,暴露外层接口都是很有效的方法.
fn main() {
    // first step:获取参数
    let args : Vec<String> = env::args().collect();
    // unwrap_or_else，它定义于标准库的 Result<T, E> 上。
    // 使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理。
    // 当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。
    // 然而，当其值是 Err 时，该方法会调用一个 闭包（closure）
    // 也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。
    // 现在你需要理解的是 unwrap_or_else 会将 Err 的内部值
    // 也就是示例 增加的 not enough arguments 静态字符串的情况
    // 传递给闭包中位于两道竖线间的参数 err。
    // 闭包中的代码在其运行时可以使用这个 err 值。
    let config = minigrep::parse_config(&args).unwrap_or_else(|err|{
        println!("Problem parsing argument: {}",err);
        process::exit(1);
    });

    // second step:打开文件
    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);

        process::exit(1);
    }
    
}
