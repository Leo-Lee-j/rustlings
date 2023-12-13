// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    /// 这个宏接受两个参数，如果这两个参数的值不相等，那么它会引发一个 panic。这个宏用于检查两个值是否相等。
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2, 2);
    }
}
