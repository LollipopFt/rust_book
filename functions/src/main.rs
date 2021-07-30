fn main() {
    // another_function(5, 2);

    let x: u8 = plus_one(5);
    println!("The value of x is {}", x);
}

// fn another_function(x: u8, y: u8) {
//     println!("The value of x is {}", x);
//     println!("The value of y is {}", y);
// }
fn plus_one(x: u8) -> u8 {
    x + 1
}
