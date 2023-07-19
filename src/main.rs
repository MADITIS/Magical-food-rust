// use std::env;
use std::fs;

fn main() {
    let file_path = "./data.txt";
    println!("In file {}", file_path);
    // reading a file using fs module and function read_to_string
    let contents = fs::read_to_string(file_path).expect("Some Error happened");

    // unused warning because not "using" it. just assigning.
    let mut max: i32 = 0;

    let mut count: i32 = 0;

    let mut elf: i8 = 1;

    for i in contents.lines() {
        if i == "" {
            if count > max {
                println!("{count}");
                max = count;
                elf += 1;
            }
            count = 0;
        } else if i != "" {
            // println!("{i}");
            count += i.trim().parse::<i32>().expect("not a number");
        }
    }

    if count > max {
        max = count;
        elf += 1;
    }

    println!("elf {elf} carrying max {max}");
}
