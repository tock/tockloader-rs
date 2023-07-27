#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(feature = "std")]
pub fn add(left: usize, right: usize) -> usize {
    let a = [left, right].to_vec();
    let mut sum = 1;
    for item in a {
        sum += item
    }
    return sum
}

#[cfg(not(feature = "std"))]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
