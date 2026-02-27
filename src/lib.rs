use std::collections::HashMap;

pub fn fibo1(n: u128) -> u128 {
    if n <= 2 {
        1
    } else {
        fibo1(n - 1) + fibo1(n - 2)
    }
}

pub fn fibo2(n: u128) -> u128 {
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    for _ in 3..=n {
        b = a + b;
        a = b - a;
    }
    b
}

pub fn fibo3(n: u128) -> u128 {
    let mut m: [[u128; 2]; 2] = [[1, 1], [1, 0]];
    m = puissance(&m, n - 1);
    m[0][0]
}

fn multiply(a: &[[u128; 2]; 2], b: &[[u128; 2]; 2]) -> [[u128; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ],
    ]
}

fn puissance(m: &[[u128; 2]; 2], n: u128) -> [[u128; 2]; 2] {
    if n == 1 {
        m.clone()
    } else if n % 2 == 0 {
        let half = puissance(m, n / 2);
        multiply(&half, &half)
    } else {
        let half = puissance(m, n / 2);
        multiply(&multiply(&half, &half), m)
    }
}

pub fn fibo4(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if n <= 2 {
        return 1;
    } else if !memo.contains_key(&n) {
        let tmp = fibo4(n - 1, memo) + fibo4(n - 2, memo);
        memo.insert(n, tmp);
    }
    memo[&n]
}
