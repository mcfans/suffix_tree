fn max_heapify<T: Ord + Copy>(data: &mut [T], pos: usize, end: usize) {
    let mut dad = pos;
    let mut son = dad * 2 + 1;
    while son <= end {
        if son < end && data[son] < data[son + 1] {
            son += 1;
        }
        if data[dad] > data[son] {
            return;
        } else {
            data.swap(dad, son);
            dad = son;
            son = dad * 2 + 1;
        }
    }
}

fn heap_sort<T:Ord+Copy>(data: &mut[T]) {
    let len = data.len();
    for i in (0..=len / 2 - 1).rev() {
        max_heapify(data, i, len - 1);
    }
    for i in (1..=len - 1).rev() {
        data.swap(0, i);
        max_heapify(data, 0, i - 1);
    }
}

fn main() {
    let mut nums = vec![9, 2, 1, 7, 6, 8, 5, 3, 4];
    heap_sort(nums.as_mut_slice());

    println!("{:?}", nums);
}