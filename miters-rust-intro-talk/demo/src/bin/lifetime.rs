#![allow(dead_code)]

fn longest_str<'b>(s1: &'b str, s2: &'b str) -> &'b str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = "Hello".to_string();
    let string2 = "World".to_string();
    let result = longest_str(&string1, &string2);

    println!("The longest string is {}", result);
}
