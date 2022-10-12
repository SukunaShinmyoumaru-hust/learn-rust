fn main() {
    //定义一个新的vector.
    let v: Vec<i32> = Vec::new();

    //使用宏初始化一个vector
    let v = vec![1, 2, 3];

    //可以使用push插入元素
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    //有两种方法访问Vector
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //由于引用的规则,不能创建一个可变引用,一个不可变引用:
    //一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则来确保 vector 内容的这个引用和任何其他引用保持有效。
    /*
    
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        v.push(6);
    */

    //对于vector,我们还可以使用for ... in ...遍历之
    let v = vec![100, 32, 57];
    // in &v表示i不可变,in mut &v表示i可变
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    //除了 push 之外还有一个 pop 方法，它会移除并返回 vector 的最后一个元素。
}
