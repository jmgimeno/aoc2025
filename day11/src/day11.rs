use common::read_file_as_string;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day11.txt").expect("Failed to load input"));

struct Rack {
    index: HashMap<String, usize>,
    outputs: Vec<Vec<usize>>,
}

impl Rack {
    fn new(input: &str) -> Self {
        let mut index = HashMap::new();
        let mut outputs = Vec::new();

        for line in input.lines() {
            let parts = line.split(':').collect::<Vec<_>>();
            let from = parts[0].trim();
            let from_i = intern(from, &mut index, &mut outputs);

            let targets = parts
                .get(1)
                .map(|s| s.split_whitespace())
                .into_iter()
                .flatten();
            for t in targets {
                let to_i = intern(t, &mut index, &mut outputs);
                outputs[from_i].push(to_i);
            }
        }

        Self {
            index,
            outputs,
        }
    }

    fn count_paths(&self, from: &str, to: &str) -> usize {
        // NOTE: I don't need to store the current path for the given data !!
        let &from_i = match self.index.get(from) {
            Some(i) => i,
            None => return 0,
        };
        let &to_i = match self.index.get(to) {
            Some(i) => i,
            None => return 0,
        };
        let mut cache = HashMap::new();
        Self::count_paths_inner(self, from_i, to_i, &mut cache)
    }

    fn count_paths_inner(
        &self,
        from: usize,
        to: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if from == to {
            return 1;
        }
        if let Some(&v) = cache.get(&(from, to)) {
            return v;
        }

        let sum: usize = self
            .outputs
            .get(from)
            .map(|v| {
                v.iter()
                    .filter_map(|&s| {
                        Some(self.count_paths_inner(s, to, cache))
                    })
                    .sum()
            })
            .unwrap_or(0);

        cache.insert((from, to), sum);
        sum
    }
}

fn intern(
    s: &str,
    index: &mut HashMap<String, usize>,
    outputs: &mut Vec<Vec<usize>>,
) -> usize {
    use std::collections::hash_map::Entry;
    let i = index.len(); // cannot borrow after
    match index.entry(s.to_string()) {
        Entry::Occupied(o) => *o.get(),
        Entry::Vacant(v) => {
            v.insert(i);
            outputs.push(Vec::new());
            i
        }
    }
}

pub fn part1(input: &str) -> usize {
    let rack = Rack::new(input);
    rack.count_paths("you", "out")
}

pub fn part2(input: &str) -> usize {
    let rack = Rack::new(input);

    let leg1a = rack.count_paths("svr", "fft");
    let leg2a = rack.count_paths("fft", "dac");
    let leg3a = rack.count_paths("dac", "out");
    let fft_before_dac = leg1a * leg2a * leg3a;

    let leg1b = rack.count_paths("svr", "dac");
    let leg2b = rack.count_paths("dac", "fft");
    let leg3b = rack.count_paths("fft", "out");
    let dac_before_fft = leg1b * leg2b * leg3b;

    fft_before_dac + dac_before_fft
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 534);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(part2(input), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 499645520864100);
    }
}
