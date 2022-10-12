### rust学习笔记

#### lab1 和 lab2

​	lab1和lab2主要了解一些RUST的基本语法.

​	lab1打印一个Hello World,而lab2完成了一个猜数的游戏.

- RUST程序有一个主函数,程序从主函数开始执行

- RUST可以使用println打印一行信息,其中{}表示占位,这个类似于C语言的`%*`/

- RUST可以在前面添加包,注意添加包的时候要在toml文件中添加依赖

- cargo的用法

  - `cargo new`建立一个新的cargo项目.
  - `cargo run`编译加运行
  - `cargo build`编译

- 使用`io.stdin`包获取一个数据,如代码所示:其中read代表存放的位置,注意引用的时候要加mut

  ```RUST
  io::stdin()
              .read_line(&mut guess)
              .expect("Failed to read line");
  ```

- 可以使用匹配的方法,这一点和SML很像.

  - 表达式带match表示这是一个匹配的表达式,这个表达式先计算前面的获得一个返回值.
  - 这个返回值会和下面的几个选项进行匹配.匹配的话就执行或者返回=>后面的语句.

  ```RUST
  match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
  };
  ```

- 我们还可以执行loop语句来执行循环,continue和break的用法和C语言一致.

#### lab3

##### lab3_1 variables

- 可以使用`let 名字 : 类型 = 初始值`来定义一个新的变量.其中let后不加mut表示const.
- 变量允许覆盖,就是多次声明,多次声明的话取最后一次.变量和C语言一样有作用域

##### lab3_2 datatype

- 整数类型
  ![image-20220915154000286](../../Library/Application Support/typora-user-images/image-20220915154000286.png)

- 整数字面量
  ![image-20220915154031268](../../Library/Application Support/typora-user-images/image-20220915154031268.png)
- 浮点类型f32(单精度)和f64(双精度)
- 布尔类型字面量:true和false
- 字符类型,用单引号扩起来
- 复合类型请看代码

##### lab3_3 function

定义函数的参数和返回值

##### lab3_4 control

可以使用if-else,也可以使用while

#### Chapter1 所有权

首先，让我们看一下所有权的规则。当我们通过举例说明时，请谨记这些规则：

- Rust 中的每一个值都有一个被称为其 **所有者**（*owner*）的变量。
- 值在任一时刻有且只有一个所有者。
- 当所有者（变量）离开作用域，这个值将被丢弃。

RUST语言也有作用域，我们看看一些变量的 **作用域**（*scope*）。作用域是一个项（item）在程序中有效的范围。也就是说一个变量在哪些区域有效

RUST和垃圾回收语言的相同之处在于申请一块内存是程序员的责任,而RUST和垃圾回收语言的的不同就是内存在拥有它的变量离开作用域后就被自动释放。下面是示例 中作用域例子的一个使用 `String` 而不是字符串字面量的版本：

```rust
    {
        let s = String::from("hello"); // 从此处起，s 开始有效
        // 使用 s
    }                                  // 此作用域已结束，
                                       // s 不再有效
```

这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。

为了确保内存安全，这种场景下 Rust 的处理有另一个细节值得注意。在 `let s2 = s1` 之后，Rust 认为 `s1` 不再有效，因此 Rust 不需要在 `s1` 离开作用域后清理任何东西。看看在 `s2` 被创建之后尝试使用 `s1`会发生什么；这段代码不能运行：

```RUST
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```

如果你在其他语言中听说过术语 **浅拷贝**（*shallow copy*）和 **深拷贝**（*deep copy*），那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。不过因为 Rust 同时使第一个变量无效了，这个操作被称为 **移动**（*move*），而不是浅拷贝。上面的例子可以解读为 `s1` 被 **移动** 到了 `s2` 中。那么具体发生了什么，如图所示。

![image-20220915164756537](../../Library/Application Support/typora-user-images/image-20220915164756537.png)

