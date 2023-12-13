// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // if let 可以结合 Some 来匹配
        // 在这个例子中，optional_target 是一个 Option 类型的值，
        // if let 语句试图将其解构为 Some(word)。
        // 如果 optional_target 确实是 Some 类型的值，
        // 那么 word 就会被绑定到这个 Some 中的值，
        // 然后执行 assert_eq!(word, target); 这行代码。
        // 如果 optional_target 是 None，那么 if let 语句就什么都不做。
        if let Some(word) = optional_target {
            println!("'word' is {}", word);
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
