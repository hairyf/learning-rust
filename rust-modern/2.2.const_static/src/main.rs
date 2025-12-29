static MY_STATIC: i32 = 42;
static mut MY_MUT_STATIC: i32 = 42;

fn main() {
    const SECOND_HOUR: usize = 3_600;
    const SECOND_DAY: usize = 24 * SECOND_HOUR; // dompile-time constant

    {
        const SE: usize = 1_000;
        println!("{SE}");
    }

    println!("{SECOND_DAY}");
    // println!("{SE}"); // 报错，因为 SE 在另一个作用域内

    println!("{MY_STATIC}");
}
