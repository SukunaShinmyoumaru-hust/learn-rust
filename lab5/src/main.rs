// 增加属性来派生 Debug trait，并使用调试格式打印 Rectangle 实例
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//当然,我们可以在impl块中添加实现的方法,然后我们就可以调用方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        //当然还可以使用!dbg宏来输出debug输出.
        width: dbg!(30),
        height: 50,
    };

    println!(
        //{} Display mode.{:?} Debug mode
        "The area of the rectangle is {} square pixels.",
        area1(&rect1)
    );

    println!(
        //{} Display mode.{:?} Debug mode
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}