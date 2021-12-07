use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(values: &Vec<String>) -> io::Result<()> {
    let mut x:i32 = 0;
    let mut y:i32 = 0;

    for c in values {
        let s = c.split(" ");
        let vec = s.collect::<Vec<&str>>();
        let command: &str = vec[0];
        let value: i32 = vec[1].parse::<i32>().unwrap();
        
        if command == "forward"{
            x+=value;
        }
        if command == "up"{
            y-=value;
        }
        if command == "down"{
            y+=value;
        }
    }
    println!("Part1 x:{} y:{} res:{}",x,y,x*y);
    Ok(())
}


fn part2(values: &Vec<String>) {
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut aim:i32 = 0;

    for c in values {
        let s = c.split(" ");
        let vec = s.collect::<Vec<&str>>();
        let command: &str = vec[0];
        let value: i32 = vec[1].parse::<i32>().unwrap();
        
        if command == "forward"{
            x+=value;
            y+=aim*value;
        }
        if command == "up"{
            aim-=value;
        }
        if command == "down"{
            aim+=value;
        }
    }
    println!("Part2 x:{} y:{} res:{}",x,y,x*y);
}

fn read_file(path:&str) -> Vec<String>{
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let values: Vec<String> = reader.lines().
        map(|item| item.unwrap().parse::<String>()).
        map(|item| item.unwrap()).
        collect();
    values
}

fn main() -> io::Result<()> {
    let values:Vec<String> = read_file("../input.txt");

    let _x = part1(&values);
    let _x = part2(&values);
    Ok(())
}
