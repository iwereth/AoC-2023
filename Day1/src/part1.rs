use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main(){

    let mut sum : u64 = 0;

    let f = File::open("Day1/inputs/input").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let mut first : Option<u64> = None;
        let mut second : Option<u64> = None;

        let input = line.unwrap();

        for c in input.chars() {

            if !c.is_digit(10) {
                continue;
            }

            if let None = first {
                first = Some(c.to_digit(10).unwrap() as u64);
            }

            second = Some(c.to_digit(10).unwrap() as u64);
        }

        sum = sum + first.unwrap() *10;
        sum = sum + second.unwrap() ;

    }

    println!("{:?}", sum);

}
