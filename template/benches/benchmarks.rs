use {{project-name}}::{{project-name}}::{INPUT, part1, part2};

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