use std::{
    collections::HashMap, fmt::{Display, Write}, ops::Range, vec
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharData {
    None,
    Char(u8),
}

impl Display for CharData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharData::None => f.write_str("None"),
            CharData::Char(c) => f.write_fmt(format_args!("{}   ", *c as char)),
        }
    }
}

#[derive(Debug)]
pub struct Matrix {
    id_line: Vec<i64>,

    lines: Vec<Vec<CharData>>,

    sorted_ranges: Vec<HashMap<Range<usize>, Vec<Range<usize>>>>,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Matrix\n")?;
        for column in 0..self.id_line.len() {
            f.write_fmt(format_args!("{}    ", column))?;
        }
        f.write_char('\n')?;
        for (_, line) in self.lines.iter().enumerate() {
            for char in line {
                f.write_fmt(format_args!("{} ", char))?;
            }
            f.write_char('\n')?;
        }

        for line in &self.id_line {
            f.write_fmt(format_args!("{} ", line))?;
        }
        f.write_char('\n')?;

        for (idx, map) in self.sorted_ranges.iter().enumerate() {
            f.write_fmt(format_args!("Line {}:\n", idx))?;
            for (key, value) in map {
                f.write_fmt(format_args!("{:?} -> {:?}\n", key, value))?;
            }
        }

        Ok(())
    }
}

impl Matrix {
    pub fn new(data: &[(i64, String)]) -> Matrix {
        let len = data.len();

        let mut id_line = Vec::with_capacity(len);

        let mut lines: Vec<Vec<CharData>> = vec![];

        for (idx, pair) in data.iter().enumerate() {
            id_line.push(pair.0);

            let content = &pair.1;
            if content.len() > lines.len() {
                lines.resize_with(content.len(), || vec![CharData::None; len]);
            }

            for (i, c) in content.bytes().rev().enumerate() {
                lines[i][idx] = CharData::Char(c);
            }
        }

        let line_count = lines.len();

        let mut sorted_ranges = vec![];

        for _ in 0..line_count {
            sorted_ranges.push(HashMap::new());
        }
        sorted_ranges.push(HashMap::new());

        Matrix {
            id_line,
            lines,
            sorted_ranges,
        }
    }

    fn swap_column(&self, a: usize, b: usize) {
        unsafe {
            let len = self.id_line.len();
            let id_line_ptr = self.id_line.as_ptr().cast_mut();
            let id_line_mut = std::slice::from_raw_parts_mut(id_line_ptr, len);
            id_line_mut.swap(a, b);

            let max_len = self.lines.len();
            for i in 0..max_len {
                let len = self.lines[i].len();
                let line_ptr = self.lines[i].as_ptr().cast_mut();
                let line_mut = std::slice::from_raw_parts_mut(line_ptr, len);
                line_mut.swap(a, b);
            }
        }
        // self.id_line.swap(a, b);

        // let max_len = self.lines.len();
        // for i in 0..max_len {
        //     self.lines[i].swap(a, b);
        // }
    }

    pub fn sort_in_place(&mut self) {
        println!("First line begin");
        self.sort_first_line();
        println!("First line done");

        let mut ranges = self.calculate_same_group_range(0, None);
        
        println!("First line range calculate done");
        for i in 1..self.lines.len() {
            ranges.par_iter().for_each(|range| {
                self.sort_line(i, range.clone());
            });
            ranges = self.calculate_same_group_range(i, Some(ranges));
        }
    }

