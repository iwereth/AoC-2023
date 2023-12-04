use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn check_position_is_symbol(line: &Vec<char>, pos: usize) -> bool {

    if !line[pos].is_digit(10) && line[pos] != '.' {
        return true;
    }
    return false;
}

fn check_positions(line: &Vec<char>, pos : usize) -> bool{

    if pos > 0 {
        if check_position_is_symbol(line, pos - 1) {
            return true;
        }
    }

    if check_position_is_symbol(line, pos) {
        return true;
    }

    if pos < line.len() - 1 {
        if check_position_is_symbol(line, pos + 1) {
            return true;
        }
    }
    return false;

}

fn main() {
    let f = File::open("Day3/inputs/input").unwrap();
    let reader = BufReader::new(f);

    let mut line_iter = reader.lines().into_iter();

    let mut line1 : Option<Vec<char>> = None;
    let mut line2 : Option<Vec<char>> = Some(line_iter.next().unwrap().unwrap().chars().collect());
    let mut line3 : Option<Vec<char>> = Some(line_iter.next().unwrap().unwrap().chars().collect());

    let mut sum : usize = 0;

    loop {

        if line2.is_none() {
            break;
        }

        let mut start : Option<usize> = None;
        let mut end : Option<usize> = None;
        let mut is_part : bool = false;

        for pos in 0 .. line2.as_ref().unwrap().len() {

            let c = line2.as_ref().unwrap()[pos];

            if c.is_digit(10) {

                if let None = start {
                    start = Some(pos);
                }

                end = Some(pos);

                if line1.is_some() {
                    if check_positions(&line1.as_ref().unwrap(), pos) {
                        is_part = true;
                    }
                }

                if check_positions(&line2.as_ref().unwrap(), pos) {
                    is_part = true;
                }

                if line3.is_some() { 
                    if check_positions(&line3.as_ref().unwrap(), pos) { 
                        is_part = true;
                    }
                }
            }

            else {

                if start.is_some() {

                    if is_part {

                        let num_string = line2.as_ref().unwrap()
                                                .iter()
                                                .skip(start.unwrap()).take(end.unwrap() - start.unwrap() + 1)
                                                .fold(String::new(), |mut x, y| {x.push(*y); x});

                        let num = num_string.parse::<usize>().unwrap();
                        println!("{}", num);
                        sum = sum + num;

                    }

                }

                start = None;
                end = None;
                is_part = false;

            }
        }

        if start.is_some() {

            if is_part {

                let num_string = line2.as_ref().unwrap()
                                        .iter()
                                        .skip(start.unwrap()).take(end.unwrap() - start.unwrap() + 1)
                                        .fold(String::new(), |mut x, y| {x.push(*y); x});

                let num = num_string.parse::<usize>().unwrap();
                println!("{}", num);
                sum = sum + num;

            }

        }

        line1 = line2;
        line2 = line3;
        
        let t = line_iter.next();

        if t.is_some() {
            line3 = Some(t.unwrap().unwrap().chars().collect());
        }
        else {
            line3 = None;
        }

    }

    println!("{}", sum);

}