我们可以认为在s2 = s1之后,只有s2是有效的,s1这个时候可以认为是一个亡值

如果我们 **确实** 需要深度复制 `String` 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 `clone`的通用函数。后面会讨论方法语法，不过因为方法在很多语言中是一个常见功能，所以之前你可能已经见过了。

这是一个实际使用 `clone` 方法的例子：

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

这段代码能正常运行，并且明确产生图中行为，这里堆上的数据 确实 被复制了。当出现 clone 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源。你很容易察觉到一些不寻常的事情正在发生。

![image-20220915165132045](../../Library/Application Support/typora-user-images/image-20220915165132045.png)

这里还有一个没有提到的小窍门。这些代码使用了整型并且是有效的，他们是示例 4-2 中的一部分：

```rust
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```

但这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 `clone`，不过 `x` 依然有效且没有被移动到 `y`中。

原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 `y` 后使 `x` 无效。换句话说，这里没有深浅拷贝的区别，所以这里调用 `clone` 并不会与通常的浅拷贝有什么不同，我们可以不用管它。

Rust 有一个叫做 `Copy` trait 的特殊标注，可以用在类似整型这样的存储在栈上的类型上（第 10 章详细讲解 trait）。如果一个类型实现了 `Copy` trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。Rust 不允许自身或其任何部分实现了 `Drop` trait 的类型使用 `Copy` trait。如果我们对其值离开作用域时需要特殊处理的类型使用 `Copy` 标注，将会出现一个编译时错误。

那么哪些类型实现了 `Copy` trait 呢？你可以查看给定类型的文档来确认，不过作为一个通用的规则，任何一组简单标量值的组合都可以实现 `Copy`，任何不需要分配内存或某种形式资源的类型都可以实现 `Copy`。如下是一些 `Copy` 的类型：

- 所有整数类型，比如 `u32`。
- 布尔类型，`bool`，它的值是 `true` 和 `false`。
- 所有浮点数类型，比如 `f64`。
- 字符类型，`char`。
- 元组，当且仅当其包含的类型也都实现 `Copy` 的时候。比如，`(i32, i32)` 实现了 `Copy`，但 `(i32, String)` 就没有。

将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样。示例使用注释展示变量何时进入和离开作用域：

