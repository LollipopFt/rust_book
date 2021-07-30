fn main() {
    borrowing();

    let x = String::from("boku no yume");
    let fw = first_word(&x);
    println!("{}", fw);
}

fn first_word(s: &String) -> &str { // do not want ownership: use &;
    let bytes = s.as_bytes(); // convert String to array of bytes (as_bytes method)

    for (i, &item) in bytes.iter().enumerate() { // iterate over array of bytes w/ iter method; enumerate wraps the result of iter and returns each element as part of a tuple
                                                 // iter returns each element in a collection; first element of tuple returned from enumerate is index, second element is reference to element
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..] // if no space in String, take the whole string
}

fn borrowing() {
    let mut s1 = String::from("supercalifragilisticexpialidocious");
    let len    = cal_length(&s1); // references s1: will not drop s1 in the function cal_length when it goes out of scope; can have unlimited number of immutables
    println!("The length of {} is {}.", s1, len);
    let t      = change(&mut s1); // note: cannot have a mutable reference while having an immutable before the immutable is used
    println!("{}", t)
}

fn cal_length(s: &String) -> usize { // &String must be used to match &s1 used in making len value
    s.len()
}
// carried on from ownership crate

fn change(t: &mut String) -> String {
    t.push_str(" world");
    t.to_string()
}

// Rules of References:
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.