#![feature(iter_array_chunks)]

use std::fmt::{Debug, Write};

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entry {
    start: usize,
    size: usize,
    file_id: usize,
}

struct DiskMap {
    entries: Vec<Entry>,
    total_size: usize,
}

impl Debug for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut written = 0;
        for entry in &self.entries {
            while entry.start > written {
                f.write_char('.')?;
                written += 1;
            }
            for _ in 0..entry.size {
                f.write_fmt(format_args!("{}", entry.file_id))?;
                written += 1;
            }
        }
        while self.total_size > written {
            f.write_char('.')?;
            written += 1;
        }
        f.write_char('\n')
    }
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let mut entries = Vec::new();
        let mut next_entry_start = 0;
        let mut file_id = 0;
        let mut chunks = value
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_digit(10).unwrap())
            .array_chunks::<2>();
        chunks.by_ref().for_each(|[file_s, empty_s]| {
            entries.push(Entry {
                start: next_entry_start,
                size: file_s as usize,
                file_id: file_id,
            });
            next_entry_start += file_s as usize;
            next_entry_start += empty_s as usize;
            file_id += 1
        });

        if let Some(remain) = chunks.into_remainder() {
            let file_s = remain.as_slice()[0];
            entries.push(Entry {
                start: next_entry_start,
                size: file_s as usize,
                file_id: file_id,
            });
        }

        let last_entry = entries.last().unwrap();
        let total_size = last_entry.start + last_entry.size;
        Self {
            entries,
            total_size,
        }
    }
}

#[derive(Debug)]
struct EmptyEntry {
    index: usize,
    start: usize,
    size: usize,
}

impl DiskMap {
    fn get_first_empty_entry(&self, start_index: Option<usize>) -> Option<EmptyEntry> {
        let start_index = start_index.unwrap_or(0).saturating_sub(1);
        let mut cursor = if start_index > 0 && start_index < self.entries.len() {
            self.entries[start_index].start
        } else {
            0
        };
        for (idx, next_entry) in self.entries.iter().enumerate().skip(start_index) {
            if next_entry.start > cursor {
                return Some(EmptyEntry {
                    start: cursor,
                    size: next_entry.start - cursor,
                    index: idx,
                });
            } else {
                cursor = next_entry.start + next_entry.size;
            }
        }
        if cursor < self.total_size {
            let last_entry = self.entries.last().unwrap();
            return Some(EmptyEntry {
                start: last_entry.start + last_entry.size,
                size: self.total_size - (last_entry.start + last_entry.size),
                index: self.entries.len(),
            });
        }
        None
    }

    fn get_first_empty_entry_min_size(&self, min_size: usize) -> Option<EmptyEntry> {
        let start_index = 0;
        let mut cursor = if start_index > 0 && start_index < self.entries.len() {
            self.entries[start_index].start
        } else {
            0
        };
        for (idx, next_entry) in self.entries.iter().enumerate().skip(start_index) {
            if next_entry.start > cursor && next_entry.start - cursor >= min_size {
                return Some(EmptyEntry {
                    start: cursor,
                    size: next_entry.start - cursor,
                    index: idx,
                });
            } else {
                cursor = next_entry.start + next_entry.size;
            }
        }
        if cursor < self.total_size && self.total_size - cursor >= min_size {
            let last_entry = self.entries.last().unwrap();
            return Some(EmptyEntry {
                start: last_entry.start + last_entry.size,
                size: self.total_size - (last_entry.start + last_entry.size),
                index: self.entries.len(),
            });
        }
        None
    }

    fn index_write(&mut self, entry: Entry, idx: usize) {
        assert!(idx > 0 /* for now write at 0 isn't supported, file is assumed there */);
        let entry_before = self.entries[idx - 1];
        // dbg!(&entry_before);
        // dbg!(&entry);
        assert!(entry_before.start + entry_before.size <= entry.start);
        if let Some(entry_after) = self.entries.get(idx + 1) {
            assert!(entry.start + entry.size <= entry_after.start);
        }
        if idx >= self.entries.len() {
            assert!(entry.start + entry.size <= self.total_size);
            self.entries.push(entry);
        } else {
            self.entries.insert(idx, entry);
        }
    }

    fn remove_entry(&mut self, entry: &Entry) {
        self.entries.remove(
            self.entries
                .iter()
                .enumerate()
                .find_map(|(idx, e)| if *e == *entry { Some(idx) } else { None })
                .unwrap(),
        );
    }

    fn fragmented_reorganize(&mut self) {
        let mut cache_index = 0;
        loop {
            // copy read entry
            let last_entry = *self.entries.last().unwrap();
            let first_empty = self.get_first_empty_entry(Some(cache_index)).unwrap();

            // are we done?
            if first_empty.start >= last_entry.start + last_entry.size {
                break;
            }

            // we are not done - remove last_entry before inserting new ones
            self.remove_entry(&last_entry);

            let Entry {
                file_id,
                size: mut to_write_size,
                ..
            } = last_entry;

            // recompute because we removed an entry
            let mut first_empty = self.get_first_empty_entry(Some(cache_index)).unwrap();
            while to_write_size > 0 {
                let new_entry = Entry {
                    file_id,
                    size: to_write_size.min(first_empty.size),
                    start: first_empty.start,
                };
                self.index_write(new_entry, first_empty.index);
                to_write_size -= new_entry.size;
                first_empty = self.get_first_empty_entry(Some(cache_index)).unwrap();
                cache_index = first_empty.index;
            }
        }
    }

    /// helper
    fn sum_consecutive(a: usize, b: usize) -> usize {
        let n = a.abs_diff(b) + 1;
        (n * (a + b)) / 2
    }

    fn checksum(&self) -> usize {
        self.entries
            .iter()
            .map(|e| e.file_id * Self::sum_consecutive(e.start, e.start + e.size - 1))
            .sum()
    }

    fn get_entry_from_file_id(&self, file_id: usize) -> Option<(usize, Entry)> {
        self.entries
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, e)| e.file_id == file_id)
            .map(|(i, e)| (i, *e))
    }

    fn unfragmented_reorganize(&mut self) {
        // assume last entry is max file id
        let mut to_move_file_id = self.entries.last().unwrap().file_id;
        while to_move_file_id > 0 {
            // to move
            let (entry_idx, entry) = self.get_entry_from_file_id(to_move_file_id).unwrap();

            // for next file
            to_move_file_id -= 1;

            if let Some(empty) = self.get_first_empty_entry_min_size(entry.size) {
                if empty.index > entry_idx {
                    // don't move the entry further in the disk
                    continue;
                }
                // move the entry
                // this doesn't change empty index because `empty.index > entry_idx`
                self.remove_entry(&entry);
                self.index_write(
                    Entry {
                        file_id: entry.file_id,
                        size: entry.size,
                        start: empty.start,
                    },
                    empty.index,
                );
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk_map = DiskMap::from(input);
    disk_map.fragmented_reorganize();
    Some(disk_map.checksum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk_map = DiskMap::from(input);
    disk_map.unfragmented_reorganize();
    Some(disk_map.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