```rust
fn main() {
  let s = String::from("hello");  // s 进入作用域

  takes_ownership(s);             // s 的值移动到函数里 ...
                                  // ... 所以到这里不再有效

  let x = 5;                      // x 进入作用域

  makes_copy(x);                  // x 应该移动函数里，
                                  // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
  println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
  println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

当尝试在调用 `takes_ownership` 后使用 `s` 时，Rust 会抛出一个编译时错误。这些静态检查使我们免于犯错。试试在 `main` 函数中添加使用 `s` 和 `x` 的代码来看看哪里能使用他们，以及所有权规则会在哪里阻止我们这么做。

返回值也可以转移所有权。

```rust
fn main() {
  let s1 = gives_ownership();         // gives_ownership 将返回值
                                      // 移给 s1

  let s2 = String::from("hello");     // s2 进入作用域

  let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                      // takes_and_gives_back 中,
                                      // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {           // gives_ownership 将返回值移动给
                                           // 调用它的函数

  let some_string = String::from("yours"); // some_string 进入作用域

  some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

  a_string  // 返回 a_string 并移出给调用的函数
}
```

在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据。

我们就可以使用RUST里面的引用来解决这些问题

下面是如何定义并使用一个（新的）`calculate_length` 函数，它以一个对象的引用作为参数而不是获取值的所有权：

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

这些 & 符号就是 **引用**，它们允许你使用值但不获取其所有权。如果没有&符号,那么在进入calculate_length函数之后main函数就丧失了s1的所有权

![image-20220915171058820](../../Library/Application Support/typora-user-images/image-20220915171058820.png)

仔细看看这个函数调用：

```rust
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
```

`&s1` 语法让我们创建一个 **指向** 值 `s1` 的引用，但是并不拥有它。因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。

同理，函数签名使用 `&` 来表明参数 `s` 的类型是一个引用。让我们增加一些解释性的注释：

```rust
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
}
```

变量 `s` 有效的作用域与函数参数的作用域一样，不过当引用停止使用时并不丢弃它指向的数据，因为我们没有所有权。当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。

我们将创建一个引用的行为称为 **借用**（*borrowing*）。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。

如果我们尝试修改借用的变量呢？尝试示例中的代码。

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。

但是我们可以创建可变引用.通过一个小调整就能修复示例代码中的错误：(注意:可变引用本质上引用,也不会放弃所有权)

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

首先，我们必须将 `s` 改为 `mut`。然后必须在调用 `change` 函数的地方创建一个可变引用 `&mut s`，并更新函数签名以接受一个可变引用 `some_string: &mut String`。这就非常清楚地表明，`change` 函数将改变它所借用的值。

不过可变引用有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败：

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

这样子是错误的

我们不能在同一时间多次将 `s` 作为可变变量借用。第一个可变的借入在 `r1` 中，并且必须持续到在 `println!` 中使用它，但是在那个可变引用的创建和它的使用之间，我们又尝试在 `r2` 中创建另一个可变引用，它借用了与 `r1` 相同的数据。

防止同一时间对同一数据进行多个可变引用的限制允许可变性，不过是以一种受限制的方式允许。新 Rustacean 们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。

这个限制的好处是 Rust 可以在编译时就避免数据竞争。**数据竞争**（*data race*）类似于竞态条件，它可由这三个行为造成：

- 两个或更多指针同时访问同一数据。
- 至少有一个指针被用来写入数据。
- 没有同步数据访问的机制。

数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 **同时** 拥有：

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
```

这样子是没问题的

类似的规则也存在于同时使用可变与不可变引用中。这些代码会导致一个错误：

```rust
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题

    println!("{}, {}, and {}", r1, r2, r3);
```

哇哦！我们 **也** 不能在拥有不可变引用的同时拥有可变引用。不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了！然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。

注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。例如，因为最后一次使用不可变引用（`println!`)，发生在声明可变引用之前，所以如下代码是可以编译的：

```rust
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
```

不可变引用 `r1` 和 `r2` 的作用域在 `println!` 最后一次使用之后结束，这也是创建可变引用 `r3` 的地方。它们的作用域没有重叠，所以代码是可以编译的。编译器在作用域结束之前判断不再使用的引用的能力被称为非词法作用域生命周期（Non-Lexical Lifetimes，简称 NLL）。

在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 **悬垂指针**（*dangling pointer*），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

以C++为例,对于一个char* a = new char(5);如果我们调用free(a),这相当于释放了a指向的那块区域,但是a本身占领的区域没有被释放,这样子我们再去访问a,会产生内存泄露.

让我们尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免：

文件名: src/main.rs

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

让我们仔细看看我们的 `dangle` 代码的每一步到底发生了什么：

```rust
fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
```

因为 `s` 是在 `dangle` 函数内创建的，当 `dangle` 的代码执行完毕后，`s` 将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 `String`，这可不对！Rust 不会允许我们这么做。

让我们概括一下之前对引用的讨论：

- 在任意给定时间，**要么** 只能有一个可变引用(可变引用可以修改引用的元素)，**要么** 只能有多个不可变引用。
- 引用必须总是有效的。引用的元素所有权必须在main函数手里.

**字符串 slice**（*string slice*）是 `String` 中一部分值的引用，它看起来像这样：

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
		let ignore_start = &s[..2] //like &s[0..2]
		let ignore_end = &s[2..] //like &s[2..s.len()]
