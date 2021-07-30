#![allow(dead_code)]
use std::io;

fn main() {
    // println!("loop:");
    // normalloop();
    // println!("");
    // println!("while loop:");
    // whileloop();
    // println!("");
    // println!("for loop using while:");
    // forloop_usingwhile();
    // println!("");
    // println!("for loop:");
    // forloop();
    // println!("");
    // println!("improvement of while loop using for loop:");
    // forupgraded_whileloop();
    // println!("");
    println!("fibonacci sequence:");
    fibonacci();
}

fn normalloop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // semicolon assigns value after break (counter * 2) to result (let function)

    println!("The result is {}", result)
}

fn whileloop() {
    let mut num = 3;
    while num != 0 {
        println!("{}", num);

        num -= 1;
    }

    println!("LIFTOFF!")
}

fn forloop_usingwhile() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        // error prone: program will panic if the index length is incorrect
        println!("The value is {}.", a[index]);

        index += 1;
    }
}

fn forloop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        // a.iter() executes code for each item in a collection
        println!("The value is {}", element)
    }
}

fn forupgraded_whileloop() {
    for num in (1..4).rev() {
        // (1..4) is a range from 1 to 3; rev() function reverses the order
        println!("{}", num);
    }
    println!("LIFTOFF!")
}

fn fibonacci() {
    let mut numstring = vec![0, 1];

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    let n: u32 = n.trim().parse().unwrap();

    for _i in 0..(n - 1) {
        numstring.push(numstring[numstring.len() - 1] + numstring[numstring.len() - 2]);
        numstring.remove(0);
    }
    println!("{:?}", numstring);
    println!("nth fibonacci: {}", numstring[numstring.len() - 1])
}
