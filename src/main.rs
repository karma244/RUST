use std::io::stdin;

fn main() {
    let mut input_text = String::new();
    stdin().read_line(&mut input_text).unwrap();
    let x = input_text.trim().parse::<i32>().unwrap();
    for i in 1..=x {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let input:Vec<usize> = buffer.split_whitespace().map(|s| s.trim().parse().unwrap()).collect::<Vec<_>>();
        
        print!("Case #{i}: {}\n", input[0]+input[1]);
    }
}