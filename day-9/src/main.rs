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

fn get_points(matrix: &Vec<Vec<u32>>) -> Vec<Point>{
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
    
    println!("sum: {}",&sum);

    points
}

fn part1(){
    let data = read_file("./input.txt");
    println!("{}",data.len());
    let mut matrix: Vec<Vec<u32>> = vec![];

    for row in data{
        let arr: Vec<u32> = row.split("").filter(|e| e.len()==1).map(|e| e.parse::<u32>().unwrap()).collect();
        matrix.push(arr);
    }
    
    let _points: Vec<Point> = get_points(&matrix);
}

fn part2(){
    let data = read_file("./input.txt");
    let mut matrix: Vec<Vec<u32>> = vec![];

    for row in data{
        let arr: Vec<u32> = row.split("").filter(|e| e.len()==1).map(|e| e.parse::<u32>().unwrap()).collect();
        matrix.push(arr);
    }

    let points: Vec<Point> = get_points(&matrix);

    let mut sums: Vec<u32> = vec![];
    for point in points{
        let mut visited_points: Vec<String> = vec![];
        let mut sum: u32 = 0u32;
        check_recursiv(point.x,point.y,&matrix,&mut visited_points,&mut sum);
        sums.push(sum);
    }

    sums.sort_by(|a, b| b.cmp(a));
    println!("{}",sums[0]*sums[1]*sums[2]);
}

fn check_recursiv(x:usize ,y:usize ,matrix: &Vec<Vec<u32>>, visited_points: &mut Vec<String>, sum: &mut u32){
    let mut above:u32 = 9;
    if y > 0{
        let ab_string = format!("{}-{}",y-1,x);
        above = if visited_points.contains(&ab_string) {9} else {matrix[y-1usize][x]};
        visited_points.push(ab_string);
    }
    if above != 9{
        *sum+=1;
        check_recursiv(x,y-1,&matrix,visited_points,sum);
    }

    let mut bellow:u32 = 9;
    if y < matrix.len()-1{
        let be_string = format!("{}-{}",y+1,x);
        bellow = if visited_points.contains(&be_string) {9} else {matrix[y+1][x]};
        visited_points.push(be_string);
    }
    if bellow != 9{
        *sum+=1;
        check_recursiv(x,y+1,&matrix,visited_points,sum);
    }

    let mut left:u32 = 9;
    if x > 0{
        let le_string = format!("{}-{}",y,x-1);
        left = if visited_points.contains(&le_string) {9} else {matrix[y][x-1]};
        visited_points.push(le_string);
    }
    
    if left != 9{
        *sum+=1;
        check_recursiv(x-1,y,&matrix,visited_points,sum);
    }

    let mut right:u32 = 9;
    if x < matrix[y].len()-1{
        let ri_string = format!("{}-{}",y,x+1);
        right = if visited_points.contains(&ri_string) {9} else {matrix[y][x+1]};
        visited_points.push(ri_string);
    }

    if right != 9{
        *sum+=1;
        check_recursiv(x+1,y,&matrix,visited_points,sum);
    }
}



fn main() {
    part2();
}
