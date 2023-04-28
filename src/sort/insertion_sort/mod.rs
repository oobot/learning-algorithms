use std::cmp::Ordering;

fn insertion_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 1..list.len() {
        for j in 0..i { // 已排好序的切片部分
            if list.get(i).unwrap() < list.get(j).unwrap() {
                let e = list.swap_remove(i);
                list.insert(j, e);
                break;
            }
        }
    }
    list
}

fn insertion_sort_generic<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    for i in 1..list.len() {
        for j in 0..i { // 已排好序的切片部分
            if list.get(i).unwrap().cmp(list.get(j).unwrap()) == Ordering::Less {
                let e = list.swap_remove(i);
                list.insert(j, e);
                break;
            }
        }
    }
    list
}


#[test]
fn test() {
    let list = vec![8, 3, 5, 4, 6];
    let result = insertion_sort(list);
    println!("{:?}", result);
    let list = vec![10, -1, 3, 9, 2, 27, 8, 5, 1, 3, 0, 26];
    let result = insertion_sort(list);
    println!("{:?}", result);
}

#[test]
fn test_generic() {
    let list = vec!["a", "tc", "9", "u", "r", "b", "n"];
    let result = insertion_sort_generic(list);
    println!("{:?}", result);
}



















