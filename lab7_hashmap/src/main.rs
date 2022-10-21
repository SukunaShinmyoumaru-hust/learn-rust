fn main() {
    //HashMap插入方法
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    //通过insert构建一个HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let s : String = String::from("Green");
    // s无效了
    scores.insert(s, 100);
    // 下面这一行编译不通过
    // println!("s is {}",s);

    //当然,还可以使用Vector的Collect方法构建hashMap
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // 这里 HashMap<_, _> 类型标注是必要的，因为 collect 有可能当成多种不同的数据结构
    // 而除非显式指定否则 Rust 无从得知你需要的类型。

    // 可以使用get方法获得Option<T>类型的数据
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score{
        None => {
            println!("No");
        },
        Some(i) => {
            println!("score is {}",i);
        }
    }
    // 因为score是Option<T>类型,不行
    // println!("score is {}",score);

    // 当然也可以使用for来遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 数据怎么更新呢?
    // 第一种方法是 插入两遍,这是覆盖
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // Blue is 25

    // 还可以用or_insert这个方法在空缺的时候进行插入

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // Blue不是空缺的,所以说就不会覆盖

    println!("{:?}", scores);

    // 还可以边查询边更新.or_insert函数会返回HashMap一个key-value对的引用.
    // 可以通过这个引用进行更新,如下:

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
