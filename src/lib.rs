fn main() {
    let str: &str = "Rio";
    let string: String = "Rio".to_string();
    
    let bytes = vec![82, 105, 111];
    let str_from_bytes = std::str::from_utf8(&bytes).unwrap();
    
    println!("{}", str); // "Rio" (with encode/decode)
    println!("{}", string); // "Rio" (with encode/decode)
    println!("{}", str_from_bytes); // encode 82, 105, 111 to "Rio"
    println!("{}", str_from_bytes); // encode 82, 105, 111 to "Rio"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str: &str = "Rio";
        let string: String = "Rio".to_string();
        
        let bytes = vec![82, 105, 111];
        let str_from_bytes = std::str::from_utf8(&bytes).unwrap();
        
        assert_eq!("{}", str); // "Rio" (with encode/decode)
        assert_eq!("{}", string); // "Rio" (with encode/decode)
        assert_eq!("{}", str_from_bytes); // encode 82, 105, 111 to "Rio"
        assert_eq!("{}", str_from_bytes); // encode 82, 105, 111 to "Rio"
            }
}
