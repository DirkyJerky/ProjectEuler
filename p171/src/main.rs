extern crate lazy_static;

use lazy_static::lazy_static;

const SQUARES: [usize; 10] = [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
const MAX_DIGITS: usize = 3;

lazy_static! {
    static ref IS_SQUARE: [bool; 1621] = {
        let mut squares = [false; 1621];
        
        for i in 0..=40 {
            squares[i*i] = true;
        }

        squares
    };
}

fn process(arr: &[usize; 10]) {
    let mut total = 0;

    for i in 1..10 {
        total += SQUARES[i] * arr[i];
    }

    if IS_SQUARE[total] {
        dbg!(arr);
    }
}

fn iterate(arr: &mut [usize; 10], step: usize, min_val: usize) {
    if step == MAX_DIGITS {
        process(arr);
        return
    }

    for i in min_val..=9 {
        arr[i] += 1;

        iterate(arr, step + 1, i);

        arr[i] -= 1;
    }
}

fn main() {
    let mut arr = [0usize; 10];
    iterate(&mut arr, 0, 0);
}
