use std::io::stdin;

//BOJ 2438
fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let input = buffer.trim().parse::<i32>().unwrap();

    for i in 1..=input {
        for j in 0..i {
            print!("*");
        }
        println!();
    }
}