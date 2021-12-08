use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn count_increasing(depths: &[u64], window_size: usize) -> u64 {
    let mut count: u64 = 0;

    for i in window_size..depths.len() {
        let prev_sum: u64 = depths[i-window_size..i].iter().sum();
        let curr_sum: u64 = depths[i-window_size+1..i+1].iter().sum();

        if curr_sum > prev_sum {
            count += 1;
        }
    }

    count
}


fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().take(2).collect();
    if args.len() < 2 {
        panic!("Please enter an input file");
    }

    let file_name = &args[1];
    let file = File::open(file_name).unwrap();

    let reader = BufReader::new(file);

    let lines_res: Result<Vec<String>, _> = reader.lines().collect();
    let lines = lines_res.unwrap();

    let depths_parsed_res: Result<Vec<u64>, _> = lines.iter().map(|line| str::parse::<u64>(line.trim())).collect();
    let depths = depths_parsed_res.unwrap();

    let increasing_1 = count_increasing(depths.as_slice(), 1);
    println!("window size 1: {}", increasing_1);

    let increasing_3 = count_increasing(depths.as_slice(), 3);
    println!("window size 3: {}", increasing_3);

    Ok(())
}
