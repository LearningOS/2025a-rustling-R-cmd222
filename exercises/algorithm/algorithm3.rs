/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T:Ord>(array: &mut [T]){
	//TODO
    let len = array.len();
    // 如果数组长度小于2，则它已经是有序的，无需任何操作。
    if len < 2 {
        return;
    }

    // 外层循环控制需要进行多少轮“冒泡”。
    // 每一轮都会将当前未排序部分的最大元素“冒泡”到正确的位置。
    for i in 0..len {
        // 内层循环执行实际的比较和交换。
        // `len - i - 1` 是一个优化：因为每轮结束后，
        // 末尾的元素都已经是就位的，所以我们不需要再比较它们。
        for j in 0..(len - i - 1) {
            // 如果左边的元素大于右边的元素，就交换它们的位置。
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}