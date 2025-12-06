pub trait Transpose {
    fn transpose(&self) -> Self;
}

impl Transpose for Vec<Vec<char>> {
    fn transpose(&self) -> Self {
        if self.is_empty() || self[0].is_empty() {
            return vec![];
        }

        let row_count = self.len();
        let col_count = self[0].len();
        let mut transposed = vec![vec![' '; row_count]; col_count];

        for i in 0..row_count {
            for j in 0..col_count {
                transposed[j][i] = self[i][j];
            }
        }
        transposed
    }
}

impl Transpose for String {
    fn transpose(&self) -> Self {
        let rows: Vec<&str> = self.lines().collect();
        if rows.is_empty() {
            return String::new();
        }

        let row_count = rows.len();
        let col_count = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        let mut transposed_rows: Vec<String> = vec![String::new(); col_count];

        for col_index in 0..col_count {
            for row_index in 0..row_count {
                let row = rows[row_index];
                if col_index < row.len() {
                    transposed_rows[col_index].push(row.chars().nth(col_index).unwrap());
                } else {
                    transposed_rows[col_index].push(' ');
                }
            }
        }
        transposed_rows.join("\n")
    }
}

impl Transpose for Vec<Vec<u64>> {
    fn transpose(&self) -> Self {
        if self.is_empty() || self[0].is_empty() {
            return vec![];
        }

        let row_count = self.len();
        let col_count = self[0].len();
        let mut transposed = vec![vec![0u64; row_count]; col_count];

        for i in 0..row_count {
            for j in 0..col_count {
                transposed[j][i] = self[i][j];
            }
        }
        transposed
    }
}

#[cfg(test)]
mod tests {
    use super::Transpose;

    #[test]
    fn test_transpose_square() {
        let matrix = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let transposed_matrix = matrix.transpose();
        let expected = vec![
            vec!['a', 'd', 'g'],
            vec!['b', 'e', 'h'],
            vec!['c', 'f', 'i'],
        ];
        assert_eq!(transposed_matrix, expected);
    }

    #[test]
    fn test_transpose_rectangle() {
        let matrix = vec![
            vec!['a', 'b', 'c', 'd'],
            vec!['e', 'f', 'g', 'h'],
        ];
        let transposed_matrix = matrix.transpose();
        let expected = vec![
            vec!['a', 'e'],
            vec!['b', 'f'],
            vec!['c', 'g'],
            vec!['d', 'h'],
        ];
        assert_eq!(transposed_matrix, expected);
    }

    #[test]
    fn test_transpose_string() {
        let string = String::from("abcd");
        let transposed_string = string.transpose();
        let expected = String::from("a\nb\nc\nd");
        assert_eq!(transposed_string, expected);
    }

    #[test]
    fn test_transpose_multiline_string() {
        let string = String::from("abc\ndef\nghi");
        let transposed_string = string.transpose();
        let expected = String::from("adg\nbeh\ncfi");
        assert_eq!(transposed_string, expected);
    }

    #[test]
    fn test_transpose_u64_matrix() {
        let matrix = vec![
            vec![1u64, 2u64, 3u64],
            vec![4u64, 5u64, 6u64],
        ];
        let transposed_matrix = matrix.transpose();
        let expected = vec![
            vec![1u64, 4u64],
            vec![2u64, 5u64],
            vec![3u64, 6u64],
        ];
        assert_eq!(transposed_matrix, expected);
    }
}
