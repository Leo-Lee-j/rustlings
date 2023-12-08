# Functions

Here, you'll learn how to write functions and how the Rust compiler can help you debug errors even
in more complex code.

## Further information

- [How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## 知识点
1. 定义函数
关键字为 `fn`
```rust
fn test() {
    println!("Hello, world!");
}
```

2. 函数参数
函数参数需要定义参数类型
```rust
fn test(x: i32) {
    println!("x is {}", x);
}
```
3. 函数返回值
* 函数返回值需要定义返回类型
* 当函数返回值为最后一行时，可以省略分号
```rust
fn test(x: i32) -> i32 {
    x * x
}
```

