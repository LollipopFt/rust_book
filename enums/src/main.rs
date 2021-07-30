#![allow(unused_imports, unused_variables, dead_code, unused_assignments)]
use std::net::{Ipv4Addr, Ipv6Addr};

fn main() {
    println!("Hello, world!");
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_addr() {
    let home = IpAddrKind::V4(127, 0, 0, 1); // can use a function to run both Ips: they are of the same 'IpAddrKind' type
    let loopback = IpAddrKind::V6(String::from("::1"));
}

// using enum:
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
} // much shorter

// using struct:
struct QuitMessage; // unit struct
struct Name {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColourMessage(i32, i32, i32); // much longer

impl Message {
    fn call(&self) { // define methods: refer to structs
                     // method defined here
    }
}

fn message() {
    let m = Message::Write(String::from("Hello!")); // create variable m that has Write value (String)
    m.call() // run call method
}

fn option() {
    let some_no = Some(5);
    let some_string = Some("a string");
    let null_no: Option<i8> = None; // need to specify what type is Option<T> to give None 'value'
} // note: i8 cannot be added to Option<i8>: catches assumption that something is not null when it is

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // add info to enum to include UsState value stored inside it
}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // add variable 'state' which binds to Quarter state; use 'state' in code
            println!("State quarter: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i16>) -> Option<i16> {
    x.map(|i| i + 1)
}

fn placeholder() {
    let u8_value = 7u8;
    println!("u8 value: ");
    match u8_value {
        1 => println!("1"),
        3 => println!("3"),
        7 => println!("7"),
        _ => println!(" "), // _ is a placeholder for all values not stated above
    }
}

fn if_let() {
    // only use this for cases where there is only one value to check
    let some_u8 = Some(0u8);
    if let Some(3) = some_u8 {
        println!("three");
    } else {
        println!("not three");
    }
}

fn matchcoin(coin: Coin) {
    // match case for coin
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("state quarter from {:?}", state),
        _ => count += 1, // considered as boilerplate code
    }
}

fn ifletcoin(coin: Coin) {
    // if-let case for coin: less boilerplate
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("state quarter from {:?}", state);
    } else {
        count += 1;
    }
}
