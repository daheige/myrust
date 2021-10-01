pub mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_max(){
        // 测试上一层的common max函数
        println!("max(12,10) = {}",super::common::max(12,10));
    }
}
