use anyhow::anyhow;
use std::fs::File;
use std::io::{self, BufRead};

fn item_priority(item: &char) -> anyhow::Result<u8> {
    let priority: u8 = match item {
        'a'..='z' => *item as u8 - 'a' as u8 + 1,
        'A'..='Z' => *item as u8 - 'A' as u8 + 27,
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
