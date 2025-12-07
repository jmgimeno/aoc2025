use day06::day06::{INPUT, part1, part2, part1_iterative, part2_iterative, part2_strings};

fn main() {
    divan::main()
}

#[divan::bench]
fn bench_part1() {
    part1(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part2() {
    part2(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part1_iterative() {
    part1_iterative(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part2_iterative() {
    part2_iterative(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part2_strings() {
    part2_strings(divan::black_box(&INPUT));
}