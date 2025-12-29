enum Flavor {
  Spicy,
  Sweet,
  Fruity,
}

struct Drink {
  flavor: Flavor,
  price: f64,
}

impl Drink {
  // 关联常量
  const MAX_PRICE: f64 = 10.0;
  // 关联方法
  fn buy(&self) {
    if self.price > Drink::MAX_PRICE {
      println!("I am poor");
      return;
    }
    println!("buy it");
  }
  // 关联函数
  fn new(price: f64) -> Drink {
    return Drink {
      flavor: Flavor::Sweet,
      price,
    };
  }
}

fn print_drink(drink: Drink) {
  match drink.flavor {
    Flavor::Spicy => println!("Spicy"),
    Flavor::Sweet => println!("Sweet"),
    Flavor::Fruity => println!("Fruity"),
  }
  println!("{}", drink.price);
}

fn main() {
  let sweet = Drink { 
    flavor: Flavor::Sweet,
    price: 1.99
  };
  println!("{}", sweet.price);
  print_drink(sweet); // ownership transfer

  let sweet = Drink::new(16.0);
  sweet.buy();
}
