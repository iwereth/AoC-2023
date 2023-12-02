#![allow(dead_code)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;
use std::collections::HashMap;

struct Node {
    id : usize,
    leaf: Option<usize>,
    transitions: Vec<Option<usize>>
}

impl Node {

    pub fn new(id: usize) -> Node {
        let mut ret = Node { leaf: None, transitions: Vec::new(), id: id};
        ret.transitions.resize(36, None);
        ret
    }

    pub fn char_to_idx(ch : char) -> Option<usize> {
        if ch.is_digit(10) { 
            return Some(26 + ch as usize - '0' as usize);  
        }
        else if ch.is_ascii_lowercase() {
            return Some(ch as usize - 'a' as usize);
        }
        else {
            return None;
        }
    }

    pub fn idx_to_char(idx: usize) -> Option<char> {

        let idx = idx as u8;
        if idx >= 26 && idx < 36 {
            Some((idx - 26 + '0' as u8).try_into().unwrap())
        }
        else if idx < 26 {
            Some((idx + 'a' as u8).try_into().unwrap())
        }
        else {
            None
        }
    }

    pub fn set_leaf(&mut self, b : usize) {
        self.leaf = Some(b);
    }

    pub fn set_transitions(&mut self, t : Vec<Option<usize>>){
        self.transitions = t;
    }

    pub fn set_transition(&mut self, target : char, val : usize) {

        let pos: usize = match Node::char_to_idx(target) {
            Some(x) => x,
            None => return ()
        };

        self.transitions[pos] = Some(val);
    }

    pub fn get_transition(&self, target: char) -> Option<usize> {
        let pos : usize = match Node::char_to_idx(target) {
            Some(x) => x,
            None => panic!("char_to_idx failed")
        };

        self.transitions[pos]
    }

}

struct StateMachine<'a> {
    nodes : Vec<Node>,
    failure_transitions: Vec<usize>,
    string_to_match : Option<String>,
    position : Option<usize>,
    dictionary: HashMap<&'a str, i32>,
    state: usize
}

fn print_tree(nodes: &Vec<Node>, node_id: usize, depth: usize){
    let marker = "--".repeat(depth);
    for (char_idx, next_node_id) in nodes[node_id].transitions.iter().enumerate().filter(|(_, x)| x.is_some()) {
        println!("{}>{:>3}({})", marker, next_node_id.unwrap(), Node::idx_to_char(char_idx).unwrap() );
        print_tree(nodes, next_node_id.unwrap(), depth + 1);
    }
}

impl<'a> StateMachine<'a> {

    pub fn new(dict : HashMap<&'a str, i32>) -> StateMachine<'a> {
        let mut nodes : Vec<Node> = Vec::new();
        nodes.push(Node::new(0));

        let mut state : usize ;

        // build up a trie
        for (word, target) in &dict {

            // start from root node
            state = 0;
            
            for c in word.chars() {

                // if there is no transition from current node 
                // through character c

                if let None = nodes[state].get_transition(c){

                    // create new state
                    let node = Node::new(nodes.len());
                    let id = nodes.len();

                    // register it 
                    nodes.push(node);

                    // set a transition to it on character c
                    nodes[state].set_transition(c, id);
                }

                // move to the new state
                state = nodes[state].get_transition(c).unwrap();

            }

            // set the final state as leaf
            nodes[state].set_leaf(*target as usize);
        }

        print_tree(&nodes, 0, 0);

        // set default root node transition
        for (_, elem) in nodes[0].transitions.iter_mut().enumerate() {
            if let None = *elem {
                *elem = Some(0);
            }
        }

        // calcuate failure transitions for all the nodes/states
        let mut failure_transitions : Vec<usize> = Vec::new();
        failure_transitions.resize(nodes.len(), 0);

        // used for breadth first traversal
        let mut deq : VecDeque<usize> = VecDeque::new();

        // for every state/node immediate chlid to the root
        for immediate_state in  nodes[0].transitions.iter().filter(|x| x.is_some()) {
            let immediate_state = immediate_state.unwrap();

            if immediate_state == 0 {
                continue;
            }

            // append to the traversal
            deq.push_front(immediate_state);
            
            // set the failure transition for these children
            // to the root node
            failure_transitions[immediate_state] = 0;
        }

        // until all the nodes are traversed
        while deq.len() != 0 {

            // count nodes in current level
            let nodes_in_level = deq.len();

            // for these nodes
            for _ in 0 .. nodes_in_level {
                
                // take one of the nodes in current level
                let curr_state = deq.pop_back().unwrap();
                
                // for all the nodes we can reach
                for (idx, imm_state) in nodes[curr_state].transitions.iter().enumerate().filter(|(_, x)| x.is_some()){

                    let imm_state = imm_state.unwrap();

                    let mut failure_state = curr_state;

                    // find the fail state
                    let imm_fail_state = loop {
                        failure_state = failure_transitions[failure_state];
                        let node : &Node = &nodes[failure_state];

                        if let Some(x) = node.transitions[idx] {
                            break x;
                        }
                    };

                    failure_transitions[imm_state] = imm_fail_state;

                    deq.push_front(imm_state);
                }

            }

        }

        StateMachine {   
            nodes: nodes, 
            string_to_match: None, 
            position: None, 
            failure_transitions: failure_transitions, 
            dictionary: dict, 
            state: 0 
        }
    }

    pub fn set_string(&mut self, to_match : String){
        self.string_to_match = Some(to_match);
        self.position = Some(0);
    }

    pub fn run(&mut self) -> Option<usize> {
        loop {

            if self.position.unwrap() == self.string_to_match.as_ref().unwrap().len() {
                return None;
            }

            let mut node = &self.nodes[self.state];
            
            let c = self.string_to_match.as_ref().unwrap().as_bytes()[self.position.unwrap()] as char;

            while let None = node.get_transition(c) {
                node = &self.nodes[self.failure_transitions[node.id]];
            }

            self.state = node.get_transition(c).unwrap();
            node = &self.nodes[self.state];
            self.position = Some(self.position.unwrap() + 1);

            if let Some(x) = node.leaf {
                return Some(x);
            }

        }
    }

}

fn main(){

    let dict_nums = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ]);

    let mut sm = StateMachine::new(dict_nums);

    let f = File::open("Day1/inputs/input").unwrap();
    let reader = BufReader::new(f);

    let mut sum : usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        
        sm.set_string(line);

        let mut first : Option<usize> = None;
        let mut second : Option<usize> = None;

        while let Some(num) = sm.run() {
            if first.is_none() {
                first = Some(num);
            }
            second = Some(num);
        }

        sum = sum + first.unwrap()*10 + second.unwrap();
    }

    println!("{}", sum);

}
