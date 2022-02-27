
pub fn test1() -> i32 {
    let s = "ghoehgoe".to_string();
    45
}


#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    //#[ignore]
    fn test_sample1() {
        let result = 2 + 2;

        println!("get result value {}", result);

        println!("this is message from test sample 1");

        assert_eq!(
            result, 4,
            "Couldn't eq, by {}",
            result
        );
    }
}
