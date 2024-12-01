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

// slower with BTreeMap as cache
struct SortedVec(Vec<u32> /* HashMap<u32, u32> */);

impl SortedVec {
    fn new(mut vec: Vec<u32>) -> Self {
        vec.sort();
        SortedVec(vec /* HashMap::new() */)
    }
}

impl SortedVec {
    fn similarity(&mut self, n: u32) -> u32 {
        // if self.1.contains_key(&n) {
        //     return *self.1.get(&n).unwrap();
        // }

        let mut count: u32 = 0;
        for &v in self.0.iter() {
            if v == n {
                count += 1;
            }
            if v > n {
                break;
            }
        }
        // let sim = count * n;
        // self.1.insert(n, sim);
        // sim
        count * n
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
    left.into_iter()
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
