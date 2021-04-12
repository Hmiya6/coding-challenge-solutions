// AtCoder Beginner Contest 125
// D - Flipping Signs
// https://atcoder.jp/contests/abc125/tasks/abc125_d

// THIS CODE IS WA
// THIS CODE IS WA
// THIS CODE IS WA



fn read_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn read_num() -> usize {
    let s = read_string();
    s.trim().parse().unwrap()
}

fn read_vec() -> Vec<isize> {
    let s = read_string();
    s.trim().split_whitespace()
        .map(|e| e.parse().unwrap()).collect()
}


fn main() {
    let N = read_num();
    let mut A = read_vec();
    
    for i in 1..A.len() {
        println!("{:?}", A);
        if (A[i] + A[i-1]) < 0 {
            A[i] = -A[i];
            A[i-1] = -A[i-1];
        }
    }

    let sum: isize = A.iter().sum();
    println!("{}", sum);
}
