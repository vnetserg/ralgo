//! This module contains the bottom-up implementation of
//! the well-known mergesort algorithm.

use std::cmp::{Ord, Ordering};

/// The bottom-up mergesort implementation.
/// Since mergesort uses additional memory,
/// it requires elements to also be `Copy`.
pub fn mergesort<T: Ord + Copy>(array: &mut [T]) {
    if array.len() < 2 {
        return;
    }

    // Copy `array` into vector
    let mut aux: Vec<T> = array.iter().map(|x| *x).collect();

    let mut primary = &mut *array;
    let mut secondary = &mut aux[..];
    let mut step = 1;
    let mut swapped = false;
    while step < primary.len() {
        merge_intervals(primary, secondary, step);
        std::mem::swap(&mut primary, &mut secondary);
        swapped = !swapped;
        step *= 2;
    }

    if swapped {
        secondary.clone_from_slice(primary);
    }
}

fn merge_intervals<T: Ord + Copy>(input: &[T], output: &mut [T], step: usize) {
    let mut input_chunks = input.chunks(step);
    let mut output_chunks = output.chunks_mut(2 * step);

    loop {
        let first = match input_chunks.next() {
            Some(chunk) => chunk,
            None => break,
        };
        let second = match input_chunks.next() {
            Some(chunk) => chunk,
            None => break,
        };
        let write_to = output_chunks.next().unwrap();
        merge(first, second, write_to);
    }
}

fn merge<T: Ord + Copy>(first: &[T], second: &[T], output: &mut [T]) {
    let mut first_iter = first.iter();
    let mut second_iter = second.iter();
    let mut left = first_iter.next();
    let mut right = second_iter.next();
    for result in output.iter_mut() {
        if left.is_none() {
            *result = *right.unwrap();
            right = second_iter.next();
        } else if right.is_none() {
            *result = *left.unwrap();
            left = first_iter.next();
        } else if left.unwrap().cmp(right.unwrap()) == Ordering::Less {
            *result = *left.unwrap();
            left = first_iter.next();
        } else {
            *result = *right.unwrap();
            right = second_iter.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{merge, merge_intervals, mergesort};

    #[test]
    fn merge_works() {
        fn test_merge(first: Vec<i32>, second: Vec<i32>, expected: Vec<i32>) {
            let mut merged = vec![0; expected.len()];
            merge(&first, &second, &mut merged);
            assert_eq!(merged, expected);
        }

        test_merge(vec![1], vec![2], vec![1, 2]);
        test_merge(vec![1, 3, 5], vec![2, 4, 6], vec![1, 2, 3, 4, 5, 6]);
        test_merge(vec![1], vec![2, 4, 6], vec![1, 2, 4, 6]);
        test_merge(vec![7], vec![2, 4, 6], vec![2, 4, 6, 7]);
        test_merge(vec![1, 3, 5], vec![0], vec![0, 1, 3, 5]);
        test_merge(vec![1, 3, 5], vec![8], vec![1, 3, 5, 8]);
    }

    #[test]
    fn merge_intervals_works() {
        fn test_merge_intervals(input: Vec<i32>, step: usize, expected: Vec<i32>) {
            let mut output = vec![0; expected.len()];
            merge_intervals(&input, &mut output, step);
            assert_eq!(output, expected);
        }

        test_merge_intervals(vec![1, 2], 1, vec![1, 2]);
        test_merge_intervals(vec![2, 1], 1, vec![1, 2]);
        test_merge_intervals(vec![1, 3, 2, 4], 2, vec![1, 2, 3, 4]);
        test_merge_intervals(vec![1, 4, 2, 3], 2, vec![1, 2, 3, 4]);
        test_merge_intervals(vec![3, 4, 1, 2], 2, vec![1, 2, 3, 4]);
        test_merge_intervals(vec![4, 3, 2, 1], 1, vec![3, 4, 1, 2]);
    }

    #[test]
    fn mergesort_works() {
        fn test_mergesort(mut input: Vec<i32>) {
            let mut copy = input.clone();
            copy.sort();
            mergesort(&mut input);
            assert_eq!(input, copy);
        }

        test_mergesort(vec![]);
        test_mergesort(vec![1]);
        test_mergesort(vec![1, 2]);
        test_mergesort(vec![2, 1]);
        test_mergesort(vec![9, 3, 3, 3, 3]);
        test_mergesort(vec![5, 3, 7, 4, 2, 2, 2, 3]);
        test_mergesort(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
