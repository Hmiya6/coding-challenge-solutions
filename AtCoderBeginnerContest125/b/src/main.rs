// AtCoder Beginner Contest 125
// B - Resale
// https://atcoder.jp/contests/abc125/tasks/abc125_b

/*
fn read_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn read_vec() -> Vec<usize> {
    let s = read_string();
    s.trim().split_whitespace()
        .map(|e| e.parse().unwrap()).collect()
}
*/

fn main() {
    // input data
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let N = s.trim().parse::<usize>().unwrap();
    
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let V = s.trim().split_whitespace()
        .map(|e| e.parse().unwrap()).collect::<Vec<usize>>();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let C = s.trim().split_whitespace()
        .map(|e| e.parse().unwrap()).collect::<Vec<usize>>();

    let mut profits: Vec<isize> = Vec::new();
    for i in 0..N {
        let p = V[i] as isize - C[i] as isize;
        profits.push(p);
    }

    // calculate max profit.
    // reverse sort profits vec and add each of profit (profit > 0)
    profits.sort();
    profits.reverse();
    let mut max_profit = 0;
    for p in profits {
        if p <= 0 {
            break;
        }
        max_profit += p;
    }

    println!("{}", max_profit);
}
