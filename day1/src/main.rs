use std::io;

fn main() {
    // start part 1
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

    println!("{}", count);
    // end part 1
    // start part 2

    // new vector of i32 slices
    let mut v2: Vec<&[i32]> = Vec::new();

    // loop through vector
    // jumping by three
    for i in 0..v.len() {
        if i < 3 {
            v2.push(&v[0..i]);
        } else if i > v.len() - 3 {
            v2.push(&v[i..v.len()]);
        } else {
            v2.push(&v[i..i+3]);
        }
    }

    count = 0;

    // loop through each group
    for i in 0..v2.len() - 1 {
        // get the sum of the group
        let current_sum: i32 = v2[i].iter().sum();
        // get the next sum of the group
        let next_sum: i32 = v2[i+1].iter().sum();
        
        // if the next sum is greater than the current sum
        if next_sum > current_sum {
            // increment count
            count += 1;
        }
    }

    println!("{}", count);

    // end part 2

}
