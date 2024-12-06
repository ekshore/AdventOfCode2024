use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    println!("Merry Christmas");

    let (mut list_one, mut list_two) = load_file(String::from("./day-one/input/first_input.txt"));

    list_one.sort();
    list_two.sort();

    let list_distance = find_distance(list_one.clone(), list_two.clone());
    let list_similarity_score = find_similaritiy_score(list_one, list_two);
    println!("List distance: {}", list_distance);
    println!("List similarity score: {}", list_similarity_score);
}

fn find_similaritiy_score(list_one: Vec<i32>, list_two: Vec<i32>) -> i32 {
    let list_two_summary: HashMap<i32, i32> =
        list_two.into_iter().fold(HashMap::new(), |mut acc, val| {
            acc.insert(val, acc.get(&val).unwrap_or(&0) + 1);
            acc
        });

    list_one.into_iter().fold(0, |acc, val| {
        acc + val * list_two_summary.get(&val).unwrap_or(&0)
    })
}

fn find_distance(list_one: Vec<i32>, list_two: Vec<i32>) -> i32 {
    list_one.iter().enumerate().fold(0, |acc, (idx, val)| {
        let dist = if val > list_two.get(idx).unwrap() {
            val - list_two.get(idx).unwrap()
        } else {
            list_two.get(idx).unwrap() - val
        };
        acc + dist
    })
}

fn load_file(file_path: String) -> (Vec<i32>, Vec<i32>) {
    let file_path = Path::new(&file_path);
    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];

    if let Ok(lines) = fs::read_to_string(file_path) {
        let lines = lines.lines();
        for line in lines {
            if line.is_empty() {
                break;
            }
            let mut line = line.split("   ");
            list_one.push(line.next().unwrap().parse().unwrap());
            list_two.push(line.next().unwrap().parse().unwrap());
        }
    }
    (list_one, list_two)
}

#[cfg(test)]
mod day_one {
    use crate::find_similaritiy_score;

    #[test]
    fn test_similarity() {
        let list_one: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let list_two: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        let similarity_score = find_similaritiy_score(list_one, list_two);

        assert_eq!(similarity_score, 31);
    }
}
