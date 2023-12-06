use std::cmp::min;

struct ResourceMapping {
    dest_start: usize,
    source_start: usize,
    range: usize,
}

impl ResourceMapping {
    fn process_seed(&self, seed: usize) -> Option<usize> {
        if seed >= self.source_start && seed <= self.source_start + self.range {
            return Some(self.dest_start + (seed - self.source_start));
        }
        return None;
    }
}

struct ResourceMap {
    mappings: Vec<ResourceMapping>,
}

impl ResourceMap {
    fn new() -> ResourceMap {
        ResourceMap {
            mappings: Vec::new(),
        }
    }

    fn process_seed(&self, seed: usize) -> usize {
        match self.mappings.binary_search_by_key(&seed, |r| r.source_start) {
            Ok(idx) => self.mappings.get(idx).and_then(|r| r.process_seed(seed)).unwrap(),
            Err(idx) => match idx > 0 {
                true => self.mappings.get(idx - 1).and_then(|r| r.process_seed(seed)).unwrap_or(seed),
                false => seed,
            }
        }
    }
}

struct Almanac {
    seeds: Vec<usize>,
    resource_maps: Vec<ResourceMap>,
}

impl Almanac {
    fn parse_input(input: &str) -> Almanac {
        let mut parts = input.split("\n\n");
        let seeds = parts
            .next()
            .and_then(|s| s.strip_prefix("seeds: "))
            .map(|s| s.split_whitespace())
            .unwrap()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut resource_maps = Vec::new();
        for _ in 0..7 {
            let map_lines = parts.next().unwrap().lines().skip(1);
            let mut resource_map = ResourceMap::new();
            for line in map_lines {
                let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                resource_map.mappings.push(ResourceMapping {
                    dest_start: nums.next().unwrap(),
                    source_start: nums.next().unwrap(),
                    range: nums.next().unwrap(),
                });
            }
            resource_map.mappings.sort_by_key(|r| r.source_start);
            resource_maps.push(resource_map);
        }

        return Almanac {
            seeds,
            resource_maps,
        };
    }

    fn process_seed(&self, mut seed: usize) -> usize {
        for resource_map in self.resource_maps.iter() {
            seed = resource_map.process_seed(seed);
        }
        return seed;
    }
}

fn part_1(input: &str) -> usize {
    let almanac = Almanac::parse_input(input);
    return almanac
        .seeds
        .iter()
        .map(|seed| almanac.process_seed(*seed))
        .min()
        .unwrap();
}

fn part_2(input: &str) -> usize {
    let almanac = Almanac::parse_input(input);
    let mut min_loc = usize::MAX;
    for pair in almanac.seeds.chunks_exact(2) {
        let [seed, range] = pair else { panic!() };
        for seed in *seed..*seed+*range {
            min_loc = min(min_loc, almanac.process_seed(seed));
        }
    }
    return min_loc;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_2023::data::day_5::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 35);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 525792406);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 46);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 79004094);
    }
}
