fn main() {
    println!("Hello, world!");
    let mut s = Stack::new(4);
    let _ = s.push(1); // _忽略返回结果
    let _ = s.push(2);
    let _ = s.push(3);
    if let Err(e) = s.push(5) {
        println!("push error: {}", e)
    }

    println!("stack:{:#?}", s.data);
}

struct Stack<T> {
    data: Vec<T>,
    top: usize,
    cap: usize,
}

impl<T> Stack<T> {
    fn new(cap: usize) -> Self {
        Self {
            data: Vec::new(),
            top: 0,
            cap,
        }
    }

    fn push(&mut self, item: T) -> Result<(), String> {
        if self.top >= self.cap {
            return Err("no cap can push".to_string());
        }

        self.data.push(item);
        self.top += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }

        self.top -= 1;
        self.data.pop()
    }

    fn size(&self) -> usize {
        self.top
    }
}
