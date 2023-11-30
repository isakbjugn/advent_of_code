
pub fn part_1(input: &str) -> u32 {
    let forest_width = input.lines().next().unwrap().len();
    let forest = init_forest(input);

    let mut num_visible = forest_width * 4 - 4;

    for (row, tree_row) in forest.iter().enumerate() {
        if row == 0 || row == forest_width - 1 { continue }

        for (col, tree) in tree_row.iter().enumerate() {
            if col == 0 || col == forest_width - 1 { continue }

            let mut visible: bool;

            // north
            visible = true;
            for row_tree in 0..row {
                if forest[row_tree][col] >= *tree {
                    //println!("Nordsjekk: {} skuggar for {} ({},{})", forest[row_tree][col], *tree, row, col);
                    visible = false;
                    break;
                }
            }
            if visible {
                //println!("Nordsjekk: {} er synleg ({},{})\n", *tree, row, col);
                num_visible += 1;
                continue;
            }

            // south
            visible = true;
            for row_tree in row+1..forest_width {
                if forest[row_tree][col] >= *tree {
                    //println!("Sørsjekk: {} skuggar for {} ({},{})", forest[row_tree][col], *tree, row, col);
                    visible = false;
                    break;
                }
            }
            if visible {
                //println!("Sørsjekk: {} er synleg ({},{})\n", *tree, row, col);
                num_visible += 1;
                continue;
            }

            // west
            visible = true;
            for col_tree in 0..col {
                if forest[row][col_tree] >= *tree {
                    //println!("Vestsjekk: {} skuggar for {} ({},{})", forest[row][col_tree], *tree, row, col);
                    visible = false;
                    break
                }
            }
            if visible {
                //println!("Vestsjekk: {} er synleg ({},{})\n", *tree, row, col);
                num_visible += 1;
                continue;
            }

            // east
            visible = true;
            for col_tree in col+1..forest_width {
                if forest[row][col_tree] >= *tree {
                    //println!("Austsjekk: {} skuggar for {} ({},{})", forest[row][col_tree], *tree, row, col);
                    visible = false;
                    break
                }
            }
            if visible {
                //println!("Austsjekk: {} er synleg ({},{})\n", *tree, row, col);
                num_visible += 1;
                continue;
            }

            //println!("{} er ikkje synleg ({},{})\n", *tree, row, col);
        }
    }

    num_visible as u32
}

fn init_forest(input: &str) -> Vec<Vec<u32>> {
    let forest_width = input.lines().next().unwrap().len();
    let mut forest = vec![vec![0; forest_width]; forest_width];
    for (row, line) in input.lines().enumerate() {
        for (col, tree) in line.chars().enumerate() {
            forest[row][col] = tree.to_digit(10).unwrap();
        }
    }
    forest
}

fn find_scenic_score(forest: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let forest_width = forest[0].len();
    let mut scenic_scores = vec![vec![0; forest_width]; forest_width];

    for (row, tree_row) in forest.iter().enumerate() {
        if row == 0 || row == forest_width - 1 { continue }

        for (col, tree) in tree_row.iter().enumerate() {
            if col == 0 || col == forest_width - 1 { continue }

            // north
            let mut view_north = 0;
            for row_tree in (0..row).rev() {
                view_north += 1;
                if forest[row_tree][col] >= *tree { break }
            }

            // south
            let mut view_south = 0;
            for row_tree in row+1..forest_width {
                view_south += 1;
                if forest[row_tree][col] >= *tree { break }
            }

            // west
            let mut view_west = 0;
            for col_tree in (0..col).rev() {
                view_west += 1;
                if forest[row][col_tree] >= *tree { break }
            }

            // east
            let mut view_east = 0;
            for col_tree in col+1..forest_width {
                view_east += 1;
                if forest[row][col_tree] >= *tree { break }
            }

            scenic_scores[row][col] = view_north * view_south * view_west * view_east;
        }
    }
    scenic_scores
}

pub fn part_2(input: &str) -> u32 {
    let forest = init_forest(input);
    let scenic_scores = find_scenic_score(forest);
    let mut max_scenic_score = 0;

    for row in scenic_scores {
        for score in row {
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }
    max_scenic_score
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_8.txt");
    assert_eq!(part_1(input), 21)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_8.txt");
    assert_eq!(part_2(input), 8)
}
