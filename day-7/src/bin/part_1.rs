#[derive(Debug, Clone)]
struct File<'a> {
    name: &'a str,
    size: usize,
}
#[derive(Debug, Clone)]
struct Directory<'a> {
    name: String,
    files: Vec<File<'a>>,
    size: usize,
    children: Vec<String>,
}
impl<'a> Directory<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            files: vec![],
            size: 0,
            children: vec![],
        }
    }
}
fn main() {
    let input = std::fs::read_to_string("src/input_1.prod").unwrap();
    let mut made_dirs: std::collections::HashSet<String> = std::collections::HashSet::new();
    made_dirs.insert("/".to_string());
    let mut filepath = "/".to_string();
    let mut dirs: Vec<Directory> = vec![Directory::new("/".to_string())];

    for line in input.lines().skip(2) {
        println!("Line: {:?}", line);
        if line.starts_with("$ cd") {
            if line == "$ cd .." {
                filepath = up_dir(filepath);
            } else {
                let dir_name = line.split(" ").last().unwrap();
                let temp_path = filepath.clone() + dir_name + &"/".to_string();
                if !made_dirs.contains(&temp_path) {
                    made_dirs.insert(temp_path.clone());
                    let new_dir = Directory::new(temp_path.clone());
                    println!("{:?} {:?} {:?}", dirs, temp_path, up_dir(temp_path.clone()));
                    dirs.iter_mut()
                        .find(|d| d.name == up_dir(temp_path.clone()))
                        .expect("should have already found parent")
                        .children
                        .push(temp_path.clone());
                    dirs.push(new_dir);
                }
                filepath = filepath.clone() + dir_name + &"/".to_string();
            }
        } else if line.starts_with("dir") || line.starts_with("$ ls") {
            continue;
        } else {
            let (size, name) = line.split_once(" ").expect("aoc input to be correct");
            let file = File {
                size: size
                    .parse()
                    .expect("aoc input to be correct for the file sizes"),
                name,
            };
            let dir_to_edit = &mut dirs
                .iter_mut()
                .find(|dir| dir.name == filepath)
                .expect("this dir should have been found before finding a file in it");
            dir_to_edit.files.push(file);
            dir_to_edit.size += size.parse::<usize>().unwrap();
            let mut node_holder = filepath.clone();
            loop {
                let next_dir = up_dir(node_holder.clone());
                if next_dir == "" {
                    break;
                }
                dirs.iter_mut().find(|d| d.name == next_dir).unwrap().size +=
                    size.parse::<usize>().unwrap();
                node_holder = next_dir;
            }
        }
    }
    let sum = dirs
        .iter()
        .filter(|d| d.size <= 100000)
        .map(|d| d.size)
        .sum::<usize>();
    for dir in dirs {
        println!("{:?}\n", dir);
    }
    println!("{:?}\n", sum);
}
fn up_dir(curr_dir: String) -> String {
    let mut holder = curr_dir.chars().collect::<Vec<char>>();
    holder.pop();
    if holder.len() > 0 {
        while holder[holder.len() - 1] != '/' {
            holder.pop();
        }
    }
    holder.into_iter().collect()
}