    fn calculate_same_group_range(
        &mut self,
        line: usize,
        previous_range: Option<Vec<Range<usize>>>,
    ) -> Vec<Range<usize>> {
        let mut result = vec![];

        if let Some(previous_range) = previous_range {
            for range in previous_range {
                let mut start = range.start;
                let end = range.end;
                let mut result_for_this_range = vec![];
                let mut current_char = self.lines[line][range.start];
                let cloned_range = range.clone();
                if range.len() == 1 {
                    if let CharData::Char(_) = self.lines[line][range.start] {
                        result_for_this_range.push(range);
                    }
                } else {
                    for i in range {
                        let iter_char = self.lines[line][i];
                        match (current_char, iter_char) {
                            (CharData::None, CharData::None) => {
                                start = i;
                            }
                            (CharData::None, CharData::Char(_)) => {
                                start = i;
                                current_char = self.lines[line][i];
                            }
                            (CharData::Char(_), CharData::None) => {
                                result_for_this_range.push(start..i);
                                start = i;
                                current_char = self.lines[line][i];
                            }
                            (CharData::Char(current), CharData::Char(iter_char)) => {
                                if iter_char == current {
                                    continue;
                                } else {
                                    result_for_this_range.push(start..i);
                                    start = i;
                                    current_char = self.lines[line][i];
                                }
                            }
                        }
                    }
                    result_for_this_range.push(start..end);
                }

                if !result_for_this_range.is_empty() {
                    self.sorted_ranges[line].insert(cloned_range, result_for_this_range.clone());

                    result.extend(result_for_this_range);
                }
            }
        } else {
            let mut start = 0;
            let mut current_char = self.lines[line][0];
            for i in 0..self.lines[line].len() {
                let iter_char = self.lines[line][i];

                match (current_char, iter_char) {
                    (CharData::None, CharData::None) => unreachable!("After none should not call"),
                    (CharData::None, CharData::Char(_)) => {
                        unreachable!("After none should not call")
                    }
                    (CharData::Char(_), CharData::None) => {
                        result.push(start..i);
                        start = i;
                        current_char = self.lines[line][i];
                    }
                    (CharData::Char(current), CharData::Char(iter_char)) => {
                        if iter_char == current {
                            continue;
                        } else {
                            result.push(start..i);
                            start = i;
                            current_char = self.lines[line][i];
                        }
                    }
                }
            }
            result.push(start..self.lines[line].len());
            let special_key = 0..self.lines[line].len();
            self.sorted_ranges[0].insert(special_key, result.clone());
        }
        result
    }

