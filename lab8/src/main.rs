// 这里使用 trait 关键字来声明一个 trait，后面是 trait 的名字，在这个例子中是 Summary。
// 在大括号中声明描述实现这个 trait 的类型所需要的行为的方法签名，
// 在这个例子中是 fn summarize(&self) -> String。
// 这个很像接口,实现了接口的方法就是接口的类型,有一个结构体实现了Summary的接口,这个结构体也是Summary类型
pub trait Summary {
    // 简单实现
    // fn summarize(&self) -> String;
    // 可以定义默认实现
    // 如果定义多个函数实现Summary要全部实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
// 外面的crate想要使用这个trait,要使用use操作将这个trait引入到当前
// 将trait定义成pub也是为了外面的模块能引入Summary这个trait,将其作为公有
// 外界的模块要使用use crate::Summary来引入这个公有块.
// 对于一个类,可以收拥有impl trait名 for 类名来实现trait

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 空块,就是为了使用默认实现
impl Summary for NewsArticle {
    
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 实现 trait 时需要注意的一个限制是
// 只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。

// 可以使用 trait 来接受多种不同类型的参数。
// 这个item: impl Summary代指已经实现了Summary的
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 当然还可以使用trait bound方式,他需要规定一个泛型参数
// T : E 代表T这个类型必须实现E这个trait
// T : E + F代表必须实现E和F
// 例如:pub fn notify<T: Summary>(item1: T, item2: T)
// 例如:pub fn notify<T: Summary + Display>(item: T)
// 当然还可以是用where操作:
/*
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Display
{
    println!("Hello {} {}" ,t ,u);
}
*/
// 也可以在返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型：

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 不过这只适用于返回单一类型的情况。例如，这段代码的返回值类型指定为返回 impl Summary，但是返回了 NewsArticle 或 Tweet 就行不通：
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/
// 泛型与函数的配合,要求传入的类型满足PartialOrd 和 Copy
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//我们知道impl块也是有泛型参数的,这个约束也可以添加在impl块中
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    
    println!("New article available! {}", article.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
