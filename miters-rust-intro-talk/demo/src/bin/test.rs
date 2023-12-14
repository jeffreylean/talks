pub fn addition(a: i64, b: i64) -> i64 {
    a + b
}

pub fn pop(a: &mut Vec<i32>) {
    a.pop();
}

fn main() {
    let a = 1;
    let b = 1;
    println!("{}+{}={}", a, b, addition(a, b))
}

#[cfg(test)]
mod tests {
    use crate::addition;

    #[test]
    fn test_addition() {
        assert_eq!(addition(1, 1), 2);
    }
}
