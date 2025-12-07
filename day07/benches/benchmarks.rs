use day07::day07::*;

fn main() {
    divan::main()
}

#[divan::bench]
fn bench_part1_chars_bitset() {
    part1_chars_bitset(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part1_bytes_bitset() {
    part1_bytes_bitset(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part1_chars_array() {
    part1_chars_array(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part1_slower() {
    part1_slower(divan::black_box(&INPUT));
}


#[divan::bench]
fn bench_part1() {
    part1(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part2_chars_array() {
    part2_chars_array(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part2() {
    part2(divan::black_box(&INPUT));
}