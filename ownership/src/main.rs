#![allow(dead_code, unused_variables)]

fn main() { // scope starts
    // let mut s = String::from("hello"); // store an amount of text that is unknown to us at compile time; s is valid
    // s.push_str(" world"); // appends a string literal to a String
    // println!("{}", s);
    // println!("");
    // println!("copy fixed size value (like integer type):");
    // fixedsizecopy();
    // println!("");
    // println!("deep copy non fixed size value (like String type):");
    // deepcopy_unfixedsize();
    // println!("");
    println!("example of ownership:");
    ownership();
    println!();
    println!("calling values in functions:");
    return_values();
    println!();
    println!("returning multiple values:");
    multivalue();
} // scope ends

fn stringcopy() {
    let sone = String::from("hello");
    let stwo = sone;
    // println!("{}", sone) -> this won't work: sone has been removed and shifted to stwo (only applies to types where there is no fixed size)
}

fn fixedsizecopy() {
    let x = 5; // known size at compile time: stored on stack, quick to make - x is still valid after creating value y
    let y = x;
    println!("x: {}, y: {}", x, y)
}

fn deepcopy_unfixedsize() {
    let s_one = String::from("hello");
    let s_two = s_one.clone(); // code may be expensive: copying same content 2 times
    println!("s1: {}, s2: {}", s_one, s_two)
}

fn ownership() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function and so is no longer valid here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function, but i32 is copy, so it’s okay to still use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string)
} // Here, some_string goes out of scope and drop function for s value is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer)
} // Here, some_integer goes out of scope. Nothing special happens

fn return_values() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s3 = takes_and_gives_back(s1);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("{}", s3);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string                              // some_string is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn multivalue() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
