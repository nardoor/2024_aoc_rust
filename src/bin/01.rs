use std::iter::zip;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|l| l.split_once("   ").expect("Failed split once..."))
        .for_each(|(l, r)| {
            left.push(l.parse::<u32>().expect("Failed u32 parse..."));
            right.push(r.parse::<u32>().expect("Failed u32 parse..."));
        });
    left.sort();
    right.sort();

    let mut res = 0;
    zip(left.iter(), right.iter()).for_each(|(&l, &r)| res += r.abs_diff(l));
    Some(res)
}

struct SortedVec(Vec<u32>, usize, (u32, u32));

impl SortedVec {
    fn new(mut vec: Vec<u32>) -> Self {
        vec.sort();
        SortedVec(vec, 0, (0, 0))
    }
}

impl SortedVec {
    fn similarity(&mut self, n: u32) -> u32 {
        // in cache?
        if self.2 .0 == n {
            return self.2 .1;
        }

        let mut count: u32 = 0;
        for (idx, &v) in self.0[self.1..].iter().enumerate() {
            if v == n {
                count += 1;
            }
            if v > n {
                // cache current index
                self.1 = idx;
                break;
            }
        }
        let sim = count * n;
        self.2 = (n, sim);
        sim
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|l| l.split_once("   ").expect("Failed split once..."))
        .for_each(|(l, r)| {
            left.push(l.parse::<u32>().expect("Failed u32 parse..."));
            right.push(r.parse::<u32>().expect("Failed u32 parse..."));
        });
    let mut sorted_right = SortedVec::new(right);
    let sorted_left = SortedVec::new(left);
    sorted_left
        .0
        .into_iter()
        .try_fold(0u32, |acc, e| acc.checked_add(sorted_right.similarity(e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
