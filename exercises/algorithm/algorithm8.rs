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

pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    // 入栈：将元素加入非空队列（优先q1）
    pub fn push(&mut self, elem: T) {
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }

    // 出栈：转移非空队列的前n-1个元素到另一个队列，弹出最后一个元素
    pub fn pop(&mut self) -> Result<T, &str> {
        // 确定源队列（有元素）和目标队列（空）
        let (source, target) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else if !self.q2.is_empty() {
            (&mut self.q2, &mut self.q1)
        } else {
            // 两个队列都为空，栈空
            return Err("Stack is empty");
        };

        // 将源队列的前n-1个元素转移到目标队列
        let size = source.size();
        for _ in 0..size - 1 {
            let val = source.dequeue().unwrap(); // 源队列非空，unwrap安全
            target.enqueue(val);
        }

        // 弹出源队列剩余的最后一个元素（栈顶）
        source.dequeue()
    }

    // 栈是否为空：两个队列都为空时为空
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
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