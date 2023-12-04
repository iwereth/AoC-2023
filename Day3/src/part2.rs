use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn find_num_span(line: &Vec<char>, pos: usize) -> (usize, usize) {
    let mut start: usize = pos;
    let mut end: usize = pos;

    while start > 0 && line[start - 1].is_digit(10) { 
        start = start - 1;
    }

    while end < line.len() - 1 && line[end + 1].is_digit(10) {
        end = end + 1;
    }

    (start, end)
}

fn parse_num(line: &Vec<char>, span: (Option<usize>, Option<usize>)) -> usize {

    let start = span.0.unwrap();
    let end = span.1.unwrap();

    let num_string =    line.iter().skip(start).take(end-start+1)
                                    .fold(String::new(), |mut x, y| { x.push(*y); x});

    return num_string.parse::<usize>().unwrap();
}

fn find_adj_nums_from_mid(line: &Vec<char>, pos: usize) -> (Option<usize>, Option<usize>) { 

    if line[pos].is_digit(10){
        let (start, end) = find_num_span(line, pos);

        return (
            Some(parse_num(line, (Some(start), Some(end)))),
            None
        );
    }

    let first : Option<usize>;
    let second : Option<usize>;

    if pos > 0 && line[pos - 1].is_digit(10) {
        let (start, end) = find_num_span(line, pos-1);
        first = Some(parse_num(line, (Some(start), Some(end))));
    }
    else {
        first = None;
    }

    if pos < line.len() - 1 && line[pos + 1].is_digit(10) {
        let (start, end) = find_num_span(line, pos + 1);
        second = Some(parse_num(line, (Some(start), Some(end))));
    }
    else {
        second = None;
    }

    (first, second)

}

fn main() {

    let f = File::open("Day3/inputs/input").unwrap();
    let reader = BufReader::new(f);

    let mut lines_iter = reader.lines().into_iter();

    let mut line1: Option<Vec<char>> = None;
    let mut line2: Option<Vec<char>> = Some(lines_iter.next().unwrap().unwrap().chars().collect());
    let mut line3: Option<Vec<char>> = Some(lines_iter.next().unwrap().unwrap().chars().collect());

    let mut sum = 0;

    loop { 

        if line2.is_none() {
            break;
        }

        for (pos, char) in line2.as_ref().unwrap().iter().enumerate() {

            if *char == '*' {

                let mut nums : Vec<usize> = Vec::new();

                for line in [line1.as_ref().unwrap(), line2.as_ref().unwrap(), line3.as_ref().unwrap()] {
                    let res = find_adj_nums_from_mid(line, pos);
                    if res.0.is_some(){
                        nums.push(res.0.unwrap());
                    }
                    if res.1.is_some(){
                        nums.push(res.1.unwrap());
                    }
                }

                if nums.len() == 2 {
                    sum = sum + nums[0] * nums[1];
                }
            }

        }

        line1 = line2;
        line2 = line3;
        
        let next = lines_iter.next();
        if next.is_some() {
            line3 = Some(next.unwrap().unwrap().chars().collect());
        }
        else {
            line3 = None;
        }

    }

    println!("{}", sum);

}