pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut a_mut = a;
    let mut b_mut = b;

    while b_mut != 0 {
        let carry = a_mut & b_mut;
        a_mut = a_mut ^ b_mut;
        b_mut = carry << 1;
    } 

    a_mut
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum() {
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(2, 3), 5);
        assert_eq!(get_sum(3, 4), 7);
        assert_eq!(get_sum(4, 5), 9);
    }
}