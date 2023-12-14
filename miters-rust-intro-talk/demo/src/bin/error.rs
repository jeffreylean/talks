fn get_element(v: Vec<i32>, index: usize) {
    if index >= v.len() {
        panic!("panic with overflow：{}", index);
    }
    println!("{}", v[index]);
}

fn main() {
    let list = vec![1, 2, 3, 4];
    get_element(list, 5);
}
