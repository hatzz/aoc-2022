fn main() -> anyhow::Result<()> {
    let file = std::fs::read_to_string("../calories.txt")?;

    let mut elves = file
        .trim()
        .split("\n\n")
        .map(|elf| elf.split("\n"))
        .map(|elf| {
            elf.map(|calories| usize::from_str_radix(calories, 10).unwrap())
                .collect()
        })
        .map(|elf: Vec<usize>| elf.iter().sum::<usize>())
        .collect::<Vec<usize>>();

    elves.sort();

    println!(
        "{}, {:?}",
        elves[elves.len() - 1],
        elves
            .get(elves.len() - 3..elves.len())
            .unwrap()
            .iter()
            .sum::<usize>()
    );

    Ok(())
}
