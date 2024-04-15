use std::{
    collections::HashMap,
    fmt::{Display, Write},
    ops::Range,
    vec,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharData {
    None,
    End,
    Char(u8),
}

impl Ord for CharData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (CharData::None, CharData::None) => std::cmp::Ordering::Equal,
            (CharData::None, CharData::End) => std::cmp::Ordering::Less,
            (CharData::End, CharData::None) => std::cmp::Ordering::Greater,
            (CharData::End, CharData::Char(_)) => std::cmp::Ordering::Less,
            (CharData::Char(_), CharData::End) => std::cmp::Ordering::Greater,
            (CharData::End, CharData::End) => std::cmp::Ordering::Equal,
            (CharData::None, CharData::Char(_)) => std::cmp::Ordering::Less,
            (CharData::Char(_), CharData::None) => std::cmp::Ordering::Greater,
            (CharData::Char(l), CharData::Char(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for CharData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for CharData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharData::None => f.write_str("None"),
            CharData::End => f.write_str("End "),
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
            if content.len() + 1 > lines.len() {
                lines.resize_with(content.len() + 1, || vec![CharData::None; len]);
            }

            for (i, c) in content.bytes().rev().enumerate() {
                lines[i][idx] = CharData::Char(c);
            }
            lines[content.len()][idx] = CharData::End;
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
        self.sort_first_line();

        let mut ranges = self.calculate_same_group_range(0, None);

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
                            (CharData::None, CharData::End) => {
                                result_for_this_range.push(start..i);
                                start = i;
                                current_char = self.lines[line][i];
                            }
                            (CharData::End, CharData::None) => {
                                result_for_this_range.push(start..i);
                                start = i;
                                current_char = self.lines[line][i];
                            }
                            (CharData::End, CharData::End) => {
                                result_for_this_range.push(start..i);
                                start = i;
                                current_char = self.lines[line][i];
                            }
                            (CharData::End, CharData::Char(_)) => {
                                result_for_this_range.push(start..i);
                                start = i;
                                current_char = self.lines[line][i];
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
                            (CharData::Char(_), CharData::End) => {
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
                    _ => {
                        unreachable!("After none should not call")
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
        self.sort_line(0, 0..self.lines[0].len())
    }

    fn sort_line(&self, line: usize, range: Range<usize>) {
        let start = range.start;

        let len = range.len();
        if range.len() < 2 {
            return;
        }
        for i in (0..=len / 2 - 1).rev() {
            self.max_heapify(line, start, i, len - 1);
        }
        for i in (1..=len - 1).rev() {
            self.swap_column(start, i + start);
            self.max_heapify(line, start, 0, i - 1);
        }
    }

    fn max_heapify(&self, line: usize, offset: usize, pos: usize, end: usize) {
        let mut dad = pos;
        let mut son = dad * 2 + 1;
        while son <= end {
            if son < end && self.lines[line][offset + son] < self.lines[line][offset + son + 1] {
                son += 1;
            }
            if self.lines[line][offset + dad] > self.lines[line][offset + son] {
                return;
            } else {
                self.swap_column(dad + offset, son + offset);
                dad = son;
                son = dad * 2 + 1;
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
                        } else if let CharData::End = char {
                            // This is a node where itself is a end of a string but it also the parent of other nodes
                            // ab.apple.com b.apple.com
                            // b
                            let payload = self.id_line[child.start];
                            result[need_update_range.location].payload = Some(payload);
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

#[derive(Debug)]
pub struct SuffixMatcher {
    nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct ExactMatcher {
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
        let mut current_matched: Option<i64> = None;

        for c in bytes_iter {
            let table = current_node.table;
            let moved = table << (127 - c);
            let first_bit = moved & MASK;

            if first_bit == MASK {
                let pos = moved.count_ones() - 1;
                let current_range_start = current_node.range_start;
                // println!("Finding char {} at relative position {} absolute position {}", c as char, pos, current_range_start + pos as usize);
                current_node = &self.nodes[current_range_start + pos as usize];
                if current_node.payload.is_some() {
                    current_matched = current_node.payload;
                }
            } else {
                break;
            }
        }

        current_matched
    }
}

impl ExactMatcher {
    pub fn new(nodes: Vec<Node>) -> ExactMatcher {
        ExactMatcher { nodes }
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
                current_node = &self.nodes[current_range_start + pos as usize];
            } else {
                return None;
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
        assert_eq!(Some(1), matcher.find("weather.apple.com"));
    }
}
