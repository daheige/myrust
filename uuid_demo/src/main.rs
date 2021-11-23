use uuid::Uuid;
fn main() {
    println!("Hello, world!");
    // uuid v4 version
    let my_uuid = Uuid::new_v4();
    println!("uuid:{}", my_uuid);

    // 解析是否是一个合法的uuid
    let my_u = Uuid::parse_str("a55c20ac-5281-4e36-8250-820255632724");
    if let Ok(s) = my_u {
        println!("str:{}", s.to_urn());
    }
}
