#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Dir {
    name: String,
    dirs: Vec<Dir>,
    files: Vec<File>,
}

fn get<'a>(dir: &'a mut Dir, path: &[String]) -> Option<&'a mut Dir> {
    if path.len() == 0 {
        return Some(dir);
    }

    let dir_name = path.first()?;
    if path.len() == 1 {
        let found_dir = dir.dirs.iter_mut().find(|dir| &dir.name == dir_name);
        return found_dir;
    } else {
        let dir = dir.dirs.iter_mut().find(|dir| &dir.name == dir_name)?;
        return get(dir, &path[1..path.len()]);
    }
}

fn dir_size(dir: &Dir) -> usize {
    let file_sum: usize = dir.files.iter().map(|file| file.size).sum();
    let dir_sum: usize = dir.dirs.iter().map(dir_size).sum();

    return dir_sum + file_sum
}

fn dir_sizes(dir: &Dir) -> Vec<usize> {
    let size = dir_size(dir);
    let mut sub_dir_sizes: Vec<Vec<usize>> = dir.dirs.iter().map(|dir| dir_sizes(dir)).collect();

    sub_dir_sizes.push(vec![size]);

    sub_dir_sizes.into_iter().flatten().collect()
}

fn main() -> anyhow::Result<()> {
    let commands = std::fs::read_to_string("./commands.txt")?;
    let commands = commands.split("\n");
    let mut root = Dir {
        name: String::from("/"),
        dirs: Vec::new(),
        files: Vec::new(),
    };

    let mut current_directory: Vec<String> = Vec::new();

    for command in commands {
        if command.starts_with("$ cd") {
            match command {
                "$ cd /" => {
                    current_directory.clear();
                }
                "$ cd .." => {
                    current_directory.pop();
                }
                _ => {
                    let dir_name = command.replace("$ cd ", "");
                    current_directory.push(dir_name);
                }
            }

            continue;
        }

        if command.starts_with("dir") {
            let dir_name = command.replace("dir ", "");
            let dir = Dir {
                name: dir_name,
                files: Vec::new(),
                dirs: Vec::new(),
            };

            println!("{:#?}", dir);

            println!("{:?}", current_directory);

            let current_dir = get(&mut root, &current_directory[..])
                .ok_or(anyhow::anyhow!("Current directory does not exist"))?;

            current_dir.dirs.push(dir);

            continue;
        }

        if command == "$ ls" {
            continue;
        }

        if command == "" {
            continue;
        }

        println!("{}", command);
        let file: Vec<&str> = command.split(" ").collect();
        let size = usize::from_str_radix(file[0], 10)?;
        let file_name = file[1];

        let current_dir = get(&mut root, &current_directory[..])
            .ok_or(anyhow::anyhow!("Current directory does not exist"))?;

        current_dir.files.push(File {
            name: file_name.to_string(),
            size
        })
    }

    println!("{:#?}", root);

    let sizes: Vec<usize> = dir_sizes(&root);

    let used_space: usize = dir_size(&root);
    println!("Used space: {}", used_space);
    let max_space = 70000000;
    println!("Max space: {}", max_space);
    let needed_space = 30000000;
    println!("Needed space: {}", needed_space);
    let free_space = max_space - used_space;
    println!("Free space: {}", free_space);
    let space_to_delete = needed_space - free_space;
    println!("Space to delete: {}", space_to_delete);

    let mut foo: Vec<&usize> = sizes.iter().filter(|size| size >= &&space_to_delete).collect();
    foo.sort();
    println!("{:?}", foo);
    println!("Smallest dir to delete: {}", foo[0]);

    let sum_less_than_100k: usize = sizes.iter().filter(|size| size < &&100000).sum();
    println!("{}", sum_less_than_100k);

    Ok(())
}
