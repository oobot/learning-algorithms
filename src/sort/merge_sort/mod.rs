mod iterative_merge_sort;

fn merge_sort<T: Ord + Copy>(array: & mut [T]) {
    let mid = array.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut array[..mid]);
    merge_sort(&mut array[mid..]);
    merge(array, mid);
}

fn merge<T: Ord + Copy>(array: &mut [T], mid: usize) {
    let mut new_array = vec![];

    let mut left_index = 0;
    let mut right_index = mid;

    while left_index < mid || right_index < array.len() {
        if left_index < mid && right_index < array.len() {
            if array.get(left_index) < array.get(right_index) {
                new_array.push(array[left_index]);
                left_index += 1;
            } else {
                new_array.push(array[right_index]);
                right_index += 1;
            }
        } else if left_index < mid && right_index >= array.len() {
            new_array.push(array[left_index]);
            left_index += 1;
        } else if left_index >= mid && right_index < array.len() {
            new_array.push(array[right_index]);
            right_index += 1;
        }

    }
    for i in 0..new_array.len() {
        array[i] = new_array[i];
    }
}

#[test]
fn test() {
    let mut list = vec![6, 5, 3, 1, 8, 2, 7, 9, 4];
    merge_sort(&mut list);
    println!("{:?}", list);
}