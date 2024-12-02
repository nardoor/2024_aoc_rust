use std::cmp::Ordering;

advent_of_code::solution!(2);

struct Report(Vec<u32>);

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        let split = value.split(" ");
        let v = split
            .into_iter()
            .map(|e| e.parse::<u32>().expect("Failed parse."))
            .collect();
        Self(v)
    }
}

impl Report {
    fn copy_without(&self, idx: usize) -> Self {
        let mut vec = self.0.clone();
        vec.remove(idx);
        Report(vec)
    }

    fn is_safe(&self) -> bool {
        let ord = self.0[0].cmp(&self.0[1]);
        if ord == Ordering::Equal {
            return false;
        }

        let counter_example = self.0.windows(2).find(|&vals| {
            let [a, b] = vals else { unreachable!() };
            a.cmp(b) != ord || a.abs_diff(*b) > 3
        });

        counter_example.is_none()
    }

    fn is_safe_dampener(&self) -> bool {
        let ord = self.0[0].cmp(self.0.last().unwrap());
        if ord == Ordering::Equal {
            return false;
        }

        for (idx, vals) in self.0.windows(2).enumerate() {
            let [a, b] = vals else { unreachable!() };
            if a.cmp(b) != ord || a.abs_diff(*b) > 3 {
                return self.copy_without(idx).is_safe() || self.copy_without(idx + 1).is_safe();
            }
        }
        return true;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Report::from)
            .filter(Report::is_safe)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Report::from)
            .filter(Report::is_safe_dampener)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_tricky_examples() {
        let report = Report::from("75 76 75 72 71 68");
        assert!(report.is_safe_dampener());
    }
}
