use iterchunks::IterArrayChunks;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::str::Lines;
use std::thread::current;

fn question_1a(lines: Lines) {
    let mut max_calories = 0;
    let mut sum_calories = 0;

    for line in lines {
        if line.is_empty() {
            if sum_calories > max_calories {
                max_calories = sum_calories.clone();
            }
            sum_calories = 0;
        } else {
            let parsed_value = line.parse::<i32>().unwrap();
            sum_calories += parsed_value;
        }
    }

    if sum_calories > max_calories {
        max_calories = sum_calories;
    }

    println!("Max calories: {:?}", max_calories);
}

fn question_1b(lines: Lines) {
    let mut max_calories: Vec<i32> = Vec::new();
    let mut sum_calories = 0;

    for line in lines {
        if line.is_empty() {
            max_calories.push(sum_calories.clone());
            sum_calories = 0;
        } else {
            let parsed_value = line.parse::<i32>().unwrap();
            sum_calories += parsed_value;
        }
    }

    max_calories.push(sum_calories.clone());

    max_calories.sort();
    max_calories.reverse();

    let total_calories = max_calories[0] + max_calories[1] + max_calories[2];
    println!("{:?}", total_calories);
}

fn question_2a(lines: Lines) {
    let mut scores = HashMap::new();
    scores.insert("X".to_string(), 1);
    scores.insert("Y".to_string(), 2);
    scores.insert("Z".to_string(), 3);

    let mut score = 0;

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let them = parts[0];
        let us = parts[1];

        score += scores.get(us).as_deref().unwrap();

        if them == "A" && us == "X" {
            score += 3
        } else if them == "B" && us == "Y" {
            score += 3
        } else if them == "C" && us == "Z" {
            score += 3
        } else if us == "X" && them == "C" {
            score += 6
        } else if us == "Y" && them == "A" {
            score += 6
        } else if us == "Z" && them == "B" {
            score += 6
        }
    }

    println!("{:?}", score);
}

fn question_2b(lines: Lines) {
    let them_scores = vec!["A", "B", "C"];

    let mut score: i32 = 0;

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let them = parts[0];
        let result = parts[1];

        if result == "Y" {
            score += 3;
            let them_score = them_scores.iter().position(|&r| r == them).unwrap() as i32;
            score += them_score + 1;
        } else if result == "Z" {
            score += 6;
            let them_score =
                (them_scores.iter().position(|&r| r == them).unwrap() as i32 + 1).rem_euclid(3);
            score += them_score + 1;
        } else if result == "X" {
            let them_score =
                (them_scores.iter().position(|&r| r == them).unwrap() as i32 - 1).rem_euclid(3);
            score += them_score + 1;
        }
    }

    println!("{:?}", score);
}

fn get_priority(char: &char) -> u32 {
    if char.is_lowercase() {
        //println!("{:?} {:?}", char, (*char as u32) - 96);
        return (*char as u32) - 96;
    } else {
        //println!("{:?} {:?}", char, (*char as u32) - 64 + 26);
        return (*char as u32) - 64 + 26;
    }
}

fn question_3a(lines: Lines) {
    let mut common_chars: Vec<char> = Vec::new();
    for line in lines {
        let sack_length = line.len();
        let (one, two) = line.split_at(sack_length / 2);
        assert!(one.len() == two.len());

        let mut found = false;
        for c1 in one.chars() {
            for c2 in two.chars() {
                if c1 == c2 && !found {
                    println!("{:?} {:?}", c1, line);
                    common_chars.push(c1);
                    found = true;
                }
            }
        }
    }

    let sum = common_chars.iter().map(get_priority).reduce(|a, b| a + b);
    println!("{:?}", sum)
}

fn question_3b(lines: Lines) {
    let mut common_chars: Vec<char> = Vec::new();
    for [line1, line2, line3] in lines.array_chunks() {
        let mut found = false;
        for c1 in line1.chars() {
            for c2 in line2.chars() {
                for c3 in line3.chars() {
                    if c1 == c2 && c1 == c3 && !found {
                        common_chars.push(c1);
                        found = true;
                    }
                }
            }
        }
    }

    let sum = common_chars.iter().map(get_priority).reduce(|a, b| a + b);
    println!("{:?}", sum)
}

fn question_4a(lines: Lines) {
    let mut num_contained = 0;
    for line in lines {
        let (first, second) = line.split_once(",").unwrap();
        let (first_1, first_2) = first
            .split_once("-")
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .unwrap();
        let (second_1, second_2) = second
            .split_once("-")
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .unwrap();

        if (first_1 <= second_1 && second_2 <= first_2) {
            num_contained += 1
        } else if (second_1 <= first_1 && first_2 <= second_2) {
            num_contained += 1
        }
    }

    print!("{:?}", num_contained);
}

fn question_4b(lines: Lines) {
    let mut num_contained = 0;
    for line in lines {
        let (first, second) = line.split_once(",").unwrap();
        let (first_1, first_2) = first
            .split_once("-")
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .unwrap();
        let (second_1, second_2) = second
            .split_once("-")
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .unwrap();

        if (first_2 >= second_1 && second_2 >= first_1) {
            num_contained += 1
        }
    }

    print!("{:?}", num_contained);
}

