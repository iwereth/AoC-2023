use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let file = File::open("Day6/inputs/input").unwrap();
    let reader = BufReader::new(file);

    let times : Vec<i64>;
    let distances : Vec<i64>;

    let mut line_iter = reader.lines().into_iter();

    times = line_iter.next().unwrap().unwrap().split(':').skip(1).map(|x| x.trim().split_whitespace()).flatten().map(|x| x.parse::<i64>().unwrap()).collect();
    distances = line_iter.next().unwrap().unwrap().split(':').skip(1).map(|x| x.trim().split_whitespace()).flatten().map(|x| x.parse::<i64>().unwrap()).collect();

    let td_iter = times.into_iter().zip(distances.into_iter());

    let mut ans : i64 = 1;

    for (time, distance) in td_iter {

        let det : f64 ;

        if time*time - 4*distance < 0 {
            println!("Found an invalid determinant");
            continue;
        }

        det = f64::sqrt((time*time) as f64 - (4*distance) as f64);

        if time % 2 == 1 && det < 0.5 {
            println!("Does not have a valid integral solution");
        }

        let start_speed = ((time as f64 - det)/2.0).ceil() as i64;
        let start_time = time - start_speed;

        // This is the commented code for the time I wasted by misunderstanding the problem

        // let n = (start_time - start_speed + 1)/2;
        // let ap = (n*(n+1))/2;
        // let square_sum = (n*(n+1)*(2*n + 1))/6;

        // let mut ways : i64;

        // ways = 2*((start_speed*start_time)*n + (start_time - start_speed)*ap - square_sum);

        // if (start_speed + start_time) % 2 == 0 {
        //     let t = (start_speed + start_time) / 2;
        //     ways = ways + t*t;
        // }

        // println!("Det = {}, start_speed = {}, start_time = {}, Ways = {}", det, start_speed, start_time, ways);

        // ans = ans * ways;

        let mut ways = start_time - start_speed + 1;
        if start_time * start_speed == distance {
            ways = ways - 2;
        }

        ans = ans * ways;
    }

    println!("{}", ans);
}