# Struct 结构体

结构体是一种用户定义的数据类型，用于创建自定义的数据结构。

```rust
struct Point {
  x: i32,
  y: i32,
}
```

每条数据（x 和 y）称为属性（字段 field）
通过点(.)来访问结构体的属性。

##  结构体中的方法

这里的方法是指，通过实例调用（&self、&mut self、self）

```rust
impl Point {
  fn distance(&self, other: &Point) -> f64 {
    let dx = (salf.x - other.x) as f64;
    let dy = (salf.y - other.y) as f64;
    (dx * dx + dy * dy).sqrt()
  }
}
```

## 结构体中的关联函数

关联函数是与类型相关联的函数，调用时为结构体名::函数名。

```rust
impl Point {
  fn new(x: i32, y: i32) -> Point {
    Point { x, y }
  }
}
```

## 结构体中的关联变量

这里的关联变量是指，和结构体类型相关的变量，也可以在特质或是枚举中。
```rust
impl Point {
  const PI: f64 = 3.14;
}
```
调用时使用 Point::PI