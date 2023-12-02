use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;
use std::cmp::max;

fn main(){

    let f = File::open("Day2/inputs/input").unwrap();
    let reader = BufReader::new(f);

    let regex_count = Regex::new( r"([0-9]+)").unwrap();

    let mut sum: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let start = line.chars().position(|c| c == ':').unwrap();

        let line = &line[start+1 ..];
        let sets = line.split(";");

        let mut max_red : usize = usize::MIN;
        let mut max_blue : usize = usize::MIN;
        let mut max_green : usize = usize::MIN;

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

            max_red = max(max_red, red);
            max_blue = max(max_blue, blue);
            max_green = max(max_green, green);

        }

        sum = sum + max_red*max_blue*max_green;
    }

    println!("{}", sum)
}