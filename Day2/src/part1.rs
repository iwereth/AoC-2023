use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;


fn main(){

    let f = File::open("Day2/inputs/input").unwrap();
    let reader = BufReader::new(f);

    let regex_id = Regex::new(r"(?m)^Game\s*([0-9]+):").unwrap();
    let regex_count = Regex::new( r"([0-9]+)").unwrap();

    let mut sum: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let cap_id = regex_id.captures(&line).unwrap();
        let id = cap_id.get(1).unwrap().as_str().parse::<usize>().unwrap();

        let start = line.chars().position(|c| c == ':').unwrap();

        let line = &line[start+1 ..];
        let sets = line.split(";");

        let mut possible : bool = true;

        for set in sets {
            let mut red : usize = 0;
            let mut blue : usize = 0;
            let mut green : usize = 0;

            for obs in set.split(",").map(|x| x.trim()) {
                let cap_count = regex_count.captures(obs)
                                    .unwrap().get(1).unwrap().as_str()
                                    .parse::<usize>().unwrap();
                if obs.contains("red") {
                    red = red + cap_count;
                }
                else if obs.contains("blue") {
                    blue = blue + cap_count;
                }
                else if obs.contains("green") {
                    green = green + cap_count;
                }
                else {
                    panic!("What the hell?");
                }
            }

            if red > 12 || green > 13 || blue > 14 {
                possible = false;
                break;
            }
        }

        if possible {
            sum = sum + id;
        }
    }

    println!("{}", sum)
}