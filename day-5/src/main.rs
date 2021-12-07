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
        let start = start_end[0].split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let end = start_end[1].split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let x1 = start[0];
        let y1 = start[1];
        let x2 = end[0];
        let y2 = end[1];
        if x1 == x2{
             // x1 == x2 veritcal
            let x = x1;
            if y1 < y2{
                for y in y1..y2+1{
                    field[y][x] += 1;
                }
            }else{
                for y in y2..y1+1{
                    field[y][x] += 1;
                }
            }
            continue;
        }

        if y1 == y2{
            let y = y1;
            if x1 < x2{
                for x in x1..x2+1{
                    field[y][x] += 1;
                }
            }else{
                for x in x2..x1+1{
                    field[y][x] += 1;
                }   
            }
            continue;
        }

        if x2 > x1{
            if y2 > y1{
                // + +
                for i in 0..x2-x1+1{
                    field[y1+i][x1+i] += 1;
                }
            }else{
                // - +
                for i in 0..x2-x1+1{
                    field[y1-i][x1+i] += 1;
                }
            }
            continue;
        }else{
            if y2 > y1{
                // + -
                for i in 0..x1-x2+1{
                    field[y1+i][x1-i] += 1;
                }
            }else{
                // - -
                for i in 0..x1-x2+1{
                    field[y1-i][x1-i] += 1;
                }
            }
            continue;
        }

    }

    // print_field(&field);
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
