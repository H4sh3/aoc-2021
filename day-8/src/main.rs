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

fn part1() {
    let data = read_file("./input.txt");
    
    let mut len_counts:Vec<u32> = [].to_vec();
    for _y in 0..8{
        len_counts.push(0);
    }

    for row in data{
        let split:Vec<&str> = row.split(" | ").collect();

        let digits:Vec<&str> = split[1].split(" ").collect();
        for digit in digits{
            len_counts[digit.len()] += 1;
        }
    }

    let sum = len_counts[2] + len_counts[3] + len_counts[4] + len_counts[7];
    println!("sum part 1: {}",sum);
}

fn get_dict(input:&str) -> Vec<Vec<&str>>{
    let x: Vec<&str> = input.split(" ").collect();
    let mut all_split: Vec<Vec<&str>> = vec![];

    for y in &x{
        all_split.push(y.split("").filter(|e| e.len() == 1).collect());
    }

    let mut five_digits: Vec<Vec<&str>> = vec![];
    let mut six_digits: Vec<Vec<&str>> = vec![];

    for d in &all_split{
        if d.len() == 5 {
            five_digits.push(d.to_vec());
        }
        if d.len() == 6 {
            six_digits.push(d.to_vec());
        }
    }

    let index_1:usize = all_split.iter().position(|x| x.len() == 2).unwrap();
    let index_7:usize = all_split.iter().position(|x| x.len() == 3).unwrap();
    let index_4:usize = all_split.iter().position(|x| x.len() == 4).unwrap();
    let index_8:usize = all_split.iter().position(|x| x.len() == 7).unwrap();

    let one = &all_split[index_1];
    let seven = &all_split[index_7];
    let four = &all_split[index_4];
    let aight = &all_split[index_8];

    // find three
    let mut three: Vec<&str> = vec![];
    for digit in &five_digits{
        let mut similar = 0;
        for letter in digit{
            if seven.contains(letter){
                similar+=1
            }

        }

        if similar == 3 {
            three = digit.to_vec()
        }
    }
    // println!("three: {:?}",three);

    // find five
    let mut five: Vec<&str> = vec![];
    for digit in &five_digits{
        let mut similar = 0;
        for letter in digit{
            if four.contains(letter){
                similar+=1
            }
        }

        if similar == 3 && digit != &three{
            five = digit.to_vec()
        }
    }
    //println!("five: {:?}",five);

    // find two; last one with 5 digits
    let mut two: Vec<&str> = vec![];
    for digit in &five_digits{
        if digit != &five && digit != &three{
            two = digit.to_vec()
        }
    }
    //println!("two: {:?}",two);


    // find nine
    let mut nine: Vec<&str> = vec![];
    for digit in &six_digits{
        let mut similar = 0;
        for letter in digit{
            if three.contains(letter){
                similar+=1
            }
        }

        if similar == 5 {
            nine = digit.to_vec()
        }
    }
    //println!("nine: {:?}",nine);


    // find six
    let mut six: Vec<&str> = vec![];
    for digit in &six_digits{
        let mut similar = 0;
        for letter in digit{
            if five.contains(letter){
                similar+=1
            }
        }

        if similar == 5 && !compare(&digit, &nine){
            six = digit.to_vec()
        }
    }
    //println!("six: {:?}",six);

    // find zero; last one with 6 digits
    let mut zero: Vec<&str> = vec![];
    for digit in &six_digits{
        if digit != &nine && digit != &six{
            zero = digit.to_vec()
        }
    }
    // println!("zero: {:?}",zero);
    let result = vec![zero,one.to_vec(),two,three,four.to_vec(),five,six,seven.to_vec(),aight.to_vec(),nine];
    
    //println!("zero: {:?}",result);
    result
}

fn part2(){
    let data = read_file("./input.txt");
    let mut sum = 0;
    for row in data{
        let mut number: String = "".to_string();
        let split:Vec<&str> = row.split(" | ").collect();

        let dict: Vec<Vec<&str>> = get_dict(split[0]).to_vec();

        //println!("{:?}",dict);

        let digits :Vec<&str>= split[1].split(" ").collect();

        for digit in digits{
            let mut indx = 0;
            for entry in &dict{
                let arr:Vec<&str> = digit.split("").filter(|e|e.len()==1).collect();
                if compare(&entry,&arr){
                   //println!("{}",indx);
                   number+=&indx.to_string();
                   break;
                }
                indx+=1;
            }
        }
        // println!("{}",number);
        sum += number.parse::<u32>().unwrap();
    }
    println!("sum part 2: {}",sum);
}

fn compare(v1:&Vec<&str>, v2:&Vec<&str>) -> bool
{
    if v1.len() != v2.len() {return false}
    for e1 in v1{
        if !v2.contains(e1){
            return false;
        }
    }
    return true;
}

fn main() {
    part1();
    part2();
}


