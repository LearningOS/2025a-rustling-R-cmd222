/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			q1: Queue::<T>::new(),
			q2: Queue::<T>::new(),
        }
    }

    // push 操作很简单，直接将元素加入主队列 q1 即可。
    // 时间复杂度: O(1)
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    // pop 操作是核心，需要将 q1 中除了最后一个元素之外的所有元素
    // 转移到 q2，然后取出 q1 中剩下的那个元素，最后再交换 q1 和 q2。
    // 时间复杂度: O(n)
    pub fn pop(&mut self) -> Result<T, &str> {
           // 如果主队列为空，则栈为空。
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // 1. 将 q1 中的元素逐个出队并入队到 q2，直到 q1 只剩下一个元素。
        while self.q1.size() > 1 {
            // 我们知道队列不为空，所以可以安全地 unwrap。
            if let Ok(elem) = self.q1.dequeue() {
                self.q2.enqueue(elem);
            }
        }
        
        // 2. 关键修复：调用 dequeue() 并立即处理 Result。
        //    我们知道此时 dequeue() 不会失败，所以可以使用 unwrap()。
        //    .unwrap() 会消耗掉 Result，并返回一个类型为 T 的自有值。
        //    在这一行代码执行完毕后，对 self.q1 的可变借用就彻底结束了。
        let popped_value: T = self.q1.dequeue().unwrap();

        // 3. 现在 self.q1 没有被借用，我们可以安全地进行 swap 操作。
        std::mem::swap(&mut self.q1, &mut self.q2);
        
        // 4. 返回我们之前取出的自有值，并用 Ok() 包装它。
        Ok(popped_value)
    }

    // 栈是否为空，取决于主队列 q1 是否为空。
    // 时间复杂度: O(1)
    pub fn is_empty(&self) -> bool {
		self.q1.is_empty()
    }
}
#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}