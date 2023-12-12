use std::fs;

fn main() {
    let mut content: Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string("./input.txt")
        .expect("file not found")
        .lines()
    {
        let buf = line.chars().collect::<Vec<char>>();
        content.push(buf);
    }

    let bound = (content.len() as i32, content[0].len() as i32);
    let mut schemas: Vec<i32> = Vec::new();
    let mut current_num: String = String::new();
    let mut is_num: bool = false;

    for (i, row) in content.iter().enumerate() {
        let mut row_indexes: Vec<usize> = Vec::new();
        for (j, _col) in row.iter().enumerate() {
            if content[i][j].is_numeric() {
                row_indexes.push(j);
            }

            let mut is_valid_num: bool = false;
            for k in adjacent_coordinates(i as i32, j as i32, &bound) {
                let adj = content[k.0 as usize][k.1 as usize];
                if adj != '.' && !adj.is_numeric() {
                    is_valid_num = true;
                    break;
                }
            }
        }
    }

    let mut result: i32 = 0;
    schemas.iter().for_each(|i| result += i);
    println!("part 1: {}", result);
}

fn adjacent_coordinates(i: i32, j: i32, bound: &(i32, i32)) -> Vec<(i32, i32)> {
    let coordinates: Vec<(i32, i32)> = vec![
        (i, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i - 1, j - 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
        (i, j + 1),
    ];
    let (row_count, col_count) = bound;
    return coordinates
        .into_iter()
        .filter(|(i, j)| {
            return i >= &0 && i < row_count && j >= &0 && j < col_count;
        })
        .collect::<Vec<(i32, i32)>>();
}
