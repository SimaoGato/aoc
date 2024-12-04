#[test]
fn test() {
    solve(String::from(
        "MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX",
    ));
}

pub fn solve(data: String) {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.trim().chars() {
            row.push(c);
        }
        matrix.push(row);
    }

    let mut count = 0;

    // Number of rows (vertical size)
    let rows = matrix.len();

    // Number of columns (horizontal size)
    // Assuming the matrix is well-formed (all rows have the same length)
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 'X' {
                // check right
                if j + 3 <= cols - 1 {
                    if matrix[i][j + 1] == 'M' && matrix[i][j + 2] == 'A' && matrix[i][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // check left
                if j >= 3 {
                    if matrix[i][j - 1] == 'M' && matrix[i][j - 2] == 'A' && matrix[i][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // check up
                if i >= 3 {
                    if matrix[i - 1][j] == 'M' && matrix[i - 2][j] == 'A' && matrix[i - 3][j] == 'S'
                    {
                        count += 1;
                    }
                }

                // check down
                if i + 3 <= rows - 1 {
                    if matrix[i + 1][j] == 'M' && matrix[i + 2][j] == 'A' && matrix[i + 3][j] == 'S'
                    {
                        count += 1;
                    }
                }

                // check up-right
                if i >= 3 && j + 3 <= cols - 1 {
                    if matrix[i - 1][j + 1] == 'M'
                        && matrix[i - 2][j + 2] == 'A'
                        && matrix[i - 3][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // check up-left
                if i >= 3 && j >= 3 {
                    if matrix[i - 1][j - 1] == 'M'
                        && matrix[i - 2][j - 2] == 'A'
                        && matrix[i - 3][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // check down-right
                if i + 3 <= rows - 1 && j + 3 <= cols - 1 {
                    if matrix[i + 1][j + 1] == 'M'
                        && matrix[i + 2][j + 2] == 'A'
                        && matrix[i + 3][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // check down-left
                if i + 3 <= rows - 1 && j >= 3 {
                    if matrix[i + 1][j - 1] == 'M'
                        && matrix[i + 2][j - 2] == 'A'
                        && matrix[i + 3][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", count);

    count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 'M' {
                // Check Up-Rigth
                if i >= 2 && j + 2 <= cols - 1 {
                    if matrix[i - 1][j + 1] == 'A' && matrix[i - 2][j + 2] == 'S' {
                        if matrix[i - 2][j] == 'M' && matrix[i][j + 2] == 'S' {
                            count += 1;
                        }
                        if matrix[i - 2][j] == 'S' && matrix[i][j + 2] == 'M' {
                            count += 1;
                        }
                    }
                }

                // Check Up-Left
                if i >= 2 && j >= 2 {
                    if matrix[i - 1][j - 1] == 'A' && matrix[i - 2][j - 2] == 'S' {
                        if matrix[i - 2][j] == 'M' && matrix[i][j - 2] == 'S' {
                            count += 1;
                        }
                        if matrix[i - 2][j] == 'S' && matrix[i][j - 2] == 'M' {
                            count += 1;
                        }
                    }
                }

                // Check Down-Right
                if i + 2 <= rows - 1 && j + 2 <= cols - 1 {
                    if matrix[i + 1][j + 1] == 'A' && matrix[i + 2][j + 2] == 'S' {
                        if matrix[i + 2][j] == 'M' && matrix[i][j + 2] == 'S' {
                            count += 1;
                        }
                        if matrix[i + 2][j] == 'S' && matrix[i][j + 2] == 'M' {
                            count += 1;
                        }
                    }
                }

                // Check Down-Left
                if i + 2 <= rows - 1 && j >= 2 {
                    if matrix[i + 1][j - 1] == 'A' && matrix[i + 2][j - 2] == 'S' {
                        if matrix[i + 2][j] == 'M' && matrix[i][j - 2] == 'S' {
                            count += 1;
                        }
                        if matrix[i + 2][j] == 'S' && matrix[i][j - 2] == 'M' {
                            count += 1;
                        }
                    }
                }

                matrix[i][j] = '.';
            }
        }
    }

    println!("Part 2: {}", count);
}
