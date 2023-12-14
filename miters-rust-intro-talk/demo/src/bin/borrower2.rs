pub fn pop(a: &mut Vec<i32>) {
    a.pop();
}

fn main() {
    let mut test = vec![1, 2, 3];
    let last = test.last().unwrap();
    println!("{last}");
    pop(&mut test);

    println!("{:?}", test);
}
