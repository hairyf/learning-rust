fn main() {
    // 不可变与命名
    let _nice_count = 100; // 自动推导 i32 类型
    let _nice_number:i64 = 54;
    // nice_count = 32; // 报错，因为 nice_count 是不可变的

    // 声明可变
    let mut _count:i32 = 3;
    _count = 4;

    // shadowing
    let x = 5;
    {
        let x = 10;
        println!("inner x = {x}");
    } // 内部的 x 被销毁了
    println!("outer x = {x}");

    let x = "hello";

    println!("x = {x}"); // 同一作用域内重新声明变量，会覆盖之前的变量

    let mut x = "this";

    println!("x = {x}");

    x = "that";

    println!("x = {x}");
}
