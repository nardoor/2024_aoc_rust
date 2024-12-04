advent_of_code::solution!(4);

struct Grid(Vec<Vec<char>>);

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self(value.lines().map(|l| l.chars().collect()).collect())
    }
}

trait Offset {
    fn offset(&self, offset: (isize, isize)) -> Option<(usize, usize)>;
}

impl Offset for (usize, usize) {
    fn offset(&self, offset: (isize, isize)) -> Option<(usize, usize)> {
        let new_x = self.0.checked_add_signed(offset.0)?;
        let new_y = self.1.checked_add_signed(offset.1)?;
        Some((new_x, new_y))
    }
}

impl Grid {
    fn get(&self, (x, y): (usize, usize)) -> Option<char> {
        self.0.get(y)?.get(x).cloned()
    }

    fn check_from_start(
        &self,
        start: (usize, usize),
        dir: (isize, isize),
        needle: &Vec<char>,
    ) -> bool {
        let mut pos = start;
        let mut index = 0;

        loop {
            let char_at_pos = self.get(pos);
            // out of grid?
            if char_at_pos.is_none() {
                break;
            }
            // wrong char?
            if char_at_pos.unwrap() != needle[index] {
                break;
            }

            index += 1;

            // found!
            if index == needle.len() {
                return true;
            }

            if let Some(new_pos) = pos.offset(dir) {
                pos = new_pos;
            } else {
                // out of grid (one of x or y was 0)
                break;
            }
        }

        false
    }

    fn search_count_needle(&self, needle: &str) -> u32 {
        let dirs = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        let needle_vec: Vec<char> = needle.chars().collect();
        self.0
            .iter() // for each line
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, &c)| {
                        // for each start_pos
                        if c == needle_vec[0] {
                            dirs.iter() // for each dir, check if we find `needle`
                                .filter(|&dir| self.check_from_start((x, y), *dir, &needle_vec))
                                .count() as u32
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    /// get pairs of pos corresponding to each diagonal
    fn cross_pos(pos: (usize, usize)) -> Option<[((usize, usize), (usize, usize)); 2]> {
        Some([
            (pos.offset((1, 1))?, pos.offset((-1, -1))?),
            (pos.offset((1, -1))?, pos.offset((-1, 1))?),
        ])
    }
    fn check_x_from_middle(&self, middle: (usize, usize), needle: &[char; 3]) -> bool {
        let Some(cross_pos) = Self::cross_pos(middle) else {
            return false;
        };

        // assert middle is the middle letter
        // assert_eq!(self.get(middle).unwrap(), needle[1]);

        for (pos1, pos2) in cross_pos {
            let Some(char1) = self.get(pos1) else {
                return false;
            };
            let Some(char2) = self.get(pos2) else {
                return false;
            };
            if !(char1 == needle[0] && char2 == needle[2])
                && !(char1 == needle[2] && char2 == needle[0])
            {
                // not a match for this diagonal
                return false;
            }
        }

        // both diagonals matched
        true
    }

    fn search_count_x_needle(&self, needle: &[char; 3]) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(x, &c)| {
                        if c == needle[1] {
                            self.check_x_from_middle((*x, y), needle)
                        } else {
                            false
                        }
                    })
                    .count() as u32
            })
            .sum::<u32>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    Some(grid.search_count_needle("XMAS"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    Some(grid.search_count_x_needle(&['M', 'A', 'S']))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
