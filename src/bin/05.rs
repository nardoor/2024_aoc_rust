use std::collections::{HashMap, HashSet};
advent_of_code::solution!(5);

struct Rule {
    before: u32,
    after: u32,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (before, after) = value.split_once("|").unwrap();
        Rule {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        }
    }
}

struct RuleSet(HashMap<u32, HashSet<u32>>);

impl From<Vec<Rule>> for RuleSet {
    fn from(rules: Vec<Rule>) -> Self {
        let mut map = HashMap::new();

        rules.into_iter().for_each(|rule| {
            map.entry(rule.before)
                .and_modify(|set: &mut HashSet<u32>| {
                    set.insert(rule.after);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(rule.after);
                    set
                });
        });
        Self(map)
    }
}

impl RuleSet {
    fn get_not_allowed_before(&self, val: u32) -> Option<&HashSet<u32>> {
        self.0.get(&val)
    }
}

struct Print(Vec<u32>);

impl From<&str> for Print {
    fn from(value: &str) -> Self {
        Print(value.split(",").map(|n| n.parse().unwrap()).collect())
    }
}

impl Print {
    fn is_valid(&self, rules: &RuleSet) -> bool {
        self.get_first_invalid_index(rules, None).is_none()
    }

    fn get_first_invalid_index(
        &self,
        rules: &RuleSet,
        start_index: Option<usize>,
    ) -> Option<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .skip(start_index.unwrap_or(0))
            .find_map(|(idx, v)| {
                let before_slice = &self.0[..idx];
                if let Some(not_allowed_before) = rules.get_not_allowed_before(*v) {
                    before_slice
                        .into_iter()
                        .enumerate()
                        .find_map(|(not_allowed_idx, before)| {
                            if not_allowed_before.contains(before) {
                                Some((idx, not_allowed_idx))
                            } else {
                                None
                            }
                        })
                } else {
                    None
                }
            })
    }

    /// Returns `true` if needed reordering
    fn reorder(&mut self, rules: &RuleSet) -> bool {
        let mut updated = false;
        let mut start_idx = 0;

        // do some kind of ~`bubble sort`
        while let Some((idx, invalid_index)) = self.get_first_invalid_index(rules, Some(start_idx))
        {
            let v = self.0.swap_remove(invalid_index);
            self.0.insert(idx, v);
            // self.0.swap(invalid_index, idx);
            // do not re-check everything from start
            // `invalid_index + 1` is the first index that could fail
            // once we've swapped
            start_idx = (invalid_index).min(self.0.len() - 1);
            updated = true;
        }
        return updated;
    }

    fn get_middle(&self) -> u32 {
        let len = self.0.len();
        self.0[(len - 1) / 2]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, prints) = input.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules.lines().map(Rule::from).collect();
    let rules = RuleSet::from(rules);
    let prints: Vec<Print> = prints.lines().map(Print::from).collect();

    Some(
        prints
            .iter()
            .filter(|p| p.is_valid(&rules))
            .map(|p| p.get_middle())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, prints) = input.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules.lines().map(Rule::from).collect();
    let rules = RuleSet::from(rules);
    let mut prints: Vec<Print> = prints.lines().map(Print::from).collect();

    Some(
        prints
            .iter_mut()
            .filter_map(|p| if p.reorder(&rules) { Some(p) } else { None })
            .map(|p| p.get_middle())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
