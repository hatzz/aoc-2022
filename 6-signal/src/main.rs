use std::collections::BTreeSet;

fn find_unique_sequence(stream: &String, num_characters: usize) -> Option<usize> {
    for i in 0..stream.len() {
        let mut set: BTreeSet<char> = BTreeSet::new();
        let slice = &stream[i..i + num_characters];
        set.extend(slice.chars());

        if set.len() == num_characters {
            return Some(i + num_characters)
        }
    }

    None
}

fn main() -> anyhow::Result<()> {
    let stream = std::fs::read_to_string("./datastream.txt")?;

    let start_of_packet = find_unique_sequence(&stream, 4);
    let start_of_message = find_unique_sequence(&stream, 14);

    println!("Packet: {:?}", start_of_packet);
    println!("Message: {:?}", start_of_message);

    Ok(())
}
