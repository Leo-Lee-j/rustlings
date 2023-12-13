// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.


struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    /*
    catch_unwind 函数接受一个闭包作为参数，并执行这个闭包。如果闭包中发生了 panic，
    catch_unwind 会捕获这个 panic，并返回一个 Err 结果。如果闭包正常执行完毕，
    catch_unwind 会返回一个 Ok 结果，其中包含了闭包的返回值。
     */
    #[test]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = panic::catch_unwind(|| Rectangle::new(-10, 10));
        assert!(_rect.is_err());
    }

    #[test]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = panic::catch_unwind(|| Rectangle::new(10, -10));
        assert!(_rect.is_err());
    }
}
