use std::borrow::BorrowMut;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let f = File::open("Day4/inputs/input").unwrap();
    let reader = BufReader::new(f);

    let mut res : usize = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        let mut splits = line.split_whitespace();
        
        let hs = HashSet::<usize>::from_iter(splits.borrow_mut().into_iter().skip(2).take(10).map(|x| x.parse::<usize>().unwrap()));

        let count = splits.into_iter().skip(1).map(|x| x.parse::<usize>().unwrap()).filter(|x| hs.contains(x)).count();

        if count > 0 {
            res = res + usize::pow(2, count as u32 - 1);
        }
    }
    println!("{}", res);
}