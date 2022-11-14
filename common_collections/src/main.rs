use::std::collections::HashMap;

fn main() {
    vector_example();
    hash_map_example();
}

fn make_vector() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v 
}

fn vector_example() {
    let mut vec = make_vector();
    vec.push(7);
    println!("{:?}", vec);

    let third: Option<&i32> = vec.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &mut vec {
        *i += 55;
        println!("{:?}", i);
    }    
}

fn hash_map_example() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 22);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score is {}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}