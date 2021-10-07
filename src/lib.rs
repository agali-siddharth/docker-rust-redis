#[cfg(test)]
mod tests {
    fn adder(a :i32, b :i32) -> i32 {
        a + b
    }
    #[test]
    fn it_works() {
        assert_eq!(adder(3,2), 5);
    }
}
