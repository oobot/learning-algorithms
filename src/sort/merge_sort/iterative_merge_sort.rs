use std::cmp::min;

fn merge_sort<T: Ord + Copy>(array: &mut [T]) {
    let mut step = 1;
    while step < array.len() {
        let mut vec = Vec::with_capacity(array.len());
        let (mut start, mut mid, mut end) = (0, step, min(step + step, array.len()));
        while start < array.len() {
            vec.extend(merge(&array[start..mid], &array[mid..end]));

            start = end;
            mid = min(start + step, array.len() - 1);
            end = min(mid + step, array.len());
        }

        for i in 0..array.len() {
            array[i] = vec[i];
        }

        step += step;
    }
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let mut vec = Vec::with_capacity(left.len() + right.len());
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() || right_index < right.len() {
        if left_index < left.len() && right_index < right.len() {
            if left.get(left_index) < right.get(right_index) {
                vec.push(left[left_index]);
                left_index += 1;
            } else {
                vec.push(right[right_index]);
                right_index += 1;
            }
        } else if left_index < left.len() && right_index >= right.len() {
            vec.push(left[left_index]);
            left_index += 1;
        } else if left_index >= left.len() && right_index < right.len() {
            vec.push(right[right_index]);
            right_index += 1;
        }
    }

    vec
}

#[test]
fn test() {
    let mut list = vec![6, 5, 3, 1, 8, 2, 7, 9, 4];
    merge_sort(&mut list);
    println!("{:?}", list);
}
