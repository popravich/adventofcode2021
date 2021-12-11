use std::collections::VecDeque;


pub fn main(input: &str) -> Result<(usize, usize), String> {

    let mut score = 0;
    let mut incomplete_scores = Vec::new();
    for line in input.lines() {
        match analyze(line) {
            Status::Corrupted(c) => {
                score += corrupted_score(c);
            }
            Status::Incomplete(stack) => {
                let score = stack
                    .iter()
                    .rev()
                    .map(|&c| incomplete_score(c))
                    .fold(0, |total, score| total * 5 + score);
                let idx = incomplete_scores.binary_search(&score)
                    .or_else::<(), _>(|idx| Ok(idx)).unwrap();
                incomplete_scores.insert(idx, score);
            }
            Status::Ok => {}
        }
    }
    let part2 = incomplete_scores[incomplete_scores.len() / 2];

    Ok((score, part2))
}

pub fn analyze(code: &str) -> Status {
    let mut stack = VecDeque::with_capacity(code.len() / 2);
    for c in code.chars() {
        if is_start(c) {
            stack.push_back(get_end(c));
        } else {
            match stack.pop_back() {
                Some(x) if c != x => return Status::Corrupted(c),
                None => return Status::Incomplete(stack),
                _ => {},
            }
        }
    }
    if stack.is_empty() {
        Status::Ok
    } else {
        Status::Incomplete(stack)
    }
}

pub fn is_start(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

pub fn get_end(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("unexpected character: {}", c),
    }
}

pub fn corrupted_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!("unexpected character: {}", c),
    }
}

pub fn incomplete_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!("unexpected character: {}", c),
    }
}

#[derive(Debug)]
pub enum Status {
    Ok,
    Corrupted(char),
    Incomplete(VecDeque<char>),
}


#[cfg(test)]
mod test {
    use crate::day10::main;

    static DATA: &str = concat!(
        "[({(<(())[]>[[{[]{<()<>>\n",
        "[(()[<>])]({[<{<<[]>>(\n",
        "{([(<{}[<>[]}>{[]{[(<()>\n",
        "(((({<>}<{<{<>}{[]{[]{}\n",
        "[[<[([]))<([[{}[[()]]]\n",
        "[{[{({}]{}}([{[{{{}}([]\n",
        "{<[[]]>}<{[{[{[]{()[[[]\n",
        "[<(<(<(<{}))><([]([]()\n",
        "<{([([[(<>()){}]>(<<{{\n",
        "<{([{{}}[<[[[<>{}]]]>[]]\n",
    );

    #[test]
    fn test_solution() {
        let (p1, p2) = main(DATA).expect("Invalid data");
        assert_eq!(p1, 26397);
        assert_eq!(p2, 288957);
    }
}
