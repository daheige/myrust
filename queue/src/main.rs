fn main() {
    println!("Hello, world!");

    // let mut q = Queue::new();
    let mut q = Queue::new_with_cap(3);
    q.push(1);
    q.push(2);
    q.push(3);
    // 这里判断是否可以push数据
    if let Err(e) = q.push(4) {
        println!("{}", e);
    }

    println!("q len: {}", q.size());
    let len = q.size();
    for i in 0..len + 1 {
        println!("index: {}", i);
        let v = q.pop();
        if let Some(data) = v {
            println!("current value: {}", data);
        } else {
            println!("current value: {}", "empty");
        }
    }
}

// 实现一个队列，先进先出模式
struct Queue<T> {
    queue: Vec<T>,
    cap: usize,
}

impl<T> Queue<T> {
    fn new_with_cap(size: usize) -> Self {
        let queue: Vec<T> = Vec::new();
        Queue {
            queue: queue,
            cap: size,
        }
    }

    fn new() -> Self {
        let queue: Vec<T> = Vec::new();
        Queue {
            queue: queue,
            cap: 0,
        }
    }

    fn push(&mut self, item: T) -> Result<(), String> {
        if self.cap > 0 && self.size() >= self.cap {
            return Err("no cap can push".to_string());
        }

        self.queue.push(item);
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.size() > 0 {
            let v = self.queue.remove(0);
            Some(v)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.queue.len()
    }
}
