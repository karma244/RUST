use std::io::stdin;

fn main() {
    let mut input_text = String::new();
    stdin()
        .read_line(&mut input_text)
        .unwrap();

    let x = input_text.trim().parse::<i32>().unwrap();
    //INTEGER가 아닌 문자열 입력 시 오류 뱉음, 공백 입력 시 오류 뱉음.
}