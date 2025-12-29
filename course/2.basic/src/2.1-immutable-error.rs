fn main() {
  let x = 5;
  println!("The value of x is: {}", x);
  // rust 的默认变量是不可变的，所以不能再次赋值
  x = 6;
  println!("The value of x is: {}", x);
}