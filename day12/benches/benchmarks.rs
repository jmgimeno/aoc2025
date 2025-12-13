use day12::day12::{INPUT, part1};

fn main() {
    divan::main()
}

#[divan::bench]
fn bench_part1() {
    part1(divan::black_box(&INPUT));
}
