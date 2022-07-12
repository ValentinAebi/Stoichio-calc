use Stoichio_calc::arith::{gcd, gcd_vec, lcm, lcm_vec};

#[test]
fn gcd_72_30_test(){
    assert_eq!(6, gcd(72, 30));
}

#[test]
fn gcd_25_45_test(){
    assert_eq!(5, gcd(25, 45))
}

#[test]
fn gcd_35_minus77_test(){
    assert_eq!(7, gcd(35, -77))
}

#[test]
fn gcd_vect_32_20_44_8_52(){
    assert_eq!(4, gcd_vec(&Vec::from([32, 20, 44, 8, 52])))
}

#[test]
fn gcd_vect_22_5_41_101(){
    assert_eq!(1, gcd_vec(&Vec::from([22, 5, 41, 101])))
}

#[test]
fn lcm_45_25_test(){
    assert_eq!(9*5*5, lcm(45, 25))
}

#[test]
fn lcm_minus44_100_test(){
    assert_eq!(11*25*4, lcm(-44, 100))
}

#[test]
fn lcm_vect_22_33_51_8(){
    // 2*11   3*11   3*17   2*2*2  ->  2*2*2*11*3*17
    assert_eq!(2*11*3*17*4, lcm_vec(&Vec::from([22, 33, 51, 8])))
}
