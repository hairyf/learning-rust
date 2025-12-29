fn main() {
    // 进制的字面量
    let a1 = -125;
    let a2 = 0xFF;
    let a3 = 0o13;
    let a4 = 0b10;

    println!("{a1}, {a2}, {a3}, {a4}");

    // Max Min
    println!("u32 max: {}, min: {}, bytes: {}", u32::MAX, u32::MIN, std::mem::size_of::<u32>());
    println!("i32 max: {}, min: {}, bytes: {}", i32::MAX, i32::MIN, std::mem::size_of::<i32>());
    println!("u64 max: {}, min: {}, bytes: {}", u64::MAX, u64::MIN, std::mem::size_of::<u64>());
    println!("usize max: {}, min: {}, bytes: {}", usize::MAX, usize::MIN, std::mem::size_of::<usize>());
    println!("isize max: {}, min: {}, bytes: {}", isize::MAX, isize::MIN, std::mem::size_of::<isize>());

    // float
    let f1:f32 = 1.23234;
    let f2:f64 = 9.88888;
    println!("f1: {:.2}, f2: {:.2}", f1, f2);

    // bool
    let is_ok = true;
    let can_ok:bool = false;
    println!("is_ok? {is_ok}, can_ok? {can_ok}");
    println!(
        "is ok or can ok?{}, can ok and is ok?{}", 
        is_ok || can_ok, 
        can_ok && is_ok
    );

    // char
    let char_c = 'a';
    let emo_char = '🍎';
    println!("You Get {char_c} feel {emo_char}");
    println!("{}", emo_char as usize);
    println!("{}", emo_char as i32);
}
