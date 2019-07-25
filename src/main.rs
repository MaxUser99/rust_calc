use std::io;
use std::io::Read;

const NUM_COUNT: usize = 2;
const BUFFER_SIZE: usize = 1;

#[derive(Debug)]
enum ActionTypes {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NONE,
}

// console clear
fn cls() {
    print!("{}[2J", 27 as char);
}

fn u8_to_action_type(input : &u8) -> ActionTypes {
    match char::from(*input) {
        '+' => ActionTypes::ADD,
        '-' => ActionTypes::SUBTRACT,
        '*' => ActionTypes::MULTIPLY,
        '/' => ActionTypes::DIVIDE,
        _ => ActionTypes::NONE
    }
}

fn handle_action_input() -> ActionTypes {
    let mut buffer = [0; BUFFER_SIZE];

    io::stdin()
        .read_exact(&mut buffer)
        .expect("Input error");
    u8_to_action_type(buffer.first().unwrap())
}

fn handle_var_input() -> [f64; NUM_COUNT] {
    let mut input = String::new();
    let mut res : [f64; NUM_COUNT] = [0.; NUM_COUNT];
    let mut i = 0;
    loop {
        if i == NUM_COUNT {
            break;
        }
        io::stdin()
            .read_line(&mut input)
            .expect("input error");
        match input.trim().parse::<f64>() {
            Ok(val) => {
                res[i] = val;
                i += 1;
            },
            Err(_) => println!("not expected input")
        }
        input = String::new();
    }
    res
}

fn get_result(vars : &[f64; NUM_COUNT], action: &ActionTypes) -> Result<f64, String> {
    match action {
        ActionTypes::ADD => Ok(vars[0] + vars[1]),
        ActionTypes::SUBTRACT => Ok(vars[0] - vars[1]),
        ActionTypes::MULTIPLY => Ok(vars[0] * vars[1]),
        ActionTypes::DIVIDE => {
            if vars[1] == 0. {
                Err(String::from("divider is equal to zero"))
            } else {
                Ok(vars[0] / vars[1])
            }
        },
        ActionTypes::NONE => Err(String::from("unknown action"))
    }
}

fn main_loop() {
    cls();
    println!("Enter numbers");
    let nums = handle_var_input();
    println!("Enter action sign");
    let action = handle_action_input();
    let res = get_result(&nums, &action);
    match res {
        Ok(result) => println!("Your result: {}", result),
        Err(error) => println!("error: {}", error)
    }
    io::stdin().read_line(&mut String::new()).expect("qwer");
    io::stdin().read_line(&mut String::new()).expect("qwer");
}

fn main_menu() {
    let mut input_buffer :[u8; BUFFER_SIZE];
    loop {
        println!("Press Enter to solve example");
        println!("Press q to exit");
        input_buffer = [0; BUFFER_SIZE];
        io::stdin()
            .read_exact(&mut input_buffer)
            .expect("cant read string");
        match input_buffer[0] as char {
            'Q' | 'q' => return,
            _ => main_loop()
        }
    }
}

fn main() {
    main_menu();
}
