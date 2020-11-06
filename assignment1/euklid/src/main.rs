use std::mem::replace;

/*
Given pseudocode:
function gcd(a, b)
»   while b != 0
»   »   t := b
»   »   b := a mod b
»   »   a := t
»   return a
*/
fn gcd_it(mut a:u32, mut b:u32) -> u32 {
    while b!=0 {
        let next_b=a%b;
        a=replace(&mut b,next_b);
    }
    a
}

/*
Given Pseudocode:
function gcd(a, b)
»   if b == 0
»   »   return a
»   else
»   »   return gcd(b, a mod b)
 */
fn gcd_rc(a:u32,b:u32) -> u32 {
    match b {
        0 => a,
        _=> gcd_rc(b,a % b)
    }
}

const EXAMPLES:[(u32, u32,u32);4]=[(12, 6,6),(9, 4,1),(10, 4,2),(21, 14,7)];

fn main() {
    for p in EXAMPLES.iter() {
        println!("gcd_rc{:?}={} gcd_it{:?}={}]",(p.0,p.1),gcd_rc(p.0,p.1),(p.0,p.1),gcd_it(p.0,p.1))
    }
}

#[cfg(test)]
mod tests {
    use crate::{EXAMPLES, gcd_rc, gcd_it};

    #[test]
    fn test_rc_commutative() {
        for p in EXAMPLES.iter() {
            assert_eq!(gcd_rc(p.0,p.1),gcd_rc(p.1,p.0))
        }
    }
    #[test]
    fn test_rc() {
        for p in EXAMPLES.iter()  {
            assert_eq!(gcd_rc(p.0,p.1),p.2)
        }
    }

    #[test]
    fn test_it() {
        for p in EXAMPLES.iter()  {
            assert_eq!(gcd_it(p.0,p.1),p.2)
        }
    }
    #[test]
    fn test_it_commutative() {
        for p in EXAMPLES.iter()  {
            assert_eq!(gcd_it(p.0,p.1),gcd_it(p.1,p.0))
        }
    }
}