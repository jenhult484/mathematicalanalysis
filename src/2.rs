fn main() {
    let x = 1;
    let y = 2;
    println!("{} + {} = {}", x, y, add(x, y));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
