fn main() {
    // copy
    let c1 = 1;
    let c2 = c1;
    println!("{c1} {c2}");

    // move
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // s1 的所有权转移给 s2, 如果想使用 s1，需要 clone() 方法
    // println!("{s1} {s2}"); // value borrowed here after move
    println!("{s2}");

    let len = get_length(s1); // 所有权转移给 get_length 函数
    // println!("{s1}"); // value borrowed here after move
    println!("{}", len);

    let back = first_word("Hello World");
    println!("{}", back);
    let back = first_word("we are the world");
    println!("{}", back);
}

fn get_length(s: String) -> usize {
  println!("String {}", s);
  // 函数结束之后, main::s1 也销毁了，所以无法再使用 s1
  s.len()
}

// missing lifetime specifier
fn dangle() -> String {
  "hello".to_owned()
}

// 静态的生命周期
fn dangle_static() -> &'static str {
  "jdkfj"
}

// String 与 &str vec u8 ref
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}