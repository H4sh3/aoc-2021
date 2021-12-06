use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn read_file(path:&str) -> Vec<String>{
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let values: Vec<String> = reader.lines().
        map(|item| item.unwrap().parse::<String>()).
        map(|item| item.unwrap()).
        collect();
    values
}

fn filter_by_bit_pos(numbers: &Vec<String>, pos:usize) -> Vec<String>{
    let mut most_common = '1';

    let mut zeros = 0;
    let mut ones = 0;
    for n in numbers{
        if n.chars().nth(pos).unwrap() == '0'{
            zeros += 1;
        }else{
            ones += 1;
        }
        
    }
    if zeros > ones {
        most_common = '0';
    }

    numbers.iter().cloned().filter(|e| e.chars().nth(pos).unwrap() == most_common).collect()
}

fn filter_by_bit_pos_des(numbers: &Vec<String>, pos:usize) -> Vec<String>{
    let mut most_common = '0';

    let mut zeros = 0;
    let mut ones = 0;
    for n in numbers{
        if n.chars().nth(pos).unwrap() == '0'{
            zeros += 1;
        }else{
            ones += 1;
        }
    }

    // keep fewer
    if ones < zeros {
        most_common = '1';
    }

    numbers.iter().cloned().filter(|e| e.chars().nth(pos).unwrap() == most_common).collect()
}

fn part2(numbers: Vec<String>) {
    let mut x = filter_by_bit_pos(&numbers,0);
    for i in 1..12{
        x = filter_by_bit_pos(&x,i);
        println!("{}",x.len());
    }

    let oxygen_rating = isize::from_str_radix(&x[0][..] , 2).unwrap();
    println!("Oxygen rating {}",oxygen_rating);

    let mut y = filter_by_bit_pos_des(&numbers,0);
    for i in 1..9{
        y = filter_by_bit_pos_des(&y,i);
        println!("check {}",y.len());
        println!("{}",y.len());
    }
    
    let c02_rating = isize::from_str_radix(&y[0][..] , 2).unwrap();
    println!("Co2 scrub {}",c02_rating);
    
    println!("Result {}",oxygen_rating*c02_rating);
}

fn part1(numbers: Vec<String>) {
    let rows = numbers[0].len();
    let mut g_bytes: String = "".to_owned();
    let mut e_rate_bytes: String = "".to_owned();
    for i in 0..rows{
        let mut zeros = 0;
        let mut ones = 0;
        for n in numbers.iter(){
            if n.chars().nth(i).unwrap() == '0'{
                zeros += 1
            }else{
                ones += 1
            }
        }
        
        if zeros > ones {
            g_bytes.push_str("0");
            e_rate_bytes.push_str("1");
        }else{
            g_bytes.push_str("1");
            e_rate_bytes.push_str("0");
        }
    }

    let gamma_rate = isize::from_str_radix(&g_bytes[..] , 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&e_rate_bytes[..] , 2).unwrap();
    println!("{}",gamma_rate*epsilon_rate);
}

fn main() {
    let numbers: Vec<String> = read_file("../input.txt");
    //part1(numbers);
    part2(numbers);
}
