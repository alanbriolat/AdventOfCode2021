use super::prelude::*;
use crate::util::read_file;

#[derive(Debug, thiserror::Error)]
enum SyntaxError {
    #[error("syntax error: illegal character: {0}")]
    IllegalCharacter(char),
    #[error("syntax error: incomplete")]
    Incomplete,
}

impl SyntaxError {
    fn score(&self) -> u64 {
        match self {
            SyntaxError::IllegalCharacter(')') => 3,
            SyntaxError::IllegalCharacter(']') => 57,
            SyntaxError::IllegalCharacter('}') => 1197,
            SyntaxError::IllegalCharacter('>') => 25137,
            _ => 0,
        }
    }
}

fn check_syntax(line: &str) -> Result<(), SyntaxError> {
    let mut stack: Vec<char> = Vec::with_capacity(line.len() / 2);
    for c in line.chars() {
        match (stack.last().copied(), c) {
            (_, '(') | (_, '[') | (_, '{') | (_, '<') => {
                stack.push(c);
            }
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                stack.pop();
            }
            _ => {
                return Err(SyntaxError::IllegalCharacter(c));
            }
        }
    }
    if !stack.is_empty() {
        Err(SyntaxError::Incomplete)
    } else {
        Ok(())
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let total: u64 = reader
        .lines()
        .map(|line| match check_syntax(line.unwrap().as_str()) {
            Err(err @ SyntaxError::IllegalCharacter(_)) => err.score(),
            _ => 0,
        })
        .sum();
    Ok(total.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    todo!()
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day10_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day10_input.txt")));
    runner
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::util::read_str;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(read_str(indoc! {"\
                [({(<(())[]>[[{[]{<()<>>
                [(()[<>])]({[<{<<[]>>(
                {([(<{}[<>[]}>{[]{[(<()>
                (((({<>}<{<{<>}{[]{[]{}
                [[<[([]))<([[{}[[()]]]
                [{[{({}]{}}([{[{{{}}([]
                {<[[]]>}<{[{[{[]{()[[[]
                [<(<(<(<{}))><([]([]()
                <{([([[(<>()){}]>(<<{{
                <{([{{}}[<[[[<>{}]]]>[]]
            "}))
            .unwrap(),
            "26397"
        );
        assert_eq!(part1(read_file("data/day10_input.txt")).unwrap(), "318099");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(read_str(indoc! {"\
                ???
            "}))
            .unwrap(),
            "???"
        );
        assert_eq!(part2(read_file("data/day10_input.txt")).unwrap(), "???");
    }
}
