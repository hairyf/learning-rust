enum Color {
  Red,
  Yellow,
  Blue,
  Black,
}

fn print_color(my_color: Color) {
  match my_color {
    Color::Red => println!("Red"),
    Color::Yellow => println!("Yellow"),
    Color::Blue => println!("Blue"),
    _ => println!("Unknown color"),
  }
}

enum BuildingLoaction {
  Number(i32),
  Name(String), // 不要用 &str
  Unknown,
}

impl BuildingLoaction {
  fn print_location(&self) {
    match self {
      // BuildingLoaction::Number(44)
      BuildingLoaction::Number(c) => println!("building number: {}", c),
      // BuildingLoaction::Name("ok".to_string())
      BuildingLoaction::Name(n) => println!("building name: {}", n),
      BuildingLoaction::Unknown => println!("unknown"),
    }
  }
}

fn main() {
    // print_color(Color::Black);
    let a = Color::Red;
    print_color(a);
    // let b = a; // value used here after move

    let house = BuildingLoaction::Name("fdfd".to_string());
    let house = BuildingLoaction::Number(1);
    let house = BuildingLoaction::Unknown;
    house.print_location();
}
