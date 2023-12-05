#![feature(btree_cursors)]

use std::collections::BTreeMap;
use std::ops::Bound;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::cmp::min;

fn main() {

    let file = File::open("Day5/inputs/input").unwrap();
    let mut reader = BufReader::new(file);

    let mut string = String::new();
    if reader.read_to_string(&mut string).is_err() {
        panic!("Something went wrong while reading the file");
    }

    let mut splits = string.split("\n\n");

    let seeds_string = splits.next().unwrap().split(':').skip(1).collect::<String>();
    let seeds_split = seeds_string.split_whitespace();

    let mut seeds : Vec<usize> = Vec::new();

    for seed in seeds_split {
        seeds.push(seed.parse().unwrap());
    }

    let mut maps : Vec<BTreeMap<usize, (usize, usize)>> = Vec::new();

    for split in splits {
        let mut map : BTreeMap<usize, (usize, usize)> = BTreeMap::new();
        let inner_split = split.split('\n');
        for line in inner_split.skip(1) { 
            let [dest, src, len, ..] = <[&str;3]>::try_from(line.split_whitespace().collect::<Vec<&str>>()).unwrap();
            map.insert(
                src.parse().unwrap(), (dest.parse().unwrap(), len.parse().unwrap())
            );
        }

        maps.push(map);
    }

    let mut fin : usize = usize::MAX;

    for seed in seeds {
        let res = maps.iter().fold(seed,
            |so_far, map| {
                let kv = map.upper_bound(Bound::Included(&so_far)).key_value();
                if kv.is_none() {
                    return so_far;
                }
                let kv = kv.unwrap();
                if so_far > kv.0 + kv.1.1 - 1 {
                    so_far
                }
                else{
                    so_far - kv.0 + kv.1.0
                }
            });
        fin = min(fin, res);
    }

    println!("{}", fin);
    
}