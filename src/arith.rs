
pub fn gcd(a: i32, b: i32) -> i32 {
    let a = a.abs();
    let b = b.abs();
    if a == 0 { b }
    else if b == 0 { a }
    else {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}

pub fn gcd_vec(vec: &Vec<i32>) -> i32 {
    vec.iter().fold(0, |a, b|{ gcd(a, *b) })
}

pub fn lcm(a: i32, b: i32) -> i32 {
    let a = a.abs();
    let b = b.abs();
    a*b / gcd(a, b)
}

pub fn lcm_vec(vec: &Vec<i32>) -> i32 {
    vec.iter().fold(1, |a, b|{ lcm(a, *b) })
}
