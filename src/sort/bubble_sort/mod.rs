use std::cmp::Ordering;

fn bubble_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 0..list.len() {
        for j in 0..(list.len() - i - 1) { // 注意边界
            if list.get(j).unwrap() > list.get(j+1).unwrap() {
                list.swap(j, j+1);
            }
        }
    }

    list
}

fn bubble_sort_generic<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    for i in 0..list.len() {
        for j in 0..(list.len() - i - 1) { // 注意边界
            if list.get(j).unwrap().cmp(list.get(j+1).unwrap()) == Ordering::Greater {
                list.swap(j, j+1);
            }
        }
    }

    list
}


#[test]
fn test() {
    let list = vec![8, 3, 5, 4, 6];
    let result = bubble_sort(list);
    println!("{:?}", result);
    let list = vec![10, -1, 3, 9, 2, 27, 8, 5, 1, 3, 0, 26];
    let result = bubble_sort(list);
    println!("{:?}", result);
}

#[test]
fn test_generic() {
    let list = vec!["a", "tc", "9", "u", "r", "b", "n"];
    let result = bubble_sort_generic(list);
    println!("{:?}", result);
}
