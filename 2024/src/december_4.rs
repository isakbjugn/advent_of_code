use regex::Regex;

pub fn part_1(input: &str) -> usize {
    let re = Regex::new(r"XMAS").unwrap();
    let re_reverse = Regex::new(r"SAMX").unwrap();

    let horisontal = re
        .captures_iter(input)
        .count();
    let horisontal_reverse = re_reverse
        .captures_iter(input)
        .count();

    let rotated_input = rotate_string(input);
    let vertical_reverse = re
        .captures_iter(&rotated_input)
        .count();
    let vertical = re_reverse
        .captures_iter(&rotated_input)
        .count();

    let north_east: usize = extract_northeast_diagonals(input)
        .into_iter()
        .map(|line| re.captures_iter(&line).count())
        .sum();
    let north_east_reverse: usize = extract_northeast_diagonals(input)
        .into_iter()
        .map(|line| re_reverse.captures_iter(&line).count())
        .sum();

    let north_west: usize = extract_northwest_diagonals(input)
        .into_iter()
        .map(|line| re.captures_iter(&line).count())
        .sum();
    let north_west_reverse: usize = extract_northwest_diagonals(input)
        .into_iter()
        .map(|line| re_reverse.captures_iter(&line).count())
        .sum();

    horisontal + horisontal_reverse + vertical + vertical_reverse + north_east + north_east_reverse + north_west + north_west_reverse
}

fn rotate_string(input: &str) -> String {
    // Collect rows into a vector of strings
    let rows: Vec<&str> = input.lines().collect();

    // Ensure we are working with a square grid
    let size = rows.len();
    if rows.iter().any(|row| row.len() != size) {
        panic!("Input string must represent a square grid!");
    }

    // Create the rotated string by collecting columns as rows
    let mut rotated = String::new();
    for col in 0..size {
        for row in (0..size).rev() {
            rotated.push(rows[row].chars().nth(col).unwrap());
        }
        if col < size - 1 {
            rotated.push('\n');
        }
    }

    rotated
}

fn extract_northeast_diagonals(input: &str) -> Vec<String> {
    let rows: Vec<&str> = input.lines().collect();
    let size = rows.len();

    let mut diagonals = Vec::new();

    // Start from each position in the first row (top boundary)
    for col_start in 0..size {
        let mut diagonal = String::new();
        let mut col = col_start;
        for row in 0..size {
            if col >= size {
                break;
            }
            diagonal.push(rows[row].chars().nth(col).unwrap());
            col += 1;
        }
        diagonals.push(diagonal);
    }

    // Start from each position in the last column (right boundary)
    for row_start in 1..size {
        let mut diagonal = String::new();
        let mut row = row_start;
        for col in 0..size {
            if row >= size {
                break;
            }
            diagonal.push(rows[row].chars().nth(col).unwrap());
            row += 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn extract_northwest_diagonals(input: &str) -> Vec<String> {
    let rows: Vec<&str> = input.lines().collect();
    let size = rows.len();

    let mut diagonals = Vec::new();

    // Start from each position in the first row (top boundary)
    for col_start in (0..size).rev() {
        let mut diagonal = String::new();
        let mut col = col_start;
        for row in 0..size {
            if col >= size {
                break;
            }
            diagonal.push(rows[row].chars().nth(col).unwrap());
            if col == 0 {
                break;
            }
            col -= 1;
        }
        diagonals.push(diagonal);
    }

    // Start from each position in the first column (left boundary)
    for row_start in 1..size {
        let mut diagonal = String::new();
        let mut row = row_start;
        let mut col = size - 1;
        for _ in 0..size {
            if row >= size || col >= size {
                break;
            }
            diagonal.push(rows[row].chars().nth(col).unwrap());
            row += 1;
            if col == 0 {
                break;
            }
            col -= 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}

pub fn part_2(input: &str) -> usize {
    let stencil = "M*S\n*A*\nM*S";
    let stencil_90 = rotate_string(stencil);
    let stencil_180 = rotate_string(&stencil_90);
    let stencil_270 = rotate_string(&stencil_180);
    
    count_stencil_matches(input, stencil)
        + count_stencil_matches(input, &stencil_90)
        + count_stencil_matches(input, &stencil_180)
        + count_stencil_matches(input, &stencil_270)
}

fn count_stencil_matches(input: &str, stencil: &str) -> usize {
    let input_grid = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let stencil_grid = stencil.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut count = 0;

    let input_rows = input_grid.len();
    let input_cols = input_grid[0].len();
    let stencil_rows = stencil_grid.len();
    let stencil_cols = stencil_grid[0].len();

    // Iterate over all possible top-left positions for the stencil in the input
    for i in 0..=input_rows - stencil_rows {
        for j in 0..=input_cols - stencil_cols {
            let mut is_match = true;

            // Check if the stencil matches at this position
            for si in 0..stencil_rows {
                for sj in 0..stencil_cols {
                    // Asterisk matches any character, otherwise compare directly
                    if stencil_grid[si][sj] != '*' && input_grid[i + si][j + sj] != stencil_grid[si][sj] {
                        is_match = false;
                        break;
                    }
                }
                if !is_match {
                    break;
                }
            }

            if is_match {
                count += 1; // Increment the count for a successful match
            }
        }
    }

    count
}


#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_1(input), 18)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_2(input), 9)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_4.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_4.txt");
    assert_eq!(part_2(input), 0)
}
