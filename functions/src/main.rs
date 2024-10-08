fn main() {
    println!("Hello, world!");

    let result = plus_two_numbers(10, 5);
    println!("the result is {result}");
}

fn plus_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}
