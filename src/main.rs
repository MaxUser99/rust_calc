use std::io;
use std::io::Read;

static ACTIONS: [char; 4] = ['+', '-', '*', '/'];

#[derive(Debug)]
enum ActionTypes {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NONE,
}

fn u8_to_action_type(input : &u8) -> ActionTypes {
    match (*input) as char {
        '+' => ActionTypes::ADD,
        '-' => ActionTypes::SUBTRACT,
        '*' => ActionTypes::MULTIPLY,
        '/' => ActionTypes::DIVIDE,
        _ => ActionTypes::NONE
    }
}

fn handle_input() -> ActionTypes {
    let mut buffer = [0; 1];
    io::stdin()
        .read_exact(&mut buffer)
        .expect("Input error");
    buffer
        .first()
        .map(|val| u8_to_action_type(val))
        .unwrap()
}

fn main() {
    let action_type = handle_input();
    println!("action type: {:?}", action_type);
}
