use std::io;

fn main() {
    println!("Enter string: ");
    
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to get string");

    println!("{}", palindrome(string));
    
}

fn palindrome(string: String) -> bool {
    let string = string.trim();
    let reverse = string.chars().rev().collect::<String>();
    
    return string==reverse;
}
