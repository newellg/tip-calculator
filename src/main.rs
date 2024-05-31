
use std::io;
use std::io::{stdout, Write};
fn read_f32() -> f32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}
fn main() {
    println!("Welcome to the tip calculator");

    println!("What was the total bill amount?: ");
    let total_bill = read_f32();
    let _=stdout().flush();
    println!("What percentage would you like to give? 10, 12, or 15?");
    let tip_percentage :f32 = read_f32();
    println!("How many people to split the bill?");
    let people_split :f32 = read_f32();
    let per_person :f32 = (total_bill/people_split) * (1.0 + tip_percentage / 100.0);

    println!("Each person should pay ${:.2}", per_person );


}
