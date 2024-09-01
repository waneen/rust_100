fn main(){
    // This is a single-line comment
    // Followed by another single-line comment

    let content = greeting();

    println!("{}",content);
    println!("{}",greet("hogeta"))
}

// `fn` <function_name> ( <input params> ) -> <return_type> { <body> }
fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to learn Rust!"
}

// An input parameter
//        👇
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests{
    use crate::greeting;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}