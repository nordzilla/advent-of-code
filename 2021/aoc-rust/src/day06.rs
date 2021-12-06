use aoc_runner_derive::*;
use std::{cell::RefCell, collections::HashMap};

type CacheInput = Vec<usize>;
type ArraysInput = [usize; 9];
type VecRotateInput = Vec<usize>;
type Output = usize;

thread_local! {
    static FISHIES_MAP_CACHE: RefCell<HashMap<(usize, usize), usize>> = RefCell::new(HashMap::with_capacity(256 * 8));
}
thread_local! {
    static FISHIES_VEC_CACHE: RefCell<[Option<usize>; 256 * 8]> = RefCell::new([None; 256 * 8]);
}

#[aoc_generator(day6, part1, MapCache)]
fn input_generator_map_cache1(raw_input: &str) -> CacheInput {
    raw_input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc_generator(day6, part2, MapCache)]
fn input_generator_map_cache2(raw_input: &str) -> CacheInput {
    raw_input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc_generator(day6, part1, ArrayCache)]
fn input_generator_array_cache1(raw_input: &str) -> CacheInput {
    raw_input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc_generator(day6, part2, ArrayCache)]
fn input_generator_array_cache2(raw_input: &str) -> CacheInput {
    raw_input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc_generator(day6, part1, VecRotation)]
fn input_generator_vec_rotate1(raw_input: &str) -> VecRotateInput {
    raw_input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold(vec![0; 9], |mut vec, timer| {
            vec[timer] += 1;
            vec
        })
}

#[aoc_generator(day6, part2, VecRotation)]
fn input_generator_vec_rotate2(raw_input: &str) -> VecRotateInput {
    raw_input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold(vec![0; 9], |mut vec, timer| {
            vec[timer] += 1;
            vec
        })
}

#[aoc_generator(day6, part1, ArraysIter)]
fn input_generator_arrays_iter1(raw_input: &str) -> ArraysInput {
    raw_input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0; 9], |mut arr, timer| {
            arr[timer] += 1;
            arr
        })
}

#[aoc_generator(day6, part2, ArraysIter)]
fn input_generator_arrays_iter2(raw_input: &str) -> ArraysInput {
    raw_input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0; 9], |mut arr, timer| {
            arr[timer] += 1;
            arr
        })
}

#[aoc_generator(day6, part1, ArrayModulus)]
fn input_generator_arrays_mod1(raw_input: &str) -> ArraysInput {
    raw_input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0; 9], |mut arr, timer| {
            arr[timer] += 1;
            arr
        })
}

#[aoc_generator(day6, part2, ArrayModulus)]
fn input_generator_arrays_mod2(raw_input: &str) -> ArraysInput {
    raw_input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0; 9], |mut arr, timer| {
            arr[timer] += 1;
            arr
        })
}

fn spawn_count_with_map_cache(days_left: usize, timer: usize) -> usize {
    let count = FISHIES_MAP_CACHE
        .with(|cache| cache.borrow().get(&(days_left, timer)).copied())
        .unwrap_or_else(|| {
            1 + (0..days_left.saturating_sub(timer))
                .rev()
                .step_by(7)
                .map(|days_left| spawn_count_with_map_cache(days_left, 8))
                .sum::<usize>()
        });
    FISHIES_MAP_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry((days_left, timer))
            .or_insert(count)
            .clone()
    })
}

fn spawn_count_with_array_cache(days_left: usize, timer: usize) -> usize {
    let count = FISHIES_VEC_CACHE
        .with(|cache| cache.borrow()[days_left * timer])
        .unwrap_or_else(|| {
            1 + (0..days_left.saturating_sub(timer))
                .rev()
                .step_by(7)
                .map(|days_left| spawn_count_with_map_cache(days_left, 8))
                .sum::<usize>()
        });
    FISHIES_VEC_CACHE.with(|cache| *cache.borrow_mut()[days_left * timer].get_or_insert(count))
}

fn spawn_count_with_vec_rotation(mut fish: Vec<usize>, days_left: usize) -> usize {
    for _ in 0..days_left {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }
    fish.iter().sum()
}

fn step([t0, t1, t2, t3, t4, t5, t6, t7, t8]: [usize; 9]) -> [usize; 9] {
    [t1, t2, t3, t4, t5, t6, t7 + t0, t8, t0]
}

fn spawn_count_with_arrays_iter(fish: [usize; 9], days_left: usize) -> usize {
    (0..days_left)
        .fold(fish, |fish, _| step(fish))
        .into_iter()
        .sum()
}

fn spawn_count_with_array_modulus(init: &[usize], days_left: usize) -> usize {
    let mut fish: [usize; 16] = [0; 16];
    fish[..init.len()].copy_from_slice(init);

    for i in 0..days_left {
        let k = fish[i % 16];
        fish[(i + 6 + 1) % 16] += k;
        fish[(i + 8 + 1) % 16] += k;
        fish[i % 16] = 0;
    }
    fish.into_iter().sum()
}

#[aoc(day6, part1, MapCache)]
fn solve_part1_map_cache(input: &CacheInput) -> Output {
    input
        .iter()
        .map(|&timer| spawn_count_with_map_cache(80, timer))
        .sum()
}

#[aoc(day6, part1, ArrayCache)]
fn solve_part1_array_cache(input: &CacheInput) -> Output {
    input
        .iter()
        .map(|&timer| spawn_count_with_array_cache(80, timer))
        .sum()
}

#[aoc(day6, part2, MapCache)]
fn solve_part2_map_cache(input: &CacheInput) -> Output {
    input
        .iter()
        .map(|&timer| spawn_count_with_map_cache(256, timer))
        .sum()
}

#[aoc(day6, part2, ArrayCache)]
fn solve_part2_array_cache(input: &CacheInput) -> Output {
    input
        .iter()
        .map(|&timer| spawn_count_with_array_cache(256, timer))
        .sum()
}

#[aoc(day6, part1, VecRotation)]
fn solve_part1_vec_rotation1(input: &VecRotateInput) -> Output {
    spawn_count_with_vec_rotation(input.clone(), 80)
}

#[aoc(day6, part2, VecRotation)]
fn solve_part2_vec_rotation2(input: &VecRotateInput) -> Output {
    spawn_count_with_vec_rotation(input.clone(), 256)
}

#[aoc(day6, part1, ArraysIter)]
fn solve_part1_arrays_iter1(input: &ArraysInput) -> Output {
    spawn_count_with_arrays_iter(input.clone(), 80)
}

#[aoc(day6, part2, ArraysIter)]
fn solve_part2_arrays_iter2(input: &ArraysInput) -> Output {
    spawn_count_with_arrays_iter(input.clone(), 256)
}

#[aoc(day6, part1, ArrayModulus)]
fn solve_part1_arrays_mod1(input: &ArraysInput) -> Output {
    spawn_count_with_array_modulus(input, 80)
}

#[aoc(day6, part2, ArrayModulus)]
fn solve_part2_arrays_mod2(input: &ArraysInput) -> Output {
    spawn_count_with_array_modulus(input, 256)
}
