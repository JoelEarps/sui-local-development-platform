pub(crate) fn say_hello() -> bool {
    println!("Hello!");
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn say_hello_test(){
        assert_eq!(say_hello(), true);
    }
}