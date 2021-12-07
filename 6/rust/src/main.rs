use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn read_file(path:&str) -> Vec<String>{
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let values: Vec<String> = reader.lines().
        map(|item| item.unwrap().parse::<String>()).
        map(|item| item.unwrap()).
        filter(|e| e.len() >= 0).
        collect();
    values
}

fn part1(){
    let data = read_file("./input.txt");
    let mut list: Vec<u32> = data[0].split(",").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    println!("{}",list.len());

    let days = 80;
    for i in 0..days{
        for j in 0..list.len(){
            if list[j] == 0{
                list[j] = 6;
                list.push(8);
            }else{
                list[j] -= 1;
            }
        }
    }

    println!("{}",&list.len());
}

fn part2(){
    let data = read_file("./input.txt");
    let mut list: Vec<u64> = data[0].split(",").map(|v| v.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut day_ray: Vec<u64> = [0,0,0,0,0,0,0,0,0].to_vec();

    for value in list{
        day_ray[value as usize] += 1;
    }

    let days = 256;
    for _ in 0..days{
        let mut tmp_day_ray: Vec<u64> = [0,0,0,0,0,0,0,0,0].to_vec();

        for i in 0..9{
            if i == 0{
                tmp_day_ray[8] = day_ray[0];
                continue;
            }
            if i == 7{
                tmp_day_ray[6] = day_ray[0]+day_ray[7];
                continue;
            }

            tmp_day_ray[i-1] = day_ray[i];
        }
        day_ray = tmp_day_ray;
        println!("{:?}",day_ray);
    }

    let mut sum:u64 = 0;
    for cnt in &day_ray{
        println!("{}",cnt);
        sum+=cnt;
    }

    println!("sum {}",sum);
}


fn main() {
    part2();
}
