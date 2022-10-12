fn main() {
    // if-else语句 这个和C++一样(不用括号) 但是不能用 if number来判断非0
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 也可以像lab2里面一样使用loop-break
    //如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。
    //你可以选择在一个循环上指定一个循环标签（loop label）
    //然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。下面是一个包含两个嵌套循环的示例：
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; //退出内层循环
            }
            if count == 2 {
                break 'counting_up; //退出外层的循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    //当然可以从循环返回,在break之后跟上一个值就是loop块的返回值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    //也可以使用while语句
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //也可以使用for语句遍历迭代器,其中(A..B)和py的功能很像
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
