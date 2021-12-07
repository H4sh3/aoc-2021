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
    let lines = read_file("./input.txt");
    for line in &lines{
        println!("{}",line);
    }

    let mut field: Vec<Vec<u32>> = [].to_vec();

    let size = 1000;
    for _x in 0..size{
        let mut row:Vec<u32> = [].to_vec();
        for _y in 0..size{
            row.push(0);
        }
        field.push(row);
    }

    for line in &lines{
        let start_end = line.split(" -> ").collect::<Vec<&str>>();
        let start = start_end[0].split(",").map(|e| e.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let end = start_end[1].split(",").map(|e| e.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        if start[0] == end[0]{
             // x1 == x2 veritcal
            let x = start[0];
            if start[1] < end[1]{
                for y in start[1]..end[1]+1{
                    field[y as usize][x as usize] += 1;
                }
            }else{
                for y in end[1]..start[1]+1{
                    field[y as usize][x as usize] += 1;
                }   
            }
        }

        if start[1] == end[1]{
            let y = start[1];
            if start[0] < end[0]{
                println!("1");
                for x in start[0]..end[0]+1{
                    field[y as usize][x as usize] += 1;
                }
            }else{
                println!("1");
                for x in end[0]..start[0]+1{
                    field[y as usize][x as usize] += 1;
                }   
            }
        } 
    }
    print_field(&field);
    eval_field(&field);
}

fn eval_field(field:&Vec<Vec<u32>>){
    let mut cnt = 0;
    for row in field{
        for value in row{
            if value >= &2{
                cnt+=1
            }
        }
    }
    println!("{}",cnt);
}

fn print_field(field:&Vec<Vec<u32>>){
    for row in field{
        println!("{:?}",row)
    }
}
