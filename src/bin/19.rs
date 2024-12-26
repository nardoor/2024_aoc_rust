use std::collections::{BTreeMap, BTreeSet, VecDeque};

advent_of_code::solution!(19);

#[derive(Debug)]
struct Towel(String);

impl From<&str> for Towel {
    fn from(value: &str) -> Self {
        assert!(!value.is_empty());
        let value = value.trim();
        Self(value.to_owned())
    }
}

#[derive(Debug)]
struct Design(String);

impl From<&str> for Design {
    fn from(value: &str) -> Self {
        let value = value.trim();
        Self(value.to_owned())
    }
}

enum CanBeDoneRes {
    Ok(usize),
    CacheHit(usize),
    Nop,
}

impl CanBeDoneRes {
    fn to_opt(self) -> Option<usize> {
        match self {
            Self::CacheHit(cp) | Self::Ok(cp) => Some(cp),
            Self::Nop => None,
        }
    }
}

impl Design {
    fn match_at(&self, idx: usize, towel: &Towel) -> bool {
        // dbg!(idx);
        // dbg!(self.0.len());
        // dbg!(&self);
        assert!(idx < self.0.len());
        return self.0[idx..].starts_with(&towel.0);
    }

    fn can_be_done<'s1, 's2>(
        &'s1 self,
        idx: usize,
        towels: &Vec<Towel>,
        cache: &mut BTreeMap<&'s2 str, usize>,
        cache_not: &mut BTreeSet<&'s2 str>,
    ) -> CanBeDoneRes
    where
        's1: 's2,
    {
        let mut total = 0;
        // dbg!(idx);
        // dbg!(&self.0);
        // dbg!(&self.0[idx..]);
        if idx == self.0.len() {
            return CanBeDoneRes::Ok(1);
        }
        if idx > self.0.len() {
            return CanBeDoneRes::Nop;
        }
        if cache_not.contains(&self.0[idx..]) {
            return CanBeDoneRes::Nop;
        }
        if let Some(&cp) = cache.get(&self.0[idx..]) {
            // dbg!("cache hit");
            // dbg!(&self.0[idx..]);
            return CanBeDoneRes::CacheHit(cp);
        }
        for t in towels {
            if !self.match_at(idx, t) {
                continue;
            }
            // dbg!("matching: ", &t.0);
            match self.can_be_done(idx + t.0.len(), towels, cache, cache_not) {
                CanBeDoneRes::Nop => {
                    cache_not.insert(&self.0[(idx + t.0.len())..]);
                }
                CanBeDoneRes::CacheHit(cp) => total += cp,
                CanBeDoneRes::Ok(cp) => {
                    total += cp;
                    cache.insert(&self.0[(idx + t.0.len())..], cp);
                }
            }
        }
        if total == 0 {
            return CanBeDoneRes::Nop;
        }
        return CanBeDoneRes::Ok(total);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let mut towels: Vec<Towel> = towels.split(", ").map(Towel::from).collect();
    towels.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    // dbg!(&towels);
    let designs: Vec<Design> = designs
        .split("\n")
        .filter(|&d| !d.is_empty())
        .map(Design::from)
        .collect();
    let mut cache: BTreeMap<&str, usize> = BTreeMap::new();
    let mut cache_not: BTreeSet<&str> = BTreeSet::new();

    Some(
        designs
            .iter()
            .filter(|&d| {
                d.can_be_done(0, &towels, &mut cache, &mut cache_not)
                    .to_opt()
                    .is_some()
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let mut towels: Vec<Towel> = towels.split(", ").map(Towel::from).collect();
    towels.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    // dbg!(&towels);
    let designs: Vec<Design> = designs
        .split("\n")
        .filter(|&d| !d.is_empty())
        .map(Design::from)
        .collect();
    let mut cache: BTreeMap<&str, usize> = BTreeMap::new();
    let mut cache_not: BTreeSet<&str> = BTreeSet::new();

    Some(
        designs
            .iter()
            .filter_map(|d| {
                d.can_be_done(0, &towels, &mut cache, &mut cache_not)
                    .to_opt()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
