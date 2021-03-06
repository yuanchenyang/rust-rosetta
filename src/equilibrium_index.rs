// http://rosettacode.org/wiki/Equilibrium_index
use std::iter::AdditiveIterator;
use std::num::Zero;
use std::ops::Add;

fn equilibrium_indices<T: Add<T, T> + Sub<T, T> + Eq + Zero + Copy>(v: &[T]) -> Vec<uint> {
    let mut right = v.iter().map(|&x| x).sum();
    let mut left: T = Zero::zero();
    let mut ret = vec![];

    for (i, &el) in v.iter().enumerate() {
        // NOTE: -= and += doesn't work on left/right and el for some reason.
        right = right - el;
        if left == right {
            ret.push(i);
        }
        left = left + el;
    }
    ret
}

#[test]
fn test_equilibrium_indices() {
    let v = [-7i, 1, 5, 2, -4, 3, 0];
    assert_eq!(equilibrium_indices(v), vec![3u, 6]);
}

#[cfg(not(test))]
fn main() {
    let v = [-7i, 1, 5, 2, -4, 3, 0];
    let indices = equilibrium_indices(v);
    println!("Equilibrium indices for {} are: {}", v.as_slice(), indices);
}
