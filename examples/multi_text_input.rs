use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    
    let input:Vec<usize> = buffer.split_whitespace().map(|s| s.trim().parse().unwrap()).collect::<Vec<_>>();
    //input[0], input[1]등의 형식으로 사용함.
}