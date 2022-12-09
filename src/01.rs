use std::{fs::File, io::Read};

fn main() {
    // ------ PART ONE --------
    //
    let filename = "input.txt";
    let file_result = File::open(filename);
    let mut file = match file_result {
        Ok(file)  => file,
        Err(error)  => panic!("{}", error),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(error)  => panic!("{}", error),
    }

    let mut elf : usize  = 0;
    let mut calories :Vec<i32> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            elf += 1
        } else {
            // add mnissing vec element
            if calories.len() == elf {
                calories.push(0);
            }

            let c = match line.parse::<u32>() {
                Ok(result) => result,
                Err(error)  => panic!("bitch you got strings in your numbies {}", error),
            };

            calories[elf] += c as i32;
        }
    }


    let mut max = 0;
    for c in calories {
        if c > max {
            max = c
        }
    }
    println!("Max Calories is  {}", max);

    // ------ PART TWO --------
    calories.sort();
    println!("top{}", top_n(&calories, 1)+top_n(&calories, 2)+top_n(&calories,  3));
}

fn top_n(vector : &Vec<i32>, n: u32)-> i32 {
    return vector[vector.len() -(n as usize)];
}
