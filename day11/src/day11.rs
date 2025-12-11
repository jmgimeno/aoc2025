use common::read_file_as_string;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day11.txt").expect("Failed to load input"));

struct Rack {
    outputs: HashMap<String, Vec<String>>,
}

impl Rack {
    fn new(input: &str) -> Self {
        let mut outputs = HashMap::new();
        for line in input.lines() {
            let parts1 = line.split(':').collect::<Vec<&str>>();
            let parts2 = parts1[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            outputs.insert(parts1[0].to_string(), parts2.to_vec());
        }
        Self { outputs }
    }

    fn count_paths<'a>(&self, from: &'a str, to: &'a str) -> usize {
        let mut path = Vec::new();
        let mut cache = HashMap::new();
        Self::count_paths_inner(self, from, to, &mut path, &mut cache)
    }

    fn count_paths_inner<'a>(&self, from: &'a str, to: &'a str, path: &mut Vec<String>, cache: &mut HashMap<(String, String), usize>) -> usize {
        if from == to {
            1
        } else if cache.contains_key(&(from.to_string(), to.to_string())) {
            cache[&(from.to_string(), to.to_string())]
        } else {
            path.push(from.to_string());
            let sum = self
                .outputs
                .get(from)
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|s| (!path.contains(s)).then_some(self.count_paths_inner(s, to, path, cache)))
                .sum();
            cache.insert((from.to_string(), to.to_string()), sum);
            path.pop();
            sum
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
