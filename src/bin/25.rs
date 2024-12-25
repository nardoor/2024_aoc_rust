advent_of_code::solution!(25);

const SCHEMA_WIDTH: usize = 5;
const SCHEMA_HEIGHT: usize = 7;

#[derive(Debug)]
struct Key {
    key_shape: [u8; 5],
}

impl TryFrom<&str> for Key {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let first_line = value.lines().next().unwrap();
        if let Some(_idx) = first_line.chars().find(|c| *c != '.') {
            return Err(());
        }
        // this allows indexing
        assert!(value.is_ascii());
        let mut key_shape = [0; 5];
        for x in 0..SCHEMA_WIDTH {
            for y in (0..(SCHEMA_HEIGHT - 1)).rev() {
                let char_idx = x + y * (SCHEMA_WIDTH + 1/* \n */);
                let c = value[char_idx..char_idx + 1].chars().next().unwrap();
                if c == '.' {
                    key_shape[x] = (SCHEMA_HEIGHT - 2 - y) as u8;
                    break;
                }
            }
        }
        Ok(Key { key_shape })
    }
}

#[derive(Debug)]
struct Lock {
    pin_shape: [u8; 5],
}

impl TryFrom<&str> for Lock {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let first_line = value.lines().next().unwrap();
        if let Some(_idx) = first_line.chars().find(|c| *c != '#') {
            return Err(());
        }
        // this allows indexing
        assert!(value.is_ascii());
        let mut pin_shape = [0; 5];
        for x in 0..SCHEMA_WIDTH {
            for y in 0..SCHEMA_HEIGHT {
                let char_idx = x + y * (SCHEMA_WIDTH + 1/* \n */);
                let c = value[char_idx..char_idx + 1].chars().next().unwrap();
                if c == '.' {
                    pin_shape[x] = (y - 1) as u8;
                    break;
                }
            }
        }
        Ok(Lock { pin_shape })
    }
}

impl Key {
    fn could_fit_lock(&self, lock: &Lock) -> bool {
        if let Some(_idx) = self
            .key_shape
            .iter()
            .zip(lock.pin_shape.iter())
            .find(|&(&k, &p)| k + p > SCHEMA_HEIGHT as u8 - 2)
        {
            return false;
        }
        return true;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.split("\n\n").for_each(|schema| {
        if let Ok(lock) = Lock::try_from(schema) {
            locks.push(lock);
        } else if let Ok(key) = Key::try_from(schema) {
            keys.push(key);
        } else {
            panic!()
        }
    });

    /* for know hypothesis is that all keys and all locks are != */
    Some(
        locks
            .iter()
            .map(|lock| keys.iter().filter(|&key| key.could_fit_lock(lock)).count())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_lock() {
        let lock = r"#####
.####
.####
.####
.#.#.
.#...
.....
";
        let lock = Lock::try_from(lock);
        assert!(lock.is_ok());

        let lock = lock.unwrap();
        assert_eq!(lock.pin_shape, [0, 5, 3, 4, 3]);
    }

    #[test]
    fn test_parsing_key() {
        let key = r".....
#....
#....
#...#
#.#.#
#.###
#####
";
        let key = Key::try_from(key);
        assert!(key.is_ok());

        let key = key.unwrap();
        assert_eq!(key.key_shape, [5, 0, 2, 1, 3]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
