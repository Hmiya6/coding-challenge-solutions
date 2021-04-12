use std::io::Read;


/*
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}
*/

fn main() {
    let mut graph: Vec<String> = Vec::new();
    
    // input
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let height: usize = iter.next().unwrap().parse().unwrap();
    let width: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap() - 1;
    let y: usize = iter.next().unwrap().parse().unwrap() - 1;
    for _ in 0..height {
        let tmp = iter.next().unwrap();
        graph.push(String::from(tmp));
    }

    let horizontal_axis = graph[y].as_str();
    let mut vertical_axis = String::new();
    for i in 0..height {
        vertical_axis.push(graph[i][x]);
    }


}
