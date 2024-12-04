pub fn part_1(input: &str) -> usize {
    [
        "XMAS",
        "SAMX",
        "X\nM\nA\nS",
        "S\nA\nM\nX",
        "X***\n*M**\n**A*\n***S",
        "***X\n**M*\n*A**\nS***",
        "S***\n*A**\n**M*\n***X",
        "***S\n**A*\n*M**\nX***",
    ]
    .into_iter()
    .map(|stencil| count_stencil_matches(input, stencil))
    .sum()
}

pub fn part_2(input: &str) -> usize {
    [
        "M*S\n*A*\nM*S",
        "M*M\n*A*\nS*S",
        "S*M\n*A*\nS*M",
        "S*S\n*A*\nM*M",
    ]
    .into_iter()
    .map(|stencil| count_stencil_matches(input, stencil))
    .sum()
}

fn count_stencil_matches(input: &str, stencil: &str) -> usize {
    let input_grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let stencil_grid = stencil
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
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
                    if stencil_grid[si][sj] != '*'
                        && input_grid[i + si][j + sj] != stencil_grid[si][sj]
                    {
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
    assert_eq!(part_1(input), 2618)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_4.txt");
    assert_eq!(part_2(input), 2011)
}
