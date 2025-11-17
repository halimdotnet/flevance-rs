use rand::Rng;
use rand::prelude::SliceRandom;


fn main() {
    let str = random_char(16);
    println!("Random Character: {}", str);

    let num = random_num(6);
    println!("Random Number: {}", num);

    match random_num_unique(8) {
        Ok(unique_num) => println!("Random Unique Number: {}", unique_num),
        Err(e) => println!("Error: {}", e),
    }
}

fn random_char (len : usize) -> String {
    let lowercases = "abcdefghijklmnopqrstuvwxyz";
    let uppercases = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "`~!@#$%^&*()-_=+{}[]|\\:;\"',<.>/?";

    let charset: Vec<char> = format!("{}{}{}{}", lowercases, uppercases, numbers, symbols)
        .chars()
        .collect();

    let mut rng = rand::rng();

    (0..len)
        .map(|_| charset[rng.random_range(0..charset.len())])
        .collect()
}

fn random_num (len : usize) -> String {
    let mut rng = rand::rng();
    (0..len)
        .map(|_| rng.random_range(0..10).to_string())
        .collect()
}

fn random_num_unique (len : usize) -> Result<String, String> {
    let mut ds: Vec<char> = "0123456789".chars().collect();
    let mut rng = rand::rng();
    ds.shuffle(&mut rng);

    Ok(ds.iter().take(len).collect())
}
