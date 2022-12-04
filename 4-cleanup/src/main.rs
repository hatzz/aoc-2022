use std::fs::File;
use std::io::{self, BufRead};

type SectionRange = (u8, u8);
type SectionPair = (SectionRange, SectionRange);

fn get_section_range(section: &str) -> anyhow::Result<SectionRange> {
    let sections: Vec<&str> = section.split("-").collect();

    Ok((
        u8::from_str_radix(sections[0], 10)?,
        u8::from_str_radix(sections[1], 10)?,
    ))
}

fn get_section_pair(line: String) -> anyhow::Result<SectionPair> {
    let pairs: Vec<&str> = line.split(",").collect();

    Ok((get_section_range(pairs[0])?, get_section_range(pairs[1])?))
}

fn fully_contains(pairs: &SectionPair) -> bool {
    if pairs.0 .0 >= pairs.1 .0 && pairs.0 .1 <= pairs.1 .1 {
        return true;
    }

    if pairs.0 .0 <= pairs.1 .0 && pairs.0 .1 >= pairs.1 .1 {
        return true;
    }

    return false;
}

fn overlap(pairs: &SectionPair) -> bool {
    let second = pairs.1 .0..=pairs.1 .1;

    for i in pairs.0 .0..=pairs.0 .1 {
        if second.contains(&i) {
            return true;
        }
    }

    return false;
}

fn get_fully_contained() -> anyhow::Result<usize> {
    let file = File::open("cleaning-pairs.txt")?;

    let mut sum: usize = 0;

    for line in io::BufReader::new(file).lines() {
        let pair = get_section_pair(line?)?;

        if fully_contains(&pair) {
            sum += 1
        }
    }

    Ok(sum)
}

fn get_overlapping() -> anyhow::Result<usize> {
    let file = File::open("cleaning-pairs.txt")?;

    let mut sum: usize = 0;

    for line in io::BufReader::new(file).lines() {
        let pair = get_section_pair(line?)?;

        if overlap(&pair) {
            sum += 1
        }
    }

    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    println!("Fully contained: {}", get_fully_contained()?);
    println!("Overlapping: {}", get_overlapping()?);

    Ok(())
}
