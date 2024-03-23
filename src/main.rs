fn main() {
    println!("Hello, world!");
}

mod test {

    #[test]
    fn test_main() {
        crate::main();
        assert!(true);
    }
}
