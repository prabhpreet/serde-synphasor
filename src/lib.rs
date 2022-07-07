/// # Test
///
/// ```
/// let x = 5;
/// ```

pub struct Test;
impl Test {
    fn hello() {
        println!("Hello");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
