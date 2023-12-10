
use std::fs;
use regex::{Regex};
use im::{vector};
use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    println!("Decoded sum, finding all digits: {}", sum(&contents, find_all_ints));

    println!("Decoded sum, finding all digits and words: {}", sum(&contents, find_all_ints2));

}

fn sum(contents: &String, find_integers: impl Fn(&str) -> Vec<u32>) -> u32 {
    let mut accumulate: u32 = 0;
    for line in contents.split('\n') {
        // print!("text: {}", line);
        if ! line.is_empty() {
            let val = decode(line, &find_integers);
            // print!(" -> {}", val);
            accumulate += val;
        }
        // println!("");
    }

    return accumulate;
}

fn decode(line: &str, find_integers: impl Fn(&str) -> Vec<u32>) -> u32 {

    let all_ints = find_integers(line);
    let digit1 = all_ints.first().unwrap();
    let digit2 = all_ints.last().unwrap();

    //println!("First int is {:?}. Last int is {:?}", digit1, digit2);

    return digit1 * 10 + digit2;
}

fn find_all_ints(line: &str) -> Vec<u32> {
    let re = Regex::new(r"([0-9])").unwrap();
    let mut results = vec![];
    for (_, [digit]) in re.captures_iter(line).map(|c| c.extract()) {
        results.push(digit.parse::<u32>().unwrap());
    }

    return results;
}

fn find_all_ints2(line: &str) -> Vec<u32> {
    let digit_words = vector!["zero","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut results: Vec<u32> = vec![];

    let mut digit_map = BTreeMap::new();

    // find all digits first
    let all_digits = r"0123456789";
    let mut i = 0;
    for c in line.chars() {
        let digit = all_digits.find(c);
        if digit.is_some() {
            digit_map.insert(i, digit.unwrap() as u32);
        }
        i += 1;
    }

    // next, find the first and last digit word
    let mut digit_value: u32 = 0;
    for dw in digit_words {
        let first_word = line.find(dw);
        if first_word.is_some() {
            digit_map.insert(first_word.unwrap(), digit_value);
        }
        let last_word = line.rfind(dw);   
        if last_word.is_some() {
            digit_map.insert(last_word.unwrap(), digit_value);
        }
        digit_value += 1;
    }

    for k in digit_map {
        results.push(k.1);
    }

    return results;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_all_ints() {
        let v1 = find_all_ints("pqr3stu8vwx");
        assert_eq!(v1.len(), 2);
        assert_eq!(v1[0], 3);
        assert_eq!(v1[1], 8);

        let v2 = find_all_ints("a1b2c3d4e5f");
        assert_eq!(v2.len(), 5);
        assert_eq!(v2[0], 1);
        assert_eq!(v2[1], 2);
        assert_eq!(v2[2], 3);
        assert_eq!(v2[3], 4);
        assert_eq!(v2[4], 5);

        let v3 = find_all_ints("treb7uchet");
        assert_eq!(v3.len(), 1);
        assert_eq!(v3[0], 7);
    }

    #[test]
    fn test_find_all_ints2() {
        let v1 = find_all_ints2("pqr3stu8vwx");
        assert_eq!(v1.len(), 2);
        assert_eq!(v1[0], 3);
        assert_eq!(v1[1], 8);

        let v2 = find_all_ints2("a1b2c3d4e5f");
        assert_eq!(v2.len(), 5);
        assert_eq!(v2[0], 1);
        assert_eq!(v2[1], 2);
        assert_eq!(v2[2], 3);
        assert_eq!(v2[3], 4);
        assert_eq!(v2[4], 5);

        let v3 = find_all_ints2("treb7uchet");
        assert_eq!(v3.len(), 1);
        assert_eq!(v3[0], 7);

        let mut v4 = find_all_ints2("two1nine");
        assert_eq!(v4.len(), 3);
        assert_eq!(v4[0], 2);
        assert_eq!(v4[1], 1);
        assert_eq!(v4[2], 9);

        v4 = find_all_ints2("eightwothree");
        assert_eq!(v4.len(), 3);
        assert_eq!(v4[0], 8);
        assert_eq!(v4[1], 2);
        assert_eq!(v4[2], 3);

        v4 = find_all_ints2("abcone2threexyz");
        assert_eq!(v4.len(), 3);
        assert_eq!(v4[0], 1);
        assert_eq!(v4[1], 2);
        assert_eq!(v4[2], 3);

        v4 = find_all_ints2("xtwone3four");
        assert_eq!(v4.len(), 4);
        assert_eq!(v4[0], 2);
        assert_eq!(v4[1], 1);
        assert_eq!(v4[2], 3);
        assert_eq!(v4[3], 4);

        v4 = find_all_ints2("4nineeightseven2");
        assert_eq!(v4.len(), 5);
        assert_eq!(v4[0], 4);
        assert_eq!(v4[1], 9);
        assert_eq!(v4[2], 8);
        assert_eq!(v4[3], 7);
        assert_eq!(v4[4], 2);

        v4 = find_all_ints2("zoneight234");
        assert_eq!(v4.len(), 5);
        assert_eq!(v4[0], 1);
        assert_eq!(v4[1], 8);
        assert_eq!(v4[2], 2);
        assert_eq!(v4[3], 3);
        assert_eq!(v4[4], 4);

        v4 = find_all_ints2("7pqrstsixteen");
        assert_eq!(v4.len(), 2);
        assert_eq!(v4[0], 7);
        assert_eq!(v4[1], 6);

    }

    #[test]
    fn test_decode() {

        let sample_text = "1abc2";

        assert_eq!(decode(&sample_text, find_all_ints), 12);
    }

    #[test]
    fn test_sum() {

        let sample_text = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(sum(&sample_text.to_string(), find_all_ints), 142);

    }

    #[test]
    fn test_words_and_digits() {
        let sample_text = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(sum(&sample_text.to_string(), find_all_ints2), 281);
    }

}