// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    /*{
        /*
        问题在于 main 函数中的 string2 变量。在 main 函数中，string2 是在一个新的作用域中创建的，
        当这个作用域结束后，string2 就会被销毁。但是 longest 函数返回的是 string1 或 string2 的引用，
        这就可能导致 result 变量在 string2 被销毁后仍然被使用。
         */
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }*/
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
