use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
mod GameData;

fn main() {
    let mut file = File::open("Resources.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data: GameData::GameData = serde_json::from_str(&contents).unwrap();
    print!("{}", a);
    roll_dice(3);
}
fn roll_dice(num_dice: i32) -> i32 {
    let mut tiradas: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut total = 0;
    print!("Rolling {}d6 ", num_dice);
    print!("Rolled ");
    for _ in 0..num_dice {
        let roll = rng.gen_range(1, 7);
        tiradas.push(roll);
        print!("[{}]", roll);
        total += roll;
    }
    print!("Total: {}", total);
    total
}

