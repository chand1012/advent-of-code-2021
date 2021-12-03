use std::io;


fn main() {
    // part 1
    let mut input = String::new();

    // vector of ints
    let mut v: Vec<Vec<char>> = Vec::new();

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

        // split the string into a char slice
        let c: Vec<char> = input.chars().collect();

        // push the char slice into the vector
        v.push(c);

        input.clear();
    }

    let mut one_counts: Vec<i32> = Vec::new();

    let row_length = v[0].len();

    for i in 0..row_length {
        let mut one_count = 0;

        for j in 0..v.len() {
            if v[j][i] == '1' {
                one_count += 1;
            }
        }

        one_counts.push(one_count);
    }

    let mut gamma_rate_string: String = String::new();
    let mut epsilon_string: String = String::new();

    for i in 0..row_length {
        // convert v's size to i32
        let zero_count = (v.len() as i32) - one_counts[i];
        let one_count = one_counts[i];

        if one_count > zero_count {
            gamma_rate_string.push_str("1");
            epsilon_string.push_str("0");
        } else {
            gamma_rate_string.push_str("0");
            epsilon_string.push_str("1");
        }
    }

    println!("{}", gamma_rate_string);
    println!("{}", epsilon_string);
    println!("");

    // convert the binary strings to a decimal number
    let gamma_rate = u32::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_string, 2).unwrap();

    // print the result
    println!("{}", gamma_rate); 
    println!("{}", epsilon);
    println!("{}", gamma_rate * epsilon);

    // end part 1
}
