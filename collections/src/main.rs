#[allow(unused_variables)]
mod hashmap;
use crate::hashmap::hashmaps;

fn main() {
    vector_match();
    iter();
    enumerator();
    str_op();
    str_slice();
    println!("Using hashmaps:");
    hashmaps::main();
    // note: cannot reference a vector then add a (mutable) value to a vector without using reference
} // vectors are freed once out of scope

fn vector_match() {
    let _v: Vec<i8> = Vec::new();
    let _v = vec![1, 2, 3]; // create a new vector

    let w: Vec<i8> = vec![2, 4, 5];
    let _thrd: &i8 = &w[2]; // &i8 needed since we are using &w[2]; need & to reference w[2]
                           // program will panic here if w only has 2 elements
    match w.get(2) {
        Some(thrd) => println!("The third value of vector is {}.", thrd),
        None => println!("There is no third value."),
    }
    // note: cannot reference a vector then add a (mutable)value to a vector without using reference
} // vectors are freed once out of scope

fn iter() { // iterating over values in a vector
    let v = vec![15, 22, 10];
    for i in &v { // iterating over immutable references
        println!("{}", i);
    }

    let mut w = v;
    for i in &mut w { // iterating over mutable references to make changes to all elements
        *i+=50; // use dereference operator(*) to get value in i to use += operator
        println!("{}", i);
    }
}

fn enumerator() { // vectors can only store values of the same type
    enum Spreadsheet { // use enum since all variants of an enum are defined under same enum type
        Int(i8),
        Float(f32),
        Text(String),
    }

    let _row = vec![ // vector holding different types
    Spreadsheet::Int(3),
    Spreadsheet::Text(String::from("hello")),
    Spreadsheet::Float(10.12),];
}

fn str_op() {
    let _s = String::new(); // create empty string
    let data = "printed contents"; // initial data
    let _s = data.to_string(); // adding data to string: can also do "printed contents".to_string(); & 
                               // let _s = String::from("printed contents"); -> String::from & to_string are the same
    let mut t = "foo".to_string();
    let t2 = "bar";
    t.push_str(t2); // appending string slice to string; t = "foobar": can also do t.push_str("bar");
                    // note: push_str takes string slice so no ownership of push_str(obj<-) => object(t2) can be reused
    t.push('l'); // adds 1 character as a parameter to String
    let u = "Hello ".to_string();
    let u2 = "World".to_string();
    let _u3 = u+&u2; // u is no longer valid as it has been used; u2 referenced as can only add &str(u2) to String(u)
                     // u2 is originally &String -> when add(+) is called, deref coercion turns &String to &str
    let u = "Hello".to_string();
    let u1 = "!".to_string();
    let u3 = format!("{} {}{}", u, u2, u1); // use format! macro for more complicated string combinations
    println!("This is u3: {}", u3);
}

fn str_slice() {
    let s = "stuff";
    // let s_slice = &s[2] <- this is not accepted: cannot index with 1 value
    let _s_slice = &s[0..3]; // when slicing, always use a range, representing the byte order in the string

    println!("Characters:");
    for c in "tnhuo".chars() { // calling .chars() separates & returns values of char: separate words of other languages
        println!("{}", c);
    }

    println!("Bytes:");
    for c in "tnhuo".bytes() { // prints out bytes instead of chars
        println!("{}", c);
    }
}