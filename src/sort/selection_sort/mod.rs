use std::cmp::Ordering;

fn selection_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 0..list.len() {
        let mut lowest = i;
        for j in (i+1)..list.len() {
            if list.get(j).unwrap() < list.get(lowest).unwrap() {
                lowest = j;
            }
        }
        let v = list.swap_remove(lowest);
        list.insert(i, v);
    }
    list
}

fn selection_sort_generic<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    for i in 0..list.len() {
        let mut lowest = i;
        for j in (i+1)..list.len() {
            if list.get(j).unwrap().cmp(list.get(lowest).unwrap()) == Ordering::Less {
                lowest = j;
            }
        }
        let v = list.swap_remove(lowest);
        list.insert(i, v);
    }
    list
}


#[test]
fn test() {
    let list = vec![8, 3, 5, 4, 6];
    let result = selection_sort(list);
    println!("{:?}", result);
    let list = vec![10, -1, 3, 9, 2, 27, 8, 5, 1, 3, 0, 26];
    let result = selection_sort(list);
    println!("{:?}", result);
}

#[test]
fn test_generic() {
    let list = vec!["a", "tc", "9", "u", "r", "b", "n"];
    let result = selection_sort_generic(list);
    println!("{:?}", result);
}