fn main() {
    //声明一个空数组
    
    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可直接用于字符串字面量：
    let s = "initial contents".to_string();

    //from也可以
    let s = String::from("initial contents");

    //由于String是UTF-8的,所以说:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //下面我们看看怎么更改字符串!
    //一个是push_str,这个可以push一整个String
    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = "bar";
    println!("s is {}",s);
    s.push_str(s2);
    //执行完push_str之后,s2竟然还能访问,因为s2是&str/
    println!("s is {} s2 is {}",s,s2);

    //String改变
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    // 因为执行加法操作的时候,调用了fn add(self, s: &str) -> String {
    // s2是&String,但是在执行的时候被强制转化成&str.
    // 发现签名中 add 获取了 self 的所有权，因为 self 没有 使用 &。
    // 这意味着示例中的 s1 的所有权将被移动到 add 调用中
    // 这个语句会获取 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权。

    //可以使用format来规范,这种比较好的点在于不会获取任何数据的所有权
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let hello = "Здравствуйте";

    // &str可以定义为可变的,但是不能对其进行更改.
    // let mut s = &hello[0..4];

    // s.push_str("bar");

    //下面两种方式可以用来遍历字符串
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
