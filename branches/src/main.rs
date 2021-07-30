use std::io;

fn main() {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line.");

    let num: u8 = num.trim().parse().unwrap();

    if num % 5 == 0 {
        println!("condition true: you chose {}", num);
    } else if num == 0 {
        println!("condition none: you chose {}", num);
    } else if num / 2 == 2 {
        println!("condition false: you chose {}", num);
    } else {
        println!("condition absoslute: you chose {}", num);
    }

    let condition: bool = false; // if true, print 5
    let output = if condition { 5 } else { 6 }; // must be the same type (i, u, bool, etc.)
    println!("Value: {}", output);
}
