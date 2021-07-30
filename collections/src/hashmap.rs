#[allow(unused_mut, dead_code)]

pub mod hashmaps {
    use std::collections::HashMap;

    pub fn main() {
        let x = hashmap_eg();
        println!("{:?}", x);
        println!("\nGetting values:");
        access();
        println!("\nUpdating values:");
        update();
        println!("\nInserting value after checking there is none:");
        insert_if_no_value();
        println!("\nUpdating values based on old values:");
        update_based_on_old();
        println!("\nUpdating old values if or_insert had initial value of 1:");
        update_based_on_old_w_insert_one();
    }
    fn hashmap_eg() -> HashMap<String, i8> {
        let mut score = HashMap::new();
        score.insert("blue".to_string(), 10);
        score.insert("yellow".to_string(), 50);

        let teams = vec!["blue".to_string(), "yellow".to_string()];
        let init_scores = vec![10, 50];
        let mut scores: HashMap<String, i8> = teams.into_iter().zip(init_scores.into_iter()).collect();
        /* => zip method to create a vector of tuples where "blue" is paired with "10", etc.
        => collect method gathers data into a number of collection types including HashMap: used to turn vector of 
        tuples from zip to a hashmap, defined by HashMap<_, _> function; _ makes Rust infer data types */
        scores
    }

    fn borrow() {
        let field = "favourite colour".to_string();
        let colour = "blue".to_string();
        let mut map = HashMap::new();
        map.insert(field, colour); // field, colour are invalid; types using owned trait like String are moved while 
                                   // types using Copy trait like u8 are just copied and can be reused -> can use & 
                                   // operator to reference and continue using objects
    }

    fn access() {
        let scores = hashmap_eg();
        let team_name = "blue".to_string();
        let score = scores.get(&team_name); // result: Some(&10) -> .get() returns Option<&V>
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    fn update() {
        let mut scores = hashmap_eg();
        println!("{:?}", scores);
        scores.insert("blue".to_string(), 20);
        println!("{:?}", scores);
    }

    fn insert_if_no_value() {
        let mut scores = HashMap::new();
        scores.insert("blue".to_string(), 10);
        println!("{:?}", scores);
        scores.entry("blue".to_string()).or_insert(30); /* or_insert: return a mutable reference to value of Entry 
        (.entry) key if key exists, else insert parameter as new entry of key, returns mutable reference to new value */
        scores.entry("yellow".to_string()).or_insert(50);
        println!("{:?}", scores);
    }

    fn update_based_on_old() {
        let text = "a cat ate a dog";
        let mut textmap = HashMap::new();
        for l in text.split_whitespace() {
            let count = textmap.entry(l).or_insert(0); // return &mut V and stores in count; or_insert(0) sets initial 
                                                       // value of number of words to 0
            *count+=1; // dereference value to store new value from above in it
        } // mutable reference goes out of scope here
        println!("{:?}", textmap);
    }

    fn update_based_on_old_w_insert_one() {
        let text = "a cat ate a dog";
        let mut textmap = HashMap::new();
        for l in text.split_whitespace() {
            let count = textmap.entry(l).or_insert(1);
            *count+=1;
        }
        println!("{:?}", textmap);
    }
}