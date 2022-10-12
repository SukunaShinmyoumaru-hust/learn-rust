//使用enum

enum IpAddrKind {
    V4,
    V6,
}

//现在我们定义了一个新的变量类型,就叫做IpAddrKind

//我们可以进行变量的声明:
//let four = IpAddrKind::V4;

//或者函数的声明
//fn function(ip_type : IpAddrKind){}

//并且用任何一个成员调用函数
//route(IpAddrKind::V4);

//还可以定义结构体
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//更简便的一个方式就是,枚举的时候可以把一组变量与枚举相匹配.
//匹配的话可以匹配元组或者是结构体
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    // XXX (Some kind 结构体)
}

//当然对于枚举来说,一个枚举类型也可以有impl块
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    //match一个表达式
    match coin {
        Coin::Penny => 1, //一个逗号是一个匹配,格式是 匹配项 => 如果匹配成功返回什么
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        //匹配不一定是一个结果,也可以是一个表达式,和[1,2,3]和1::[2,3]匹配 是一样的,匹配出的表达式
        //可以自动完成隐式转换

    }
}

fn main(){
    //了解了枚举之后我们要了解一下Option枚举
    //Option枚举可以帮助程序员去定义空和非空,这是一个对立的存在,程序员定义空和非空需要借助Option枚举
    //其中<T>很像Cpp的泛型,或许就是Cpp的泛型
    //enum Option<T> {
    //  Some(T),
    //  None,
    //}

    let some_number = Some(5);
    let some_string = Some("a string");

    //注意在定义None的时候一定要说明类型
    let absent_number: Option<i32> = None;

    //当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。
    //当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。
    //那么，Option<T> 为什么就比空值要好呢？
    //因为 Option<T> 和 T（这里 T 可以是任何类型）是不同的类型.
    //这样可以有效避免8+Null的出现.对于Null的加可以转化成编译错误

    //如何比较好地使用枚举类型呢?我们可以使用match
    // match 一个表达式 然后匹配表达式的值,或者是match中有一项是能和表达式匹配的.
    //比如说SML中 [1,2,3]和1::[2,3]匹配,这也是允许的

    //现在看上面的函数

    value_in_cents(Coin::Quarter(UsState::Alaska));

    //Option<T>也是可以匹配的
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //对于匹配,我们需要做到穷尽,就是所有的可能都要考虑.
    //所以说要加other这种,有点像Cpp的default

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other);
        // 这样也可以_ => xxxx();
        // _是通配符
    }

    //当然,我们还可以使用if let来执行相关操作
    //有点像if和switch的区别
    
}