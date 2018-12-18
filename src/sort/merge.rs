//! This module contains implementation of
//! the well-known mergesort algorithm.

use std::cmp::Ord;

/// Mergesort implementation.
/// Since mergesort uses additional memory,
/// it requires elements to also be `Copy`.
pub fn mergesort<T: Ord + Copy>(array: &mut [T]) {
    if array.len() < 2 {
        return;
    }

    let mut aux: Vec<T> = array.iter().map(|&x| x).collect();

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
    let mut output_chunks = output.chunks_mut(2*step);

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
    for v in output.iter_mut() {
        if let Some(first_value) = first_iter.next() {
            if let Some(second_value) = second_iter.next() {
                *v = std::cmp::min(*first_value, *second_value);
            } else {
                *v = *first_value;
            }
        } else {
            *v = *second_iter.next().unwrap();
        }
    }
}

