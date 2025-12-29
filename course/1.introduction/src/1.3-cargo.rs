fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "你好，世界！";
    let english = "Hello, world!";
    let regions = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", region);
    };
}

fn main() {
    greet_world();
}
