#![allow(dead_code, unused_variables)]

fn main() {
/*    rec();
    shortrec();
    prop_rec();*/
    let rect2 = Rec {
        width: 10,
        height: 34,
    };
    let rect3 = Rec {
        width: 5,
        height: 33,
    };
    let rect4  = Rec {
        width: 11,
        height: 35,
    };

    let sq1 = Rec::square(5);

    println!("sq1: {:?}", sq1);
    println!("Area of sq1 is {}.", sq1.area());
    println!("The area of the rectangle is {} square pixels.", rect2.area());
    println!("Can rec3 fit in rec2? {}", rect2.can_hold(&rect3));
    println!("Can rec4 fit in rec2? {}", rect2.can_hold(&rect4));
}

fn users() {

struct User {
username: String,
email: String,
signin_count: u128,
active: bool,
}

let johnwalk = User {
    email: String::from("johnny1walker0@gmail.com"),
    username: String::from("jawoh"),
    active: true,
    signin_count: 1,
	};

let mut rayleo = User {
    username: String::from("raydenleonard"),
    email: String::from("raydenleonard@gmail.com"),
    signin_count: 4,
    active: false,
	};

rayleo.email = String::from("rayleo@gmail.com");

fn builduser(email: String, username: String) -> User {
    User {
        email, // field init shorthand: can only be used when parameter and struct field names are exactly the same
        username,
        signin_count: 1,
        active: true,
    }
}

// creating instances from other instances:
let creator = User {
    email: String::from("creator@hall.com"),
    username: String::from("creategod"),
    signin_count: johnwalk.signin_count,
    active: rayleo.active,
};

let superuser = User {
    email: String::from("alphauser@gmail.com"),
    username: String::from("superman"),
    ..creator // remaining fields not explicitly set has the same values in the given instance
};

}

// tuple structs to without named fields to create different types:
fn colours_points() {
struct Colour(i32, i8, i16);
struct Point(i8, i16, i32);
let black = Colour(0, 0, 0);
let origin = Point(0, 0, 0);
}

// Finding area of rectangle
fn rec() {
	let width_one = 30;
	let height_one = 50;
	println!("The area of the rectangle is {} square pixels.", area(width_one, height_one))
}

fn area(width: u16, height: u16) -> u32 {
	(width * height).into()
}

// Finding above in a more concise manner
fn shortrec() {
	let rec_one = (20, 60);
	println!("The area of the rectangle is {} square pixels.", shortarea(rec_one))
}

fn shortarea(dimensions: (u16, u16)) -> u32 {
	(dimensions.0 * dimensions.1).into()
}

// Finding above with a structure
#[derive(Debug)]
struct Rec {
    width: u16,
    height: u16,
}

fn prop_rec() {
    let rect1 = Rec {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", prop_area(&rect1));
    println!("rect1 is {:?}", rect1);
}

fn prop_area(rec: &Rec) -> u32 {
    (rec.width * rec.height).into()
}

impl Rec { // defining function within context of Rec: implementation block
    fn area(&self) -> u32 { // &self is considered a method syntax; used as borrowing an immutable object
        (self.width * self.height).into() // instance: adding a . after a name
    }

    fn can_hold(&self, other: &Rec) -> bool {
        self.width > other.width && self.height > other.height // width of ->rect2.can_hold > than width of can_hold(&rect3)<- , vice versa
    }

    fn square(length: u16) -> Rec { // does not use self as parameter: calls like String::from
        Rec {
            width: length,
            height: length,
        }
    }
} // you could have multiple impl blocks containing different functions, but there is no need