```

“字符串 slice” 的类型声明写作 `&str`,记住了这些之后你就可以写出这样的代码

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

例如:`let s = "hello world"`这里 `s` 的类型是 `&str`：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面量是不可变的；**`&str` 是一个不可变引用。本质上还是个引用,还是一个不可变的引用**

可以认为&str是字面量,String是一个结构体.

![image-20220921163626316](../../Library/Application Support/typora-user-images/image-20220921163626316.png)

#### lab4 结构体基础

#### lab5 结构体函数以及使用

复习:

- 如何定义一个结构体?定义有几种方式?
- 结构体如何和函数结合在一起?
- 如何定义结构体的方法?

#### lab6 枚举与模式匹配

复习

- 如何定义枚举
- Option枚举
- 使用match进行匹配

#### Ch2 模块系统

##### first part

模块系统的第一部分，我们将介绍包和 crate。

- crate 是一个二进制项或者库。
- *crate root* 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 。你可以理解为这是一个代码文件,Rust编译器会根据这个代码去构建一个新的crate.
- *包*（*package*） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 *Cargo.toml*文件，阐述如何去构建这些 crate。

你可以认为 包里面有很多歌crate

包中所包含的内容由几条规则来确立。

- 一个包中至多 **只能** 包含一个库 crate(library crate)；
- 包中可以包含任意多个二进制 crate(binary crate)；包中至少包含一个 crate，无论是库的还是二进制的。

让我们来看看创建包的时候会发生什么。首先，我们输入命令 `cargo new`：

```text
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

当我们输入了这条命令，Cargo 会给我们的包创建一个 *Cargo.toml* 文件。查看 *Cargo.toml* 的内容，会发现并没有提到 *src/main.rs*，因为 Cargo 遵循的一个约定：*src/main.rs* 就是一个与包同名的二进制 crate 的crate root.就像上文中所说的,编译器会根据crate root产生root.

你可以这么理解,crate root是一个代码文件,通过crate root可以生成一个二进制crate(可以理解成“可执行文件?”),一群crate构成一个包

包里面添加crate的方法:

- 有了一个只包含 *src/main.rs* 的包，意味着它通过构建只含有一个名为 `my-project` 的二进制 crate。
- 有了一个只包含 *src/lib.rs* 的包，意味着它通过构建只含有一个名为 `my-project` 的库 crate。
- 如果一个包同时含有 *src/main.rs* 和 *src/lib.rs*，则它可以通过构建两个 crate：一个库和一个二进制项，且名字都与包相同。
- 通过将文件放在 *src/bin* 目录下，一个包可以拥有多个二进制 crate：每个 *src/bin* 下的文件都会被编译成一个独立的二进制 crate。

一个 crate 会将一个作用域内的相关功能分组到一起，使得该功能可以很方便地在多个项目之间共享。通过引入外部的crate,我们可以方便地使用外部的资源的特性.

如果我们自己写的代码的特性名字和外部资源特性的名字一样,编译器这可以防止潜在的冲突.具体的方法就是加上::符号表示从属关系

##### 第二部分

执行 `cargo new --lib restaurant`，来创建一个新的名为 `restaurant` 的库。

