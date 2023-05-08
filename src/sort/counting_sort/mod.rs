use std::cmp;

fn counting_sort(array: &[i32]) -> Vec<i32> {
    let mut max = 0;
    let mut min = 0;
    for v in array {
        max = cmp::max(max, *v);
        min = cmp::min(min, *v);
    }

    let mut counting = vec![0; (max - min + 1) as usize];
    for v in array {
        let i = (v - min) as usize;
        counting[i] += 1;
    }

    let mut new_array = Vec::with_capacity(array.len());
    for (i, mut count) in counting.into_iter().enumerate() {
        while count > 0 {
            new_array.push(i as i32 + min);
            count -= 1;
        }
    }

    new_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let array = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        let list = counting_sort(&array);
        println!("{:?}", list);
    }
}