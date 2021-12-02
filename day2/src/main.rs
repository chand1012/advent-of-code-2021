use std::io;

struct Command {
    direction: String,
    distance: i32,
}

fn main() {
    let mut input = String::new();

    // vector of ints
    let mut v: Vec<Command> = Vec::new();

    // loop until stdin is done
    loop {
        // read line from stdin
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // trim newline
        input = input.trim().to_string();

        // if empty, break
        if input.is_empty() {
            break;
        }

        let mut split = input.split_whitespace();

        // direction
        let direction = split.next().unwrap().to_string();
        
        // distance
        let distance = split.next().unwrap().parse::<i32>().unwrap();

        // create command
        let command = Command {
            direction,
            distance,
        };

        // add command to vector
        v.push(command);

        input.clear();
    }

    // part 1
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for command in &v {
        // switch statement
        match command.direction.as_str() {
            "forward" => horizontal += command.distance,
            "down" => depth += command.distance,
            "up" => depth -= command.distance,
            _ => panic!("Invalid direction"),
        }
    }

    println!("horizontal: {}", horizontal);
    println!("depth: {}", depth);
    println!("multiplied: {}", horizontal * depth);
    // end part 1
    // part 2

    // print separator
    println!("");

    horizontal = 0;
    depth = 0;
    let mut aim: i32 = 0;

    for command in &v {
        // switch statement
        match command.direction.as_str() {
            "forward" => horizontal += command.distance,
            "down" => aim += command.distance,
            "up" => aim -= command.distance,
            _ => panic!("Invalid direction"),
        }

        if command.direction == "forward" {
            depth += aim * command.distance;
        }

    }

    println!("horizontal: {}", horizontal);
    println!("depth: {}", depth);
    println!("aim: {}", aim);
    println!("multiplied: {}", horizontal * depth);

    // end part 2
}
