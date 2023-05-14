use std::cmp::max;
use std::collections::LinkedList;

fn radix_sort(array: &mut [u32]) {
    let mut buckets = vec![LinkedList::new(); 10];
    for digit in 0..max_digit(array) {
        let mo = 10_u32.pow(digit + 1);
        let dev = 10_u32.pow(digit);

        // 放入桶中
        for v in &mut *array {
            let pos = (*v % mo / dev) as usize;
            buckets[pos].push_back(*v);
        }

        // 从列表前端开始取出元素
        let mut i = 0;
        for bucket in buckets.iter_mut() {
            while let Some(v) = bucket.pop_front() {
                array[i] = v;
                i += 1;
            }
        }
    }
}

fn max_digit(array: &[u32]) -> u32 {
    let mut max_value = 0;
    for v in array {
        max_value = max(*v, max_value);
    }

    let mut max_digit = 0;
    let mut temp = max_value;
    while temp != 0 {
        max_digit += 1;
        temp /= 10;
    }
    max(1, max_digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut array = [882, 3, 5, 345, 254, 606, 588, 808, 535, 784, 715, 710];
        radix_sort(&mut array);
        println!("{:?}", array);
    }
}