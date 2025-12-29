fn main() {
  // 变量遮蔽(shadowing)
  let x = 5;
  // 在 main 函数的作用域内对之前的 x 进行遮蔽
  let x = x + 1;

  {
    // 在当前的花括号作用域内，对之前的 x 进行遮蔽
    let x = x + 2;
    println!("The value of x in the inner scope is: {}", x);
  }


  println!("The value of x is: {}", x);

  // 这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。
  // 变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
  let spaces = "   ";
  // usize数值类型
  let spaces = spaces.len(); // 字符串类型
  println!("The value of spaces is: {}", spaces);
}

fn main_3() {
  let x = define_x();
  println!("{}, world", x); 
}

fn define_x() -> &'static str {
  let x = "hello";
  return x;
}