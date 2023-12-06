use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {

    let file = File::open("Day6/inputs/input").unwrap();
    let reader = BufReader::new(file);

    let time : i64;
    let distance : i64;

    let mut line_iter = reader.lines().into_iter();

    time     =   line_iter.next().unwrap().unwrap().split(':')
                .skip(1).map(|x| x.trim().split_whitespace()).flatten().
                fold(String::new(), |mut x, y| {x.push_str(y); x}).parse::<i64>().unwrap();

    distance =   line_iter.next().unwrap().unwrap().split(':')
                .skip(1).map(|x| x.trim().split_whitespace()).flatten().
                fold(String::new(), |mut x, y| {x.push_str(y); x}).parse::<i64>().unwrap();

    let det : f64 ;

    if time*time - 4*distance < 0 {
        println!("Found an invalid determinant");
        return;
    }

    det = f64::sqrt((time*time) as f64 - (4*distance) as f64);

    if time % 2 == 1 && det < 0.5 {
        println!("Does not have a valid integral solution");
        return;
    }

    let start_speed = ((time as f64 - det)/2.0).ceil() as i64;
    let start_time = time - start_speed;

    let mut ways = start_time - start_speed + 1;
    if start_time * start_speed == distance {
        ways = ways - 2;
    }

    println!("{}", ways);

}