Filename: src/lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}
```

我们用关键字 `mod` 定义一个模块，指定模块的名字（在示例中为 `front_of_house`），并用大括号包围模块的主体。我们可以在模块中包含其他模块，就像本示例中的 `hosting` 和 `serving` 模块。模块中也可以包含其他项，比如结构体、枚举、常量、trait、函数.

通过使用模块，我们可以把相关的定义组织起来，并通过模块命名来解释为什么它们之间有相关性。使用这部分代码的开发者可以更方便的循着这种分组找到自己需要的定义，而不需要通览所有。编写这部分代码的开发者通过分组知道该把新功能放在哪里以便继续让程序保持组织性。

我们提到，*src/main.rs* 和 *src/lib.rs* 被称为 crate 根。如此称呼的原因是，这两个文件中任意一个的内容会构成名为 `crate` 的模块，可以认为这是一个根。

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

构建了许多模块之后,如果我们要调用模块的方法.调用的方法有两个:

- **绝对路径**（*absolute path*）从 crate 根部开始，以 crate 名或者字面量 `crate` 开头。
- **相对路径**（*relative path*）从当前模块开始，以 `self`、`super` 或当前模块的标识符开头。

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

但是现在 `hosting` 模块是私有的。换句话说，我们拥有 `hosting` 模块和 `add_to_waitlist` 函数的的正确路径，但是 Rust 不让我们使用，因为它不能访问私有片段。Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。

我们可以还可以通过使用 `pub` 关键字来创建公共项，使子模块的内部部分暴露给上级模块。

注意我们需要了解一个事实,就是模块定义为pub不代表我们可以访问模块的元素,我们还需要继续定义模块里面的内容为pub.

我们还可以使用 `super` 开头来构建从父模块开始的相对路径。这么做类似于文件系统中以 `..` 开头的语法。我们为什么要这样做呢？

考虑一下代码，它模拟了厨师更正了一个错误订单，并亲自将其提供给客户的情况。`fix_incorrect_order` 函数通过指定的 `super` 起始的 `serve_order` 路径，来调用 `serve_order` 函数：

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

结构体也可以有公有和私有、结构体里面的方法和变量也可以是共有或者是私有的.如果结构体里面有私有的字段,我们需要构建一个公有的关联函数创建实例.

对于枚举,就是相反的,如果设置成公有,那么所有的成员也会变成公有.

##### 第三部分

use到N级,那么所有的相对路径的声明从N级开始.

我们还可以用use语句引用当前的某些包到当前环境中,也就是添加使用相对路径引用包的”环境“,有点像Linux的PATH环境变量,添加了某个路径到PATH中就可以不用指定路径不用cd就可以调用程序了

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

但是注意引用相对路径的时候要提一下当前路径的名字.

一般来说use的时候有这么几个习惯的用法:

- 要想使用 `use` 将函数的父模块引入作用域，我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的。就像上面的例子一样就是引用到上面的那一级.
- 另一方面，使用 `use` 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。

我们还使用 `as` 指定一个新的本地名称或者别名.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

当使用 `use` 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。如果为了让调用你编写的代码的代码能够像在自己的作用域内引用这些类型，可以结合 `pub` 和 `use`。这个技术被称为 “*重导出*（*re-exporting*）”，因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域。

这句话是什么意思呢?就是程序use了一个模块,别的程序也可以用引用你use的东西.

比如说包A.A里面`use std::io`.包B`use A`就不能再访问`std::io`.如果`pub use std::io`.包B`use A`就能再访问`std::io`.

那么怎么引入外部项目呢?

[crates.io](https://crates.io/) 上有很多 Rust 社区成员发布的包，将其引入你自己的项目都需要一道相同的步骤：在 *Cargo.toml* 列出它们并通过 `use` 将其中定义的项引入项目包的作用域中。

注意标准库（`std`）对于你的包来说也是外部 crate。因为标准库随 Rust 语言一同分发，无需修改 *Cargo.toml* 来引入 `std`，不过需要通过 `use` 将标准库中定义的项引入项目包的作用域中来引用它们。

当然,如果使用了太多的use觉得特别复杂,还可以使用{}

```rust
use std::io;
use std::io::Write;

use std::io::{self, Write};

use std::collections::*;

//这个 use 语句将 std::collections 中定义的所有公有项引入当前作用域。
```

##### 第四部分

我们还可以把模块放到各个文件里面

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

声明 `front_of_house` 模块，其内容将位于 *src/front_of_house.rs*

*src/front_of_house.rs* 会获取 `front_of_house` 模块的定义内容，如示例 7-22 所示。

文件名: src/front_of_house.rs

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

 在 *src/front_of_house.rs* 中定义 `front_of_house`模块

