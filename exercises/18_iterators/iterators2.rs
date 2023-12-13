// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    /// 调用 next 方法时，迭代器会前进一步，返回当前位置的元素，并将内部的位置指针向前移动一位。
    /// 所以，当你调用 c.next() 后，c 迭代器的当前位置就不再是第一个字符，而是第二个字符。
    /// 这就是为什么后面的 c.as_str() 只包含了除了第一个字符之外的所有字符。
    match c.next() {
        None => String::new(),
        //  first.to_uppercase() 将首字母转换为大写。to_uppercase 方法返回的是一个 char 的迭代器，因为某些字符在转换为大写后可能会变为多个字符（例如，某些 Unicode 字符）。
        // .collect::<String>() 是一个从迭代器收集元素并将它们组合成一个特定类型的方法。在这里，它将 char 的迭代器转换为一个 String。
        // + c.as_str() 将剩余的字符（c）添加到首字母后面。as_str 方法将字符迭代器转换为字符串切片，然后 + 运算符将这两个字符串连接起来。
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str()
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect::<Vec<_>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
