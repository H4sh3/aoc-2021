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

fn main() {
    let mut data:Vec<u32> = read_file("./input.txt")[0].split(",").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut max: u32 = 0;
    for i in &data{
        if i > &max{
            max = *i;
        }
    }

    let mut pos_costs:Vec<u32> = vec![];
    for _ in 0..max{
        pos_costs.push(0);
    }

    for pos in 0..pos_costs.len(){
        for i in &data{
            let diff = if i > &(pos as u32) { i-pos as u32 } else { pos as u32 -i };
            pos_costs[pos] += (diff*(diff+1))/2;
        }   
    }

    let mut min = std::f32::INFINITY as u32;
    let mut min_indx: usize = 0;
    for pos in 0..pos_costs.len(){
        if pos_costs[pos] < min{
            min = pos_costs[pos];
            min_indx = pos;
        }
    }

    println!("Position {}",min_indx);
    println!("Costs {}",pos_costs[min_indx]);
}
