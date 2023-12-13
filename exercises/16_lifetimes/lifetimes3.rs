// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // 在结构体中使用引用时，你需要明确指定这个引用的生命周期
    // Book 结构体添加了 'a 生命周期参数, 意味着 name 和 title 从创建开始，
    // 只要 main 函数的作用域还在，'a 的生命周期就还在，Book 结构体实例就可以安全地使用 author 和 title 字段。
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
