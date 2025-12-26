# Enum 枚举

- 枚举（enum）是一种用户定义的数据类型，用于表示具有一组离散可能值的变量。
  - 每种可能值称为“variant”（变体）。
  - 枚举名：变体名

- 枚举的好处
  - 可以使你的代码更严谨，更易读
  - More robust programs

```rust
enum Shape {
  Circle(f64),
  Rectangle(f64, f64),
  Square(f64),
}
```