use std::cmp;

fn bucket_sort(array: &mut [i32]) {
    // 确定最大值和最小值
    let mut max = 0;
    let mut min = 0;
    for v in &mut *array {
        max = cmp::max(*v, max);
        min = cmp::min(*v, min);
    }

    // 桶数量，可以根据实际情况选择
    let bucket_num = (max / 10 + min / 10 + 1) as usize;
    // 创建桶列表
    let mut buckets = vec![vec![]; bucket_num];

    // 将数据放入桶中
    for v in &mut *array {
        // 映射规则
        let pos = ((*v - min) / 10) as usize;
        let a = buckets.get_mut(pos).unwrap();
        a.push(*v);
    }

    let mut i = 0;
    for mut bucket in buckets {
        // 使用 Rust 内置排序函数对桶内元素排序
        bucket.sort();

        // 按顺序取出元素
        for v in bucket {
            array[i] = v;
            i += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut array = [29, 25, 3, 49, 9, 37, 21, 43];
        bucket_sort(&mut array);
        println!("{:?}", array);
    }
}