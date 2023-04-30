
fn shell_sort<T: Ord>(list: &mut [T]) {
    let mut gap = list.len() / 2;
    while gap > 0 {
        for i in gap..list.len() { // 在这里排序一个子序列，i 是列的序号，也是第二个元素的位置
            for j in (i..list.len()).step_by(gap) { // 选择某个值，j 是子列表的第二个元素开始
                let mut current = j;
                let mut prev = j - gap;
                while current < list.len() {
                    if list.get(current).unwrap() < list.get(prev).unwrap() {
                        list.swap(prev, current);
                    }
                    prev += gap;
                    current += gap;
                }
            }
        }
        gap /= 2;
    }
}


#[test]
fn test() {
    let mut list = vec![8, 3, 5, 4, 6];
    shell_sort(&mut list);
    println!("{:?}", list);
    let mut list = vec![10, -1, 3, 9, 2, 27, 8, 5, 1, 3, 0, 26];
    shell_sort(&mut list);
    println!("{:?}", list);
}