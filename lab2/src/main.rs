//使用std::io库
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    //简单的输出 加!是宏而不加!是函数
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop{

        println!("Please input your guess.");
        //定义一个变量,其中mut代表变量可变,不带mut变量不可变
        let mut guess = String::new(); //一个新的String
        //输入其中.read_line表示把数据输入到哪里
        io::stdin()
            .read_line(&mut guess) // 注意这里引用的时候也要添加&
            .expect("Failed to read line");// 处理异常

        //遮蔽 标识量的冒号后面可代表这是什么类型的变量
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);//使用占位符号输出

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}