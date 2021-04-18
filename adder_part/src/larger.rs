pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    // 是否包含other形状
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
