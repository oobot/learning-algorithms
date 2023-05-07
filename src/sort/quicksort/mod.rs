use std::ptr::swap;

fn quicksort<T: Ord + Copy>(array: &mut [T]) {
    if array.len() > 1 {
        let pivot = array[0];
        let mut left = 0;
        let mut right = array.len() - 1;
        let mut pos = left;
        while left <= right { // 一次比较两个元素
            if (right >= pos && array[right] <= pivot) || (right <= pos && array[right] >= pivot) {
                array[pos] = array[right];
                pos = right;
            }
            right -= 1;

            if (left >= pos && array[left] <= pivot) || (left <= pos && array[left] >= pivot) {
                array[pos] = array[left];
                pos = left;
            }
            left += 1;
        }
        array[pos] = pivot;

        quicksort(&mut array[0..pos]);
        quicksort(&mut array[(pos +1)..]);
    }
}

fn quicksort2<T: Ord + Copy>(array: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot = left;
        let mut index = pivot + 1; // 交换元素的位置
        for i in index..=right { // 一次只比较一个元素
            if array[i] < array[pivot] {
                array.swap(i, index);
                index += 1;
            }
        }
        array.swap(pivot, index - 1); // 循环中交换了多少个值，index 的值就增加了多少
        let partition = index - 1;

        if partition > 0 {
            quicksort2(array, left, partition - 1);
        }
        quicksort2(array, partition + 1, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut array = vec![5, 7, 1, 8, 4];
        // let mut array = vec![8, 7];
        // let mut array = vec![5, 8, 7];

        quicksort(&mut array);
        println!("{:?}", array);


        // let length = array.len() - 1;
        // quicksort(&mut array, 0, length);
        // println!("{:?}", array);
    }

}
