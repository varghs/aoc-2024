use std::fmt::Display;
use regex::Regex;

pub fn part1(s: &str) -> impl Display {
    let re_capture = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    s.lines()
    .map(|line| re_capture
        .captures_iter(line)
        .map(|c: regex::Captures<'_>| (c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap())))
    .flatten()
    .map(|(x, y)| x * y).sum::<u32>()
}

pub fn part2(s: &str) -> impl Display {
    ""
}

// python code for day 4
// import re

// def num_matches(search_str, text):
//     return len(re.findall(search_str, text))

// def check_xmas_matrix(matrix) -> bool:
//     first_diagonal = matrix[0][0] + matrix[1][1] + matrix[2][2]
//     second_diagonal = matrix[2][0] + matrix[1][1] + matrix[0][2]

//     return (first_diagonal == "MAS" or first_diagonal[::-1] == "MAS") and (second_diagonal == "MAS" or second_diagonal[::-1] == "MAS")

// with open("input.txt", "r") as file:
//     text = file.read()
//     horizontal = text.split("\n")
//     vertical = [[row[i] for row in horizontal] for i in range(len(horizontal[0]))]
//     left_to_right_diagonal = [[] for i in range(len(horizontal) + len(horizontal[0]) - 1)]
//     right_to_left_diagonal = [[] for i in range(len(horizontal) + len(horizontal[0]) - 1)]

//     for i in range(len(horizontal)):
//         for j in range(i, len(horizontal[0])):
//             left_to_right_diagonal[j - i].append(horizontal[i][j])
    
//     for i in range(1, len(horizontal)):
//         for j in range(i):
//             left_to_right_diagonal[len(horizontal[0]) + i - j - 1].append(horizontal[i][j])
    
//     horizontal_mirrored = [s[::-1] for s in horizontal]

//     for i in range(len(horizontal_mirrored)):
//         for j in range(i, len(horizontal_mirrored[0])):
//             right_to_left_diagonal[j - i].append(horizontal_mirrored[i][j])
    
//     for i in range(1, len(horizontal_mirrored)):
//         for j in range(i):
//             right_to_left_diagonal[len(horizontal_mirrored[0]) + i - j - 1].append(horizontal_mirrored[i][j])
    
//     vertical = [''.join(char_list) for char_list in vertical]
//     vertical_mirrored = [s[::-1] for s in vertical]
//     left_to_right_diagonal = [''.join(char_list) for char_list in left_to_right_diagonal]
//     left_to_right_diagonal_mirrored = [s[::-1] for s in left_to_right_diagonal]
//     right_to_left_diagonal = [''.join(char_list) for char_list in right_to_left_diagonal]
//     right_to_left_diagonal_mirrored = [s[::-1] for s in right_to_left_diagonal]

//     total = 0
//     total += sum(num_matches("XMAS", text) for text in horizontal)
//     total += sum(num_matches("XMAS", text) for text in vertical)
//     total += sum(num_matches("XMAS", text) for text in left_to_right_diagonal)
//     total += sum(num_matches("XMAS", text) for text in right_to_left_diagonal)
//     total += sum(num_matches("XMAS", text) for text in horizontal_mirrored)
//     total += sum(num_matches("XMAS", text) for text in vertical_mirrored)
//     total += sum(num_matches("XMAS", text) for text in left_to_right_diagonal_mirrored)
//     total += sum(num_matches("XMAS", text) for text in right_to_left_diagonal_mirrored)
//     print(total)

//     new_total = 0
//     for i in range(len(horizontal)-2):
//         for j in range(len(horizontal[0]) - 2):
//                 matrix = [horizontal[i][j:j+3], horizontal[i+1][j:j+3], horizontal[i+2][j:j+3]]
//                 if check_xmas_matrix(matrix):
//                     new_total += 1
//     print(new_total)

// python code for day 3 of last year
// import re

// class Coordinate:
//     def __init__(self, row, x_start, x_end, total_rows, total_cols):
//         self.row = row
//         self.x_start = x_start
//         self.x_end = x_end
//         self.total_rows = total_rows
//         self.total_cols = total_cols
    
//     def check_boundary(self, lines):
//         coords = []
//         if self.row > 0:
//             coords += [(self.row-1, i) for i in range(max(0, self.x_start-1), min(total_cols, self.x_end+2))]
//         if self.x_start > 0:
//             coords += [(self.row, self.x_start-1)]
//         if self.x_end < self.total_cols - 1:
//             coords += [(self.row, self.x_end+1)]
//         if self.row < self.total_rows-1:
//             coords += [(self.row+1, i) for i in range(max(0, self.x_start-1), min(total_cols, self.x_end+2))]

//         for x, y in coords:
//             if lines[x][y] != ".":
//                 return True
            
//         return False

// class ActualCoordinate:
//     def __init__(self, x, y):
//         self.x = x
//         self.y = y
    
//     def in_range(self, other_x, other_y_start, other_y_end):
//         return self.x == other_x and self.y >= other_y_start and self.y <= other_y_end

//     def boundary_coords(self, total_rows, total_cols):
//         coords = []
//         for i in range(max(0, self.x-1), min(self.x+2, to))

// with open("input.txt", "r") as file:
//     text = file.read()
//     lines = text.split("\n")
//     matches = []
//     pattern = r"\d+"
//     total_rows = len(lines)
//     total_cols = len(lines[0])
//     for row_num, line in enumerate(lines):
//         row_matches = re.finditer(pattern, line)
//         for match in row_matches:
//             start, end = match.start(), match.end() - 1
//             num = int(match.group())
//             matches.append((Coordinate(row_num, start, end, total_rows, total_cols), num))
    
//     sum = 0
//     for match in matches:
//         if match[0].check_boundary(lines):
//             sum += match[1]
    
//     print(sum)

//     gears = []
//     for i in range(len(lines)):
//         for j in range(len(lines[0])):
//             if lines[i][j] == "*":
//                 gears.append(ActualCoordinate(j, i))
    