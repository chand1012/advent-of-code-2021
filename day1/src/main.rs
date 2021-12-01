use std::io;

fn main() {
    // get stdin
    let mut input = String::new();

    // vector of ints
    let mut v: Vec<i32> = Vec::new();

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

        // convert to int
        let n: i32 = input.parse().expect("Failed to parse input");

        // push to vector
        v.push(n);
        input = String::new();
    }

    // save a count
    let mut count: i32 = 0;

    // loop through vector
    // to the length minus one
    for i in 0..v.len() - 1 {
        // if the current element is less than the next
        if v[i] < v[i+1] {
            count += 1;
        }
    }

    println!("{}", count)

}
