advent_of_code::solution!(7);

struct Equation {
    test_val: usize,
    nums: Vec<usize>,
}

enum Op {
    Mul,
    Add,
    Concat,
}

impl Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            Self::Concat => a * 10usize.pow(b.ilog10() + 1) + b,
        }
    }
    fn all_part1() -> [Self; 2] {
        [Op::Add, Op::Mul]
    }
    fn all_part2() -> [Self; 3] {
        [Op::Mul, Op::Add, Op::Concat]
    }
}

impl From<&str> for Equation {
    fn from(line: &str) -> Self {
        let (test_val, nums) = line.split_once(":").unwrap();
        let test_val: usize = test_val.parse().unwrap();
        let nums = nums.split(" ");
        let nums: Vec<usize> = nums
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        Self { test_val, nums }
    }
}

impl Equation {
    fn _solvable(&self, current_val: usize, next_num_idx: usize, operations: &[Op]) -> bool {
        if current_val > self.test_val {
            return false;
        }
        let Some(&next_num) = self.nums.get(next_num_idx) else {
            // we reached last_num
            return current_val == self.test_val;
        };

        for op in operations {
            let current_val = op.apply(current_val, next_num);
            if self._solvable(current_val, next_num_idx + 1, operations) {
                return true;
            }
        }
        return false;
    }

    fn solvable_part1(&self) -> bool {
        self._solvable(self.nums[0], 1, &Op::all_part1())
    }
    fn solvable_part2(&self) -> bool {
        self._solvable(self.nums[0], 1, &Op::all_part2())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(Equation::from)
            .filter(|e| e.solvable_part1())
            .map(|e| e.test_val)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(Equation::from)
            .filter(|e| e.solvable_part2())
            .map(|e| e.test_val)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
