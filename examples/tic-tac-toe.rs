use std::io::stdin;

fn update_field(state: &[char]) {
    print!("----SCORE----");
    for i in 0..3 {
        let offset = i * 3;

        print!("\n-------------\n| ");
        print!("{}", state[offset]);
        print!(" | ");
        print!("{}", state[offset+1]);
        print!(" | ");
        print!("{}", state[offset+2]);
        print!(" |");
    }

    print!("\n-------------\n");
}

fn turn_changed(state: &mut [char], player: char) {
    loop {
        print!("Player: {}\nEnter the number:\n", player);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                print!("Enter the valid field number");
                continue;
            }
    
            let number = number - 1;
    
            if state[number] == 'X' || state[number] == 'O' {
                println!("This field is already taken by : {}", player);
                continue;
            }
    
            state[number] = player;
            break;
        }
        else {
            println!("Enter the valid text.");
            continue;
        }
    }
}

fn exist_winner(state: &[char]) -> bool{
    for i in 0..3 {
        if state[i] == state[i + 3] && state[i] == state[i + 6] {
            return true;
        }

        let i = i * 3;

        if state[i] == state[i + 1] && state[i] == state[i + 2] {
            return true;
        }
    }

    if (state[0] == state[4] && state[0] == state[8])
        || (state[2] == state[4] && state[2] == state[6]) {
        return true;
    }

    return false;
}

//Making a Tic-Tac-Toe game 
fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut now_player = 'X';

    loop {
        update_field(&state);

        turn_changed(&mut state, now_player);

        if exist_winner(&state) {
            update_field(&state);
            println!("Player: {} win!!!!!!", now_player);
            break;
        }

        now_player = if now_player == 'X' {'O'} else {'X'}
    }
}