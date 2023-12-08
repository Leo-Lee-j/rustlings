// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // 不可变的变量无法再次赋值变更, 可以修改为可变变量, 添加 mut 关键字
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
