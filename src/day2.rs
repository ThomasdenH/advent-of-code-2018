use aoc_runner_derive::*;

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|a| a.to_owned()).collect()
}

fn checksum(id: &str) -> (bool, bool) {
    let mut frequencies = [0; 26];
    for char in id.chars() {
        let index = (char.to_digit(36).unwrap() - 10) as usize;
        frequencies[index] += 1;
    }
    let contains2 = frequencies.contains(&2);
    let contains3 = frequencies.contains(&3);
    (contains2, contains3)
}

fn to_binary_digit(b: bool) -> u32 {
    if b {
        1
    } else {
        0
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[String]) -> u32 {
    let (a, b) = input
        .iter()
        .map(|a| checksum(a))
        .map(|(a, b)| (to_binary_digit(a), to_binary_digit(b)))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    a * b
}

#[aoc(day2, part2)]
pub fn part2(input: &[String]) -> String {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                break;
            }

            let string1 = &input[i];
            let string2 = &input[j];

            if string1
                .chars()
                .zip(string2.chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
            {
                return string1
                    .chars()
                    .zip(string2.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _b)| a)
                    .collect::<String>();
            }
        }
    }
    panic!("Solution not found!");
}

#[test]
fn test_part1_example() {
    let input_string = "abcdef\n\
                        bababc\n\
                        abbcde\n\
                        abcccd\n\
                        aabcdd\n\
                        abcdee\n\
                        ababab\n";
    let input = generator(&input_string);
    let result = part1(&input);
    assert_eq!(result, 12);
}

#[test]
fn test_part1() {
    let input_string = crate::util::read_file_to_string("./input/2018/day2.txt");
    let input = generator(&input_string);
    let result = part1(&input);
    assert_eq!(result, 7776);
}

#[test]
fn test_part2_example() {
    let input_string = "abcde\n\
                        fghij\n\
                        klmno\n\
                        pqrst\n\
                        fguij\n\
                        axcye\n\
                        wvxyz\n";
    let input = generator(&input_string);
    let result = part2(&input);
    assert_eq!(result, "fgij");
}

#[test]
fn test_part2() {
    let input_string = crate::util::read_file_to_string("./input/2018/day2.txt");
    let input = generator(&input_string);
    let result = part2(&input);
    assert_eq!(result, "wlkigsqyfecjqqmnxaktdrhbz");
}
