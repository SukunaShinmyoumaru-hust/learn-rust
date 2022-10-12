//结构体的定义是不允许有可变和不可变的,不能指定结构体的其中之一个成员是可变或者不可变
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//结构体定义有很多种,比如说
struct QuitMessage; // 类单元结构体,没有任何一个元素
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
//当然,你可以构造一个函数,函数生成一个User结构体,返回值传到调用者,所有权也传到调用者
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // 这是一种定义一个结构体变量的方式
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 当然你也可以构造一个可变的变量,用点号修改即可.
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
    //当然,也可以使用结构体的.语法来构建结构体
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("another@example.com"),
        ..user3
    };
}