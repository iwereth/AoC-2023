#![feature(btree_cursors)]
#![feature(iter_next_chunk)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Bound;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn start_unmapped_end(map: &BTreeMap<i64, (i64, i64)>, start: i64, end: i64) -> (i64, Option<(i64, i64)>) {
    let kv = map.lower_bound(Bound::Included(&start)).key_value();

    if let None = kv {
        return (end, None);
    }

    else {
        let kv = kv.unwrap();

        if *kv.0 > end {
            return (end, None);
        }

        else { 
            return (kv.0 - 1, Some((*kv.0, end)));
        }
    }
}

fn start_mapped_end(map: &BTreeMap<i64, (i64, i64)>, start: i64, end: i64) -> (i64, Option<(i64, i64)>) {
    let kv = map.upper_bound(Bound::Included(& start)).key_value().unwrap();

    if kv.0 + kv.1.1 - 1 >= end {
        return (end, None);
    }
    else {
        return (kv.0 + kv.1.1 - 1, Some((kv.0 + kv.1.1, end)));
    }
}


// verify returned start, end ranges are valid
fn find_next_range(map : &BTreeMap<i64, (i64, i64)>, start: i64, end: i64) -> (i64, i64, Option<(i64, i64)>) {

    let kv = map.upper_bound(Bound::Included(&start)).key_value();

    // if start has no applicable range for now
    if kv.is_none(){
        let (end, rem) = start_unmapped_end(map, start, end);
        return (start, end, rem);
    }

    // well we need more checks
    else {
        // check if source of this map is not overlapping with the range we are concerned with
        let kv = kv.unwrap();
        if kv.0 + kv.1.1 - 1  < start {
            let (end, rem) = start_unmapped_end(map, start, end);
            return (start, end, rem);
        }
        else {
            let (end, rem) = start_mapped_end(map, start, end);
            let shift = kv.1.0 - kv.0;
            return (shift + start, end + shift, rem);
        }
    }
}

fn main() {

    let file = File::open("Day5/inputs/input").unwrap();
    let mut reader = BufReader::new(file);

    let mut string = String::new();
    if reader.read_to_string(&mut string).is_err() {
        panic!("Something went wrong while reading the file");
    }

    let mut splits = string.split("\n\n");

    let seeds_string = splits.next().unwrap().split(':').skip(1).collect::<String>();
    let mut seeds_split = seeds_string.split_whitespace();

    let mut t_ranges : Vec<(i64,i64)> = Vec::new();

    loop {

        let next_two = seeds_split.next_chunk::<2>();
        if let Err(_) = next_two {
            break;
        }
        else {
            t_ranges.push(<[i64;2]>::try_from(next_two.unwrap().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()).unwrap().into());
        }

    }

    let mut from_ranges : BTreeSet<(i64, i64)> = BTreeSet::new();
    for range in t_ranges {
        from_ranges.insert((range.0, range.0 + range.1 - 1));
    }

    let mut maps : Vec<BTreeMap<i64, (i64, i64)>> = Vec::new();

    for split in splits {
        let mut map : BTreeMap<i64, (i64, i64)> = BTreeMap::new();
        let inner_split = split.split('\n');
        for line in inner_split.skip(1) { 
            let [dest, src, len, ..] = <[&str;3]>::try_from(line.split_whitespace().collect::<Vec<&str>>()).unwrap();
            map.insert(
                src.parse().unwrap(), (dest.parse().unwrap(), len.parse().unwrap())
            );
        }

        maps.push(map);
    }

    for map in maps { 
        let mut to_ranges : BTreeSet<(i64, i64)> = BTreeSet::new();

        for range in from_ranges {
            let mut start = range.0;
            let mut end = range.1;

            loop {
                let (next_start, next_end, rem) = find_next_range(&map, start, end);
                to_ranges.insert((next_start, next_end));
                if rem.is_none() {
                    break;
                }
                let rem = rem.unwrap();
                start = rem.0;
                end = rem.1;
            }
        }

        from_ranges = to_ranges;

    }

    println!("{}", from_ranges.first().unwrap().0);
    
}