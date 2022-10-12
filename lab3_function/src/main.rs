fn another_function() {
    println!("Another function.");
}
fn another_function1(x: i32) {
    println!("The value of x is: {}", x);
}
fn five() -> i32 {
    5
}
fn main() {
    // 无参函数,就像C语言一样调
    another_function();
    
    // 有形参的函数,也和C语言一样
    another_function1(5);
    
    // 表达式是有返回值的,大括号括起来的表达式也有返回值
    // 在RUST中,表达式一般都具有返回值
    // let语句除外,这个是没有返回值的,let x = ?这个语句一般没有返回值.

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // 有返回值的函数->后面接返回值的类型
    let x = five();
    
    println!("The value of x is: {}", x);
}

