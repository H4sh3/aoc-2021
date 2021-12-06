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

fn main() {
    let numbers: Vec<String> = read_file("../input.txt");

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
