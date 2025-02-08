#![feature(iter_array_chunks)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;
advent_of_code::solution!(23);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Cpt(char, char);

struct CptGroup<const N: usize> {
    computers: [Cpt; N],
}

impl<const N: usize> CptGroup<N> {
    fn new(mut cpt: [Cpt; N]) -> Self {
        cpt.sort();
        Self { computers: cpt }
    }
}

impl<const N: usize> PartialEq for CptGroup<N> {
    fn eq(&self, other: &Self) -> bool {
        for cpt in self.computers {
            if !other.computers.contains(&cpt) {
                return false;
            }
        }
        return true;
    }
}

impl<const N: usize> Eq for CptGroup<N> {}

impl<const N: usize> Hash for CptGroup<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for cpt in self.computers {
            cpt.hash(state);
        }
    }
}

struct CptNetwork {
    graph: HashMap<Cpt, HashSet<Cpt>>,
}

impl CptNetwork {
    fn add_conn(&mut self, c1: Cpt, c2: Cpt) {
        self.graph
            .entry(c1)
            .and_modify(|s| {
                s.insert(c2);
            })
            .or_insert({
                let mut s = HashSet::new();
                s.insert(c2);
                s
            });

        self.graph
            .entry(c2)
            .and_modify(|s| {
                s.insert(c1);
            })
            .or_insert({
                let mut s = HashSet::new();
                s.insert(c1);
                s
            });
    }

    fn part_one_groups(&self) -> usize {
        let mut cpt_groups: HashSet<CptGroup<3>> = HashSet::new();
        for cpt in self.graph.keys() {
            for combinations in self.graph.get(cpt).unwrap().into_iter().combinations(2) {
                let cpt2 = combinations[0];
                let cpt3 = combinations[1];

                if cpt.0 != 't' && cpt2.0 != 't' && cpt3.0 != 't' {
                    continue;
                }

                if self.graph.get(cpt2).unwrap().contains(cpt3) {
                    cpt_groups.insert(CptGroup::new([*cpt, *cpt2, *cpt3]));
                }
            }
        }
        cpt_groups.len()
    }

    fn filter_potential_groups(
        &self,
        potential_set: HashSet<Cpt>,
        cache: &mut HashMap<Vec<Cpt>, HashSet<Cpt>>,
    ) -> HashSet<Cpt> {
        let mut cache_key = potential_set.clone().into_iter().collect::<Vec<Cpt>>();
        cache_key.sort();
        let cache_key = cache_key;

        if let Some(best_group_cache) = cache.get(&cache_key) {
            return best_group_cache.clone();
        }
        let mut current_best_group = HashSet::new();
        let mut flag_potential_filtered = false;
        for cpt_2combinations in potential_set.iter().combinations(2) {
            let cpt1 = cpt_2combinations[0];
            let cpt2 = cpt_2combinations[1];

            if !self.graph.get(cpt1).unwrap().contains(cpt2) {
                flag_potential_filtered = true;
                let mut new_potential = potential_set.clone();
                let mut new_potential2 = potential_set.clone();
                new_potential.remove(cpt1);
                new_potential2.remove(cpt2);

                let group1 = self.filter_potential_groups(new_potential, cache);
                let group2 = self.filter_potential_groups(new_potential2, cache);

                let bigger_group = if group1.len() > group2.len() {
                    group1
                } else {
                    group2
                };
                if current_best_group.len() < bigger_group.len() {
                    current_best_group = bigger_group
                }
            }
        }
        let res = if !flag_potential_filtered {
            potential_set
        } else {
            current_best_group
        };

        cache.insert(cache_key, res.clone());
        res
    }

    fn part_two_biggest_group(&self) -> String {
        let mut best_group = HashSet::new();
        let mut cache = HashMap::new();
        for cpt in self.graph.keys() {
            let mut potential_group = self.graph.get(&cpt).unwrap().clone();
            potential_group.insert(*cpt);

            let candidate_best_group = self.filter_potential_groups(potential_group, &mut cache);
            if candidate_best_group.len() > best_group.len() {
                best_group = candidate_best_group
            }
        }

        let mut best_group_vec: Vec<Cpt> = best_group.into_iter().collect();
        best_group_vec.sort();
        let mut password = String::new();
        for &cpt in &best_group_vec {
            password.push(cpt.0);
            password.push(cpt.1);
            if cpt != *best_group_vec.last().unwrap() {
                password.push(',');
            }
        }
        password
    }
}

impl From<&str> for Cpt {
    fn from(value: &str) -> Self {
        let val = value.trim();
        assert!(val.len() == 2);
        let mut char_iter = val.chars();
        let c1 = char_iter.next().unwrap();
        let c2 = char_iter.next().unwrap();
        Self(c1, c2)
    }
}

impl From<&str> for CptNetwork {
    fn from(value: &str) -> Self {
        let mut cpt_network = CptNetwork {
            graph: HashMap::new(),
        };
        value
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .map(|(c1, c2)| (Cpt::from(c1), Cpt::from(c2)))
            .for_each(|(c1, c2)| cpt_network.add_conn(c1, c2));
        cpt_network
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(CptNetwork::from(input).part_one_groups())
}

pub fn part_two(input: &str) -> Option<String> {
    Some(CptNetwork::from(input).part_two_biggest_group())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
