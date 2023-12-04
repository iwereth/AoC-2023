use std::borrow::BorrowMut;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn amount_cards(prefix_sum: &Vec<usize>, start : usize, end : usize) -> usize {

    if start >= prefix_sum.len() {
        return 0;
    }

    if end >= prefix_sum.len() {
        return prefix_sum[start];
    }

    if start > end {
        return 0;
    }

    return prefix_sum[start] - prefix_sum[end];
}

fn main() {

    let f = File::open("Day4/inputs/input").unwrap();
    let reader = BufReader::new(f);

    // won_cards[i] = the cards won at draw of card i + 1
    let mut won_cards : Vec<usize> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        let mut splits = line.split_whitespace();
        
        let hs = HashSet::<usize>::from_iter(splits.borrow_mut().into_iter().skip(2).take(10).map(|x| x.parse::<usize>().unwrap()));

        let count = splits.into_iter().skip(1).map(|x| x.parse::<usize>().unwrap()).filter(|x| hs.contains(x)).count();

        won_cards.push(count);
    }

    // total_new_cards[i] = amount of  new cards introduced if we draw card i + 1
    // and exhaustively keep drawing new cards
    let mut total_new_cards : Vec<usize> = Vec::new();
    total_new_cards.resize(won_cards.len(), 0);

    // n = total cards
    // prefix_sum[i] = cumulative sum of the new cards introduced exhaustively at drawing of card n to card i
    // prefix_sum[i] - prefix_sum[j] = sum of cards won at drawing of card i + 1 to card j, j > i
    let mut prefix_sum : Vec<usize> = Vec::new();
    prefix_sum.resize(won_cards.len(), 0);

    let mut pos : usize = won_cards.len() - 1;
    loop {
        // one (the original card) + new cards introduced exhaustively
        total_new_cards[pos] = 1 + amount_cards(&prefix_sum, pos + 1, pos + 1 + won_cards[pos]);
        prefix_sum[pos] = total_new_cards[pos];
        if pos < won_cards.len() - 1 {
            prefix_sum[pos] = prefix_sum[pos] + prefix_sum[pos + 1];
        }
        if pos == 0 {
            break;
        }
        pos = pos - 1;
    }

    let res = total_new_cards.iter().fold(0, |x, y| x + y);

    println!("{}", res);
}