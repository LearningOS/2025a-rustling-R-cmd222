/*
	heap
	This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
           // 1. 将新元素添加到数组的末尾。
        self.count += 1;
        self.items.push(value);

        // 2. "上浮" (Heapify Up / Swim)
        // 从新添加的节点开始，向上与其父节点比较。
        // 如果它比父节点“更优”（根据 comparator），就交换它们的位置。
        // 重复此过程，直到它到达根节点或不再需要交换。
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // `(self.comparator)` 是调用函数指针的语法。
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
    }
}
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
  let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 检查右子节点是否存在。
        if right_idx <= self.count {
            // 如果两个子节点都存在，比较它们，返回“更优”的那个的索引。
            if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
                left_idx
            } else {
                right_idx
            }
        } else {
            // 如果只有左子节点存在，直接返回左子节点的索引。
            left_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
       // 如果堆为空，返回 None。
        if self.count == 0 {
            return None;
        }

        // 1. 堆的根（索引 1）就是我们要返回的元素。
        //    我们先把它和数组的最后一个元素交换。
        self.items.swap(1, self.count);
        self.count -= 1;

        // 2. 使用 `pop()` 移除并获取刚刚交换到末尾的（原来的）根元素。
        //    这样做的效率比 `remove(1)` 要高。
        let result = self.items.pop();
        
        // 3. "下沉" (Heapify Down / Sink)
        //    现在根节点（索引 1）上的元素是错误的（原来是最后一个元素）。
        //    我们需要将它向下移动到正确的位置。
        let mut idx = 1;
        while self.children_present(idx) {
            let smaller_child_idx = self.smallest_child_idx(idx);
            // 比较当前节点和它“更优”的子节点。
            if (self.comparator)(&self.items[smaller_child_idx], &self.items[idx]) {
                self.items.swap(idx, smaller_child_idx);
                idx = smaller_child_idx;
            } else {
                break;
            }
        }

        result
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}