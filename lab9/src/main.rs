#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // 出现panic!测试就失败了
        // panic!("Make this test fail");
    }
}

// 使用#[cfg(test)]和#[test]来添加测试
// assert! 宏由标准库提供，在希望确保测试中一些条件为 true 时非常有用。
// 需要向 assert! 宏提供一个求值为布尔值的参数。
// 如果值是 true，assert! 什么也不做，同时测试会通过。
// 如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。assert! 宏帮助我们检查代码是否以期望的方式运行。
// 还可以使用 assert_eq! 和 assert_ne! 宏来测试相等
#[cfg(test)]
mod tests_assert {
    #[test]
    fn assert_eq() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn assert_ne() {
        assert_ne!(2 + 2, 5);
    }

    #[test]
    fn assert(){
        assert!(2 + 2 > 3);
    }
    // 还可以在assert后面添加失败信息
    #[test]
    fn assert_with_info(){
        assert!(2 + 2 > 3, "Oh,no!");
    }
    // 可以使用should_panic和expect来让测试点必须panic,并期待panic输出的输出信息
    #[test]
    #[should_panic(expected = "G")]
    fn panicss() {
        panic!("G");
    }

    //还可以使用Result来进行测试,如果返回Ok就通过测试了,如果返回Err就错误
    #[test]
    fn results_OK() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // 函数的输出是不会显示出来的,可以使用cargo test -- --nocapture来让其输出
    // 可以通过cargo test 函数名来运行单个测试
    // 我们可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行。

    // 可以加#[ignore]来忽略
    // 当然可以执行cargo test -- --ignored来执行被忽略的
    #[test]
    #[ignore]
    fn ignore_test(){

    }
}