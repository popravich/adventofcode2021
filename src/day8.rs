use std::collections::HashMap;

/*
static SEGMENTS_TO_UNIQUE_NUMBERS: [&[u8]; 7] = [
/* 1 */ &[],
/* 2 */ &[1],
/* 3 */ &[7],
/* 4 */ &[4],
/* 5 */ &[2, 3, 5],
/* 6 */ &[0, 6, 9],
/* 7 */ &[8],
];
static SIGNALS_MATRIX: [[u8; 10]; 7] = [
/*       0, 1, 2, 3, 4, 5, 6, 7, 8, 9 */
/* 1 */ [1, 0, 1, 1, 0, 1, 1, 1, 1, 1],
/* 2 */ [1, 0, 0, 0, 1, 1, 1, 0, 1, 1],
/* 3 */ [1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
/* 4 */ [0, 0, 1, 1, 1, 1, 1, 0, 1, 1],
/* 5 */ [1, 0, 1, 0, 0, 0, 1, 0, 1, 0],
/* 6 */ [1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
/* 7 */ [1, 0, 1, 1, 0, 1, 1, 0, 1, 1],
];

static SEGMENTS_BITMASKS: [u8; 10] = [
/* \ seg: 1234567 */
/* 0 */ 0b1110111,
/* 1 */ 0b0010010,
/* 2 */ 0b1011101,
/* 3 */ 0b1011011,
/* 4 */ 0b0111010,
/* 5 */ 0b1101011,
/* 6 */ 0b1101111,
/* 7 */ 0b1010010,
/* 8 */ 0b1111111,
/* 9 */ 0b1111011,
];
*/

/// Idea is as follows:
/// we count how many time each segment is used among all digits,
/// we can observe that there are segments with unique frequencies
/// and some have similar.
/// We buid second frequencies table for input data and map segments
/// to correct one.
/// To resolve ambiguities we look at pattern for 1 and 4.
static SEGMENTS_FREQUENCIES: [(char, u8); 7] = [
/* 1 */ ('a', 8),
/* 2 */ ('b', 6),
/* 3 */ ('c', 8),
/* 4 */ ('d', 7),
/* 5 */ ('e', 4),
/* 6 */ ('f', 9),
/* 7 */ ('g', 7),
];


pub fn main(input: &str) -> Result<(usize, usize), String> {

    let data = parse_input(input);

    let part1 = data.iter()
        .map(|(_, value)| {
            value.iter().filter(|s| {
                s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7
            })
        })
        .flatten()
        .count();

    let mut segments_frequencies: HashMap<u8, String> = HashMap::new();
    for (letter, freq) in SEGMENTS_FREQUENCIES {
        segments_frequencies.entry(freq).or_default().push(letter);
    }
    let mut part2 = 0;
    for (indication_patterns, indication) in &data {
        let mut transcoding_table = TranscodingTable::default();
        let mut frequencies: HashMap<char, u8> = HashMap::new();

        let mut pattern_for_one = String::new();
        let mut pattern_for_four = String::new();

        // Collect patterns into struct & count segments frequencies;
        for pattern in indication_patterns {
            match pattern.len() {
                2 => {
                    assert!(pattern_for_one.is_empty());
                    pattern_for_one.push_str(pattern);
                }
                4 => {
                    assert!(pattern_for_four.is_empty());
                    pattern_for_four.push_str(pattern);
                }
                _ => ()
            }
            for c in pattern.chars() {
                *frequencies.entry(c).or_default() += 1;
            }
        }

        // resolve segments

        for (segment, freq) in frequencies.iter() {
            let variants = segments_frequencies.get(freq).expect("Unordinary frequency");
            transcoding_table.add(*segment, variants);
        }
        // resolve few ambigious segment with equal frequencies
        // lookup for the segments of 1;
        let segment_c = pattern_for_one
            .chars()
            .filter(|c| transcoding_table.transcode_char(c).len() > 1)
            .next()
            .unwrap();
        transcoding_table.resolve_letters_ac(segment_c);

        // lookup for the segments of 4;
        let segment_d = pattern_for_four
            .chars()
            .filter(|c| transcoding_table.transcode_char(c).len() > 1)
            .next()
            .unwrap();
        transcoding_table.resolve_letters_dg(segment_d);
        assert!(!transcoding_table.is_ambigiuos());

        for (i, num) in indication.iter().rev().enumerate() {
            let digit = transcoding_table.transcode_to_u8(&num) as usize;
            part2 += digit * 10usize.pow(i as u32);
        }
    }

    Ok((part1, part2))
}

pub fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input.lines()
        .map(|line| {
        let (a, b) = line.split_once(" | ").expect("invalid data");
        (
            a.split(' ').map(|s| s.to_string()).collect(),
            b.split(' ').map(|s| s.to_string()).collect(),
        )})
        .collect()
}


#[derive(Debug, Default)]
pub struct TranscodingTable(HashMap<char, String>);

impl TranscodingTable {
    pub fn add(&mut self, c: char, variants: &str) {
        self.0
            .entry(c)
            .or_default()
            .push_str(variants);
    }

    pub fn transcode_char(&self, c: &char) -> &str {
        self.0.get(c).unwrap()
    }

    pub fn transcode_str(&self, val: &str) -> String {
        assert!(!self.is_ambigiuos());
        let mut result = Vec::with_capacity(val.len());
        for c in val.chars() {
            result.extend(self.transcode_char(&c).chars());
        }
        result.sort();
        result.iter().collect()
    }
    pub fn transcode_to_u8(&self, val: &str) -> u8 {
        match self.transcode_str(val).as_str() {
            "abcefg" => 0,
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            x => unreachable!("unexpected pattern: {}", x),
        }

    }

    pub fn resolve_letters_ac(&mut self, c: char) {
        self.0.insert(c, "c".to_string());
        for v in self.0.values_mut() {
            if v.as_str() == "ac" {
                *v = "a".to_string();
            }
        }
    }

    pub fn resolve_letters_dg(&mut self, c: char) {
        self.0.insert(c, "d".to_string());
        for v in self.0.values_mut() {
            if v.as_str() == "dg" {
                *v = "g".to_string();
            }
        }
    }

    pub fn is_ambigiuos(&self) -> bool {
        self.0.values().any(|s| s.len() > 1)
    }
}


#[cfg(test)]
mod test {
    use crate::day8::main;

    static INPUT: &str = concat!(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    );

   #[test]
   fn part1() {
        let (part1, _) = main(INPUT).unwrap();
        assert_eq!(part1, 26);
   }

   #[test]
   fn part2() {
        let (_, part2) = main(INPUT).unwrap();
        assert_eq!(part2, 61229);
   }
}
