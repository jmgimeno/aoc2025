use day07::day07::{INPUT, part1, part2, part1_slower};

fn main() {
    divan::main()
}

#[divan::bench]
fn bench_part1() {
    part1(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part1_slower() {
    part1_slower(divan::black_box(&INPUT));
}

#[divan::bench]
fn bench_part2() {
    part2(divan::black_box(&INPUT));
}