    fn sort_first_line(&mut self) {
        for i in 0..self.lines[0].len() {
            for j in 0..self.lines[0].len() - 1 - i {
                let left = self.lines[0][j];
                let right = self.lines[0][j + 1];

                match (left, right) {
                    (CharData::Char(l), CharData::Char(r)) => {
                        if l > r {
                            self.swap_column(j, j + 1);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn sort_line(&self, line: usize, range: Range<usize>) {
        let end = range.end;
        let start = range.start;
        for i in 0..range.count() {
            for j in start..end - i - 1 {
                let left = self.lines[line][j];
                let right = self.lines[line][j + 1];

                match (left, right) {
                    (CharData::Char(l), CharData::Char(r)) => {
                        if l > r {
                            self.swap_column(j, j + 1);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn build_tree(&self) -> Vec<Node> {
        let mut result = vec![];

        let iter = self.sorted_ranges.iter().enumerate();

        result.push(Node {
            table: 0,
            range_start: 0,
            range_end: 0,
            payload: None,
        });

        let len = self.id_line.len();

        let mut need_update_nodes_ranges = vec![BuildingProcessPair {
            represent: 0..len,
            location: 0,
        }];

        for (idx, map) in iter {
            let mut next_need_update_nodes_ranges = vec![];
            for need_update_range in &need_update_nodes_ranges {
                let children = map.get(&need_update_range.represent);
                if let Some(children) = children {
                    let mut table = 0u128;
                    let start = result.len();
                    for child in children {
                        let char = self.lines[idx][child.start];
                        if let CharData::Char(char) = char {
                            table |= 1 << char;

                            // Add new node
                            result.push(Node {
                                table: 0,
                                range_start: 0,
                                range_end: 0,
                                payload: None,
                            });

                            next_need_update_nodes_ranges.push(BuildingProcessPair {
                                represent: child.clone(),
                                location: result.len() - 1,
                            });
                        } else {
                            // This is a node where itself is a end of a string but it also the parent of other nodes
                            // ab.apple.com b.apple.com  
                            // b
                            let payload = self.id_line[child.start];
                            result[need_update_range.location].payload = Some(payload);
                            // println!("Stored payload {} at {}", payload, need_update_range.location);
                        }
                    }

                    let added_node_for_update_parent = result.len() - start;

                    result[need_update_range.location].table = table;
                    result[need_update_range.location].range_start = start;
                    result[need_update_range.location].range_end =
                        start + added_node_for_update_parent;
                } else {
                    // String ends at this point
                    let payload = self.id_line[need_update_range.represent.start];
                    // println!("Storing payload {} at {}", payload, need_update_range.location);
                    result[need_update_range.location].payload = Some(payload);
                }
            }

            need_update_nodes_ranges = next_need_update_nodes_ranges;
        }
        result
    }
}

pub struct SuffixMatcher {
    nodes: Vec<Node>,
}

const MASK: u128 = 0x1 << 127;

impl SuffixMatcher {
    pub fn new(nodes: Vec<Node>) -> SuffixMatcher {
        SuffixMatcher { nodes }
    }

    pub fn find(&self, query: &str) -> Option<i64> {
        let bytes_iter = query.bytes().rev();

        let mut current_node = self.nodes.first().unwrap();

        for c in bytes_iter {
            let table = current_node.table;
            let moved = table << (127 - c);
            let first_bit = moved & MASK;

            if first_bit == MASK {
                let pos = moved.count_ones() - 1;
                let current_range_start = current_node.range_start;
                // println!("Finding char {} at relative position {} absolute position {}", c as char, pos, current_range_start + pos as usize);
                current_node =  &self.nodes[current_range_start + pos as usize];
            } else {
                break;
            }
        }

        current_node.payload
    }
}

#[derive(Debug)]
pub struct Node {
    table: u128,
    range_start: usize,
    range_end: usize,
    payload: Option<i64>,
}

struct BuildingProcessPair {
    represent: Range<usize>,
    location: usize,
}

#[cfg(test)]
mod test {

    #[test]
    fn test_matrix_sorted() {
        let data = vec![
            (1, "apple.com"),
            (2, "apple.cob"),
            (3, "apple.cbm"),
            (4, "apple.cam"),
            (5, "apple.czm"),
            (6, "appld.com"),
        ];

        let mapped: Vec<(i64, String)> = data.iter().map(|i| (i.0, i.1.to_string())).collect();
        let mut matrix = super::Matrix::new(&mapped);
        matrix.sort_in_place();

        println!("{}", matrix);

        assert_eq!(matrix.id_line.first(), Some(&2i64));
        assert_eq!(matrix.id_line, vec![2, 4, 3, 6, 1, 5]);
        assert_eq!(matrix.sorted_ranges[0].len(), 1);
        assert_eq!(
            matrix.sorted_ranges[0].get(&(0..6)),
            Some(&vec![0..1, 1..6])
        );
        assert_eq!(matrix.sorted_ranges[1].get(&(0..1)), Some(&vec![0..1]));
        assert_eq!(
            matrix.sorted_ranges[1].get(&(1..6)),
            Some(&vec![1..2, 2..3, 3..5, 5..6])
        );
    }

    #[test]
    fn test_matrix_build_tree() {
        let data = vec![
            (1, "apple.com"),
            (2, "apple.cob"),
            (3, "apple.cbm"),
            (4, "apple.cam"),
            (5, "apple.czm"),
            (6, "appld.com"),
            (7, "dauppld.com"),
            (8, "cn.apple.com"),
        ];
        let mapped: Vec<(i64, String)> = data.iter().map(|i| (i.0, i.1.to_string())).collect();

        let mut matrix = super::Matrix::new(&mapped);
        matrix.sort_in_place();
        println!("{}", matrix);

        let tree = matrix.build_tree();
        let matcher = super::SuffixMatcher { nodes: tree };

        assert_eq!(Some(8), matcher.find("cn.apple.com"));
    }
}
