use pest::Parser;
use pest_derive::Parser;

advent_of_code::solution!(3);

#[derive(Parser)]
#[grammar = "src/bin/03.pest"]
pub struct MulParser;

pub fn part_one(input: &str) -> Option<u32> {
    let Ok(mut parse) = MulParser::parse(Rule::muls, input) else {
        panic!("failed parse");
    };

    parse
        .next() // because we have an `Pairs` iterator over `muls`, we take the first (and only one)
        .unwrap()
        .into_inner() // into_inner means we take the list of `Pair` composing the `muls` which is composed of `(mul | garbage)+`
        .try_fold(0u32, |acc, pair| {
            let mut nums = pair.into_inner(); // we know pair is `mul` and is composed of two inner `num`
            let a = nums.next().unwrap().as_str().parse::<u32>().unwrap();
            let b = nums.next().unwrap().as_str().parse::<u32>().unwrap();
            Some(acc + a * b)
        })
}

pub fn part_two(input: &str) -> Option<u32> {
    let Ok(mut parse) = MulParser::parse(Rule::muls_stateful, input) else {
        panic!("failed parse");
    };
    let (_s, res) = parse
        .next()
        .unwrap()
        .into_inner()
        .try_fold((true, 0u32), |(state, acc), pair| match pair.as_rule() {
            Rule::r#do => Some((true, acc)),
            Rule::dont => Some((false, acc)),
            Rule::mul if state => {
                let mut nums = pair.into_inner();
                let a = nums.next().unwrap().as_str().parse::<u32>().unwrap();
                let b = nums.next().unwrap().as_str().parse::<u32>().unwrap();
                Some((state, acc + a * b))
            }
            _ => Some((state, acc)),
        })
        .unwrap();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1389749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(918285));
    }
}
