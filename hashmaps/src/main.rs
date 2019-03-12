use std::collections::HashMap;

fn main() {
    creating();
    ownership();
    accessing();
    updating();
}

fn creating() {
    // creating a new hash mpa and inserting some keys and values
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    // using the collect method on a vector of tuples,
    // where each tuple consists of a key and its value.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("The scores is {:?}", scores);
}

fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // NOTE field_name and field_value are invalid at this point,
    // For types implement the Copy trait like i32, the values are copied into the hash map,
    // For owned values like String, the values will be moved and the hash map will be the owner of those values.
}

fn accessing() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("The score of blue team is {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn updating() {

    let mut scores = HashMap::new();

    // overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // only inserting if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // updating based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
