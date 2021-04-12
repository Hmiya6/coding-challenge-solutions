// AtCoder Beginner Contest 125
// C - GCD on Blackboard
// https://atcoder.jp/contests/abc125/tasks/abc125_c


fn read_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn read_num() -> usize {
    let s = read_string();
    s.trim().parse().unwrap()
}

fn read_vec() -> Vec<usize> {
    let s = read_string();
    s.trim().split_whitespace()
        .map(|e| e.parse().unwrap()).collect()
}

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        return b;
    } else {
        return gcd(b, a%b);
    }
}

fn gcd_vec(v: &[usize]) -> usize {
    if v.len() == 1 {
        return v[0];
    }
    let mean: usize = v.len()/2;
    let mut left = v.to_vec();
    let right = left.split_off(mean);
    let left_gcd = gcd_vec(&left);
    let right_gcd = gcd_vec(&right);
    gcd(left_gcd, right_gcd)
}

/*
GCD  

*/
fn main() {
    let N = read_num();
    let A = read_vec();
    
    let mut max_gcd = 1;
    for i in 0..N-1 {
        let mut left = A.clone();
        let right = left.split_off(i+1);
        left.pop(); // remove A[i]
        // println!("{}, {}", left.len(), right.len());
        let left_gcd = if left.len() == 0 {
            right[0]
        } else {
            gcd_vec(&left)
        };
        let right_gcd = if right.len() == 0 {
            left[0]
        } else {
            gcd_vec(&right)
        };
        let tmp_gcd = gcd(left_gcd, right_gcd);
        if tmp_gcd > max_gcd {
            max_gcd = tmp_gcd;
        }
    }
    println!("{}", max_gcd);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 4), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 4), 4);
        assert_eq!(gcd(32, 28), 4);
    }
}


/*
fn main() {
    let N = read_num();
    let A = read_vec();
    
    let mut max_gcd = 1;
    for i in 0..N {
        let mut tmp_gcd = if i == 0 {
            A[1]
        } else {
            A[0]
        };
        for j in 0..N {
            if j == i {
                continue;
            }
            // let debug_tmp = tmp_gcd;
            tmp_gcd = gcd(A[j], tmp_gcd);
            // println!("gcd({}, {}) = {}", A[j], debug_tmp, tmp_gcd);
        }
        if tmp_gcd > max_gcd {
            max_gcd = tmp_gcd
        }
    }

    println!("{}", max_gcd);
}
*/
