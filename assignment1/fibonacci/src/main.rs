fn fib_it(t: u32) -> u32 {
    let mut current_values=(0,1);
    for _ in 0..t {
        current_values=(current_values.1,current_values.0+current_values.1);
    }
    current_values.0
}

fn fib_rc(t: u32) -> u32 {
    match t {
        0..=1 => t,
        _ => fib_rc(t-1)+fib_rc(t-2)
    }
}

#[cfg(test)]
mod tests {
    use crate::fib_it;
    use crate::fib_rc;

    const FIRST_SEQ_VALUES:[u32;10]=[0,1,1,2,3,5,8,13,21,34];

    #[test]
    fn test_it() {
        for t in 0..FIRST_SEQ_VALUES.len() {
            assert_eq!(FIRST_SEQ_VALUES[t],fib_it(t as u32))
        }
    }

    #[test]
    fn test_rc() {
        for t in 0..FIRST_SEQ_VALUES.len() {
            assert_eq!(FIRST_SEQ_VALUES[t],fib_rc(t as u32))
        }
    }
}

fn main() {
    for t in 0..20 {
        println!("fib_it({})={} fib_rc({})={}",t,fib_it(t),t,fib_rc(t))
    }
}
