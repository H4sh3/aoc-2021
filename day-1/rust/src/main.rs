use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn part1() -> Result<(), Error> {
    let measures = read(File::open("../input.txt")?)?;

    let mut cnt = 0;
    let mut prev = -1;
    for m in measures {
        if prev == -1 {
            prev = m;
            continue;
        }

        if m > prev {
            cnt += 1;
        }
        prev = m;
    }

    println!("Part 1: {}", cnt);
    Ok(())
}

fn part2() -> Result<(), Error> {
    let measures = read(File::open("input.txt")?)?;

    let mut window: Vec<i64> = Vec::new();
    for i in 0..measures.len() - 2 {
        let mut sum: i64 = 0;
        sum += measures[i];
        sum += measures[i + 1];
        sum += measures[i + 2];
        window.push(sum);
    }

    let mut cnt = 0;
    let mut prev = -1;
    for m in window {
        if prev == -1 {
            prev = m;
            continue;
        }

        if m > prev {
            cnt += 1;
        }
        prev = m;
    }

    println!("Part 2: {}", cnt);
    Ok(())
}

fn main() -> Result<(), Error> {
    let _res = part1();
    let _res = part2();

    Ok(())
}
