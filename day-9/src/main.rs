use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_file(path:&str) -> Vec<String>{
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let values: Vec<String> = reader.lines().
        map(|item| item.unwrap().parse::<String>()).
        map(|item| item.unwrap()).
        collect();
    values
}

struct Point{
    x:usize,
    y:usize
}

struct MatrixAndPoints{
    points:Vec<Point>,
    matrix: Vec<Vec<u32>>
}

fn part1() -> MatrixAndPoints{
    let data = read_file("./input.txt");
    let mut matrix: Vec<Vec<u32>> = vec![];

    for row in data{
        let arr: Vec<u32> = row.split("").filter(|e| e.len()==1).map(|e| e.parse::<u32>().unwrap()).collect();
        matrix.push(arr);
    }
    
    let mut points:Vec<Point> = vec![];

    let mut sum = 0u32;
    for y in 0..matrix.len(){
        for x in 0..matrix[y].len(){
            let curr:u32 = matrix[y][x];

            let above:u32 = if y == 0 {9} else {matrix[y-1][x]};
            let bellow:u32 = if y == matrix.len()-1 {9} else {matrix[y+1][x]};
            let left:u32 = if x == 0 {9} else {matrix[y][x-1]};
            let right:u32 = if x == matrix[y].len()-1 {9} else {matrix[y][x+1]};

            if curr < above  && curr < bellow && curr < left && curr < right {
                sum+=matrix[y][x]+1;
                points.push(Point{x:x,y:y});
            }
        }
    }
    
    println!("Sum part-1: {}",&sum);

    MatrixAndPoints{points:points,matrix:matrix}
}

fn part2(){
    let output1: MatrixAndPoints = part1();

    let mut sums: Vec<u32> = vec![];
    for point in output1.points{
        let mut visited_points: Vec<String> = vec![];
        let mut sum: u32 = 0u32;
        check_recursiv(point.x,point.y,&output1.matrix,&mut visited_points,&mut sum);
        sums.push(sum);
    }

    sums.sort_by(|a, b| b.cmp(a));
    println!("Sum part-2: {}",sums[0]*sums[1]*sums[2]);
}

fn check_point(x:usize ,y:usize ,matrix: &Vec<Vec<u32>>, visited_points: &mut Vec<String>, sum: &mut u32){
        let point_ident = format!("{}-{}",y,x);
        let next_node = if visited_points.contains(&point_ident) {9} else {matrix[y][x]};
        visited_points.push(point_ident);
        if next_node != 9{
            *sum+=1;
            check_recursiv(x,y,&matrix,visited_points,sum);
        }
}

fn check_recursiv(x:usize ,y:usize ,matrix: &Vec<Vec<u32>>, visited_points: &mut Vec<String>, sum: &mut u32){
    if y > 0{
        check_point(x,y-1,&matrix,visited_points,sum);
    }
    if y < matrix.len()-1{
        check_point(x,y+1,&matrix,visited_points,sum);
    }
    if x > 0{
        check_point(x-1,y,&matrix,visited_points,sum);
    }
    if x < matrix[y].len()-1{
        check_point(x+1,y,&matrix,visited_points,sum);
    }
}



fn main() {
    part2();
}
