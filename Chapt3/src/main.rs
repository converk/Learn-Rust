fn main() {
    println!("x: {}", another_fn(5) + 1);
    println!("y: {}", another_fn(7) + 2);
}
fn another_fn(x: u32) -> u32 {
    x + 1
}
