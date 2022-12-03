use anyhow::anyhow;
use std::fs::File;
use std::io::{self, BufRead};

fn item_priority(item: &char) -> anyhow::Result<u8> {
    let priority: u8 = match item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => return Err(anyhow!("Invalid item")),
    };

    return Ok(priority);
}

fn split_in_half<'a>(rucksack: &'a str) -> (&'a str, &'a str) {
    let half = rucksack.len() / 2;

    (&rucksack[..half], &rucksack[half..])
}

fn find_duplicate_item(rucksack: (&str, &str)) -> Option<char> {
    rucksack.0.chars().find(|&c| {
        rucksack.1.contains(c)
    })
}

fn find_shared_item(rucksacks: (String, String, String)) -> Option<char> {
    rucksacks.0.chars().find(|&c| {
        rucksacks.1.contains(c) && rucksacks.2.contains(c)
    })
}

fn sum_prioriy(file: File) -> anyhow::Result<usize> {
    let mut sum: usize = 0;

    for line in io::BufReader::new(file).lines() {
        let rucksack = line?;
        let split = split_in_half(rucksack.as_str());
        let duplicate = find_duplicate_item(split);

        if let Some(duplicate) = duplicate {
            sum += item_priority(&duplicate)? as usize
        }
    }

    Ok(sum)
}

type IoResult = Result<String, io::Error>;

fn next_three(lines: &mut io::Lines<io::BufReader<File>>) -> Option<(IoResult, IoResult, IoResult)> {
        let one = lines.next()?;
        let two = lines.next()?;
        let three = lines.next()?;

        Some((one, two, three))
}

fn group_rucksacks(file: File) -> anyhow::Result<usize> {
    let mut sum: usize = 0;
    let mut lines = io::BufReader::new(file).lines();

    while let Some(rucksacks) = next_three(&mut lines) {
        if let Some(shared) = find_shared_item((rucksacks.0?, rucksacks.1?, rucksacks.2?)) {
            sum += item_priority(&shared)? as usize;
        }
    }

    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    let sum = sum_prioriy(File::open("rucksacks.txt")?)?;
    let grouped_sum = group_rucksacks(File::open("rucksacks.txt")?)?;

    println!("{}", sum);
    println!("{}", grouped_sum);

    Ok(())
}
