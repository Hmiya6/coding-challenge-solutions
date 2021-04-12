
// AtCoder Beginner Contest 125
// A - Biscuit Generator
// https://atcoder.jp/contests/abc125/tasks/abc125_a

/*
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
*/


fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let input = s.trim().split_whitespace()
        .map(|e| e.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let (A, B, T) = (input[0], input[1], input[2]);
    
    // the machine produces B per A sec
    let mut counter = 0;
    let mut elasped_time = 0;
    
    // elasped_time += A;
    loop {
        elasped_time += A;
        if elasped_time > T+1/2 {
            break;
        }
        counter += B;
    }
    println!("{}", counter);
}