fn question_5a(lines: Lines) {
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for i in 1..10 {
        stacks.insert(i, Vec::new());
    }
    let mut stacks_parsed = false;
    for line in lines {
        if line.is_empty() {
            for i in 1..10 {
                stacks.entry(i).and_modify(|arr| arr.reverse());
            }
            stacks_parsed = true;
            continue;
        }

        if !stacks_parsed {
            for i in 0..9 {
                let container = line.chars().nth(1 + (4 * i)).unwrap();
                if container.is_alphabetic() {
                    stacks.entry(i + 1).and_modify(|arr| arr.push(container));
                }
            }
        }

        if stacks_parsed {
            let l = line
                .replace("move", "")
                .replace("from", "")
                .replace("to", "");

            let mut instructions = l.split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().unwrap());
            let move_quantity = instructions.next().unwrap();
            let from = instructions.next().unwrap();
            let to = instructions.next().unwrap();

            for _ in 0..move_quantity {
                let container = stacks.get_mut(&from).unwrap().pop().unwrap();
                stacks.entry(to).and_modify(|arr| arr.push(container));
            }
        }
    }

    println!("{:?}", stacks)
}

fn question_5b(lines: Lines) {
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for i in 1..10 {
        stacks.insert(i, Vec::new());
    }
    let mut stacks_parsed = false;
    for line in lines {
        if line.is_empty() {
            for i in 1..10 {
                stacks.entry(i).and_modify(|arr| arr.reverse());
            }
            stacks_parsed = true;
            continue;
        }

        if !stacks_parsed {
            for i in 0..9 {
                let container = line.chars().nth(1 + (4 * i)).unwrap();
                if container.is_alphabetic() {
                    stacks.entry(i + 1).and_modify(|arr| arr.push(container));
                }
            }
        }

        if stacks_parsed {
            let l = line
                .replace("move", "")
                .replace("from", "")
                .replace("to", "");

            let mut instructions = l.split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().unwrap());
            let move_quantity = instructions.next().unwrap();
            let from = instructions.next().unwrap();
            let to = instructions.next().unwrap();

            let stack = stacks.get_mut(&from).unwrap();
            let container = &mut stack[stack.len() - move_quantity..].to_vec();
            stacks.entry(to).and_modify(|arr| arr.append(container));
            for _ in 0..move_quantity {
                stacks.get_mut(&from).unwrap().pop().unwrap();
            }
        }
    }

    println!("{:?}", stacks.get(&1));
    println!("{:?}", stacks.get(&2));
    println!("{:?}", stacks.get(&3));
    println!("{:?}", stacks.get(&4));
    println!("{:?}", stacks.get(&5));
    println!("{:?}", stacks.get(&6));
    println!("{:?}", stacks.get(&7));
    println!("{:?}", stacks.get(&8));
    println!("{:?}", stacks.get(&9));
}

fn question_6a(mut lines: Lines) {
    let line = lines.next().unwrap();
    for i in 0..line.len() {
        let slice = &line[i..i+4];
        let chars: HashSet<char> = HashSet::from_iter(slice.chars().into_iter());
        if chars.len() == 4 {
            println!("{:?}", slice);
            println!("{:?}", i + 4);
            break;
        }
    }
}

fn question_6b(mut lines: Lines) {
    let line = lines.next().unwrap();
    for i in 0..line.len() {
        let slice = &line[i..i+14];
        let chars: HashSet<char> = HashSet::from_iter(slice.chars().into_iter());
        if chars.len() == 14 {
            println!("{:?}", slice);
            println!("{:?}", i + 14);
            break;
        }
    }
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

#[derive(Clone)]
struct Directory {
    name: String,
    size: i32,
    files: Vec<(i32, String)>,
    parent: Option<Box<Directory>>,
    children: Vec<Box<Directory>>,
}

fn get_dir_size(directory: &Directory) -> Vec<(String, i32)> {
    let files_in_this_dir = directory.files.clone().into_iter().map(|(b, _)| b).reduce(|a, b| a + b).unwrap();
    let children_size = directory.children.clone().into_iter().map(|child| get_dir_size(&unbox(child))).flatten();
    let mut list:  Vec<_> = children_size.collect();
    let mut sizes: Vec<(String, i32)> = Vec::new();
    sizes.append(&mut list);

    let this_dir:(String, i32) = (directory.name.clone(), files_in_this_dir + list.into_iter().map(|(_, size)| size).reduce(|x, y| x + y).unwrap());
    sizes.push(this_dir);

    return sizes;
}

fn question_7a(lines: Lines) {
    let first_dir = Directory {
                name: "/".to_string(),
                size: 0,
                files: Vec::new(),
                children: Vec::new(),
                parent: Option::None,
            };

    let mut current_dir = &first_dir;
    for line in lines {
        if line == ("$ cd /") {
            continue;
        }
        if line == "$ cd .." {
            let x = current_dir.parent.as_ref().unwrap();
            current_dir = &x;
            continue;
        }

        if line.starts_with("$ cd") {
            let new_dir = Directory {
                name: line.split("$ cd ").collect(),
                size: 0,
                files: Vec::new(),
                children: Vec::new(),
                parent: Option::None,
            };


            current_dir = &new_dir;
            continue;
        }

        if line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("dir") {
            continue;
        }

        let (bytes, filename) = line.split_once(" ").unwrap();
        current_dir.files.push((bytes.parse::<i32>().unwrap(), filename.to_string()));
    }

    let result = get_dir_size(&first_dir);
    println!("{:?}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    question_7a(lines);
}
