#![allow(dead_code, unused_variables)]
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::arch::x86_64::{
    __m128i, _mm_cmpeq_epi16, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi16, _mm_set_epi16,
    _popcnt32,
};
use std::time::Instant;

fn main() {
    //BIG 32_768
    const SIZE: usize = 65_536;
    let array: [i16; SIZE] = get_random_array();
    //println!("Array1:\n{:?}", array1);
    //println!("Array2:\n{:?}", array2);
    let start = Instant::now();
    let result = find_occurrences(&array, array[0]);
    let time = start.elapsed();
    println!("Array1:\n{}", result);
    println!("Time1:\n{:?}", time);
    let start = Instant::now();
    let result = find_occurrences_2(&array, array[0]);
    let time = start.elapsed();
    println!("Array2:\n{}", result);
    println!("Time2:\n{:?}", time);
}

fn get_random_array<T: Default + Copy, const N: usize>() -> [T; N]
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    let mut array = [T::default(); N];
    for item in &mut array {
        *item = rng.gen();
    }
    array
}

fn find_occurrences<const N: usize>(array: &[i16; N], v: i16) -> i32 {
    let mut cnt = 0;
    for item in array {
        if *item == v {
            cnt += 1;
        }
    }
    cnt
}

fn find_occurrences_2<const N: usize>(array: &[i16; N], v: i16) -> i32 {
    unsafe {
        let mut cnt = 0;
        let sse_val = _mm_set1_epi16(v);
        for i in (0..N).step_by(8) {
            let sse_arr = _mm_set_epi16(
                array[i],
                array[i + 1],
                array[i + 2],
                array[i + 3],
                array[i + 4],
                array[i + 5],
                array[i + 6],
                array[i + 7],
            );
            cnt += _popcnt32(_mm_movemask_epi8(_mm_cmpeq_epi16(sse_val, sse_arr)));
        }
        cnt >> 1
    }
}

fn find_occurrences_3<const N: usize>(array: &[i16; N], v: i16) -> i32 {
    unsafe {
        //std::mem::transmute
        let mut cnt = 0;
        let sse_val = _mm_set1_epi16(v);
        let values = array.to_vec();
        for i in (0..N).step_by(8) {
            println!("{:p}", values[i] as *mut __m128i);
            println!("{:p}", array.as_ptr() as *mut __m128i);
            let t = array.as_ptr();
            let sse_arr = _mm_loadu_si128(array.as_ptr() as *const __m128i);
            cnt += _popcnt32(_mm_movemask_epi8(_mm_cmpeq_epi16(sse_val, sse_arr)));
        }
        cnt >> 1
    }
}
