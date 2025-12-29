fn main() {
  // let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容：
  let (a, mut b): (bool,bool) = (true, false);
  // a = true,不可变; b = false，可变
  // 对于 bool 类型，{} 和 {:?} 的输出通常相同，但 {:?} 更通用，适用于所有实现了 Debug 的类型。
  println!("a = {:?}, b = {:?}", a, b);

  b = true;
  assert_eq!(a, b);
}

struct Struct {
  e: i32
}

fn main_2() {
  let (a, b, c, d, e);

  (a, b) = (1, 2);
  // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
  [c, .., d, _] = [1, 2, 3, 4, 5];
  Struct { e, .. } = Struct { e: 5 };

  assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}