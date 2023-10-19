use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let input: Vec<&str> = buffer.split_whitespace().collect();
    //input[0]의 형태로 사용합니다.
}