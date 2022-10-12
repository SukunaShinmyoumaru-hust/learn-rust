fn main() {
    // 定义一个元组,类型是(A,B,C...)
    let tup = (500, 6.4, 1);
    // 解构一个元组,可以认为是模式匹配
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    // 可以使用.
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    //数组,和C++一样.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
}
