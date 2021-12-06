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
    let data = read_file("../input.txt");

    let numbers: Vec<u32> = data[0].split(',')
    .filter_map(|e| e.parse::<u32>().ok())
    .collect();

    let mut boards:Vec<Vec<Vec<u32>>> = [].to_vec();
    let mut board: Vec<Vec<u32>> = [].to_vec();
    for i in 2..data.len(){
        if data[i].len() == 0{continue}
        let v: Vec<u32> = data[i].split(' ')
        .filter_map(|e| e.parse::<u32>().ok())
        .collect();
        
        board.push(v);

        if board.len() == 5{
            boards.push(board);
            board = [].to_vec();
        }
    }
    
    let mut used_numbers = [].to_vec();

    let num_boards = boards.len();
    let mut finished_boards: Vec<u32> = [].to_vec();

    let mut first:bool = true;

    for new_number in numbers{
        used_numbers.push(new_number);
        let mut board_index:u32 = 0;
        for board in boards.iter(){
            board_index+=1;
            let ver: bool = check_board_hori(&board,&used_numbers);
            let hor: bool = check_board_vert(&board,&used_numbers);
            if  ver||hor {
                if first{
                    println!("First board: {} res: {} \n",new_number,new_number*board_sum(&board,&used_numbers));
                    print_board(&board);
                    first = false
                }

                if finished_boards.contains(&board_index) {continue}
                finished_boards.push(board_index);
                if finished_boards.len() == num_boards{
                    println!("Last board: {} res: {}",new_number,new_number*board_sum(&board,&used_numbers));
                    print_board(&board);
                    return
                }
            }
        }
    }
}

fn board_sum(board:&Vec<Vec<u32>>,numbers: &Vec<u32>) -> u32{
    let mut sum = 0;
    for x in board{
        for y in x{
            if !numbers.contains(y){
                sum += y;
            }
        }
    }
    return sum;
}

fn print_board(board:&Vec<Vec<u32>>){
    for x in board{
        for y in x{
            print!("{} ",y);
        }
        print!("\n");
    }
}

fn check_board_hori(board:&Vec<Vec<u32>>,numbers:&Vec<u32>) -> bool{
    let mut bingo:bool = false;
    for row in board{
        let mut cnt:u32 = 0;
        for e in row{
            if numbers.contains(&e){
                cnt+=1
            }
        }
        if cnt == 5{
            bingo = true
        }
    }
    return bingo
}

fn check_board_vert(board:&Vec<Vec<u32>>,numbers:&Vec<u32>) -> bool{
    let mut bingo:bool = false;
    for row in 0..board.len(){
        let mut cnt:u32 = 0;
        for col in board{
            if numbers.contains(&col[row]){
                cnt+=1
            }
        }
        if cnt == 5{
            bingo = true
        }
    }
    return bingo
}
