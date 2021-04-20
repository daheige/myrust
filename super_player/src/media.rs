pub trait Playable {
    fn play(&self);
    fn pause(&self) {
        println!("pause");
    }
}

pub struct Audio(String);
pub struct Video(String);

pub fn new_audio(s: String) -> Audio {
    Audio(s)
}

pub fn new_video(s: String) -> Video {
    Video(s)
}

impl Playable for Audio {
    fn play(&self) {
        println!("now playing: {}", self.0);
    }

    // 采用默认的pause方法
    // fn pause(&self) {
    //     println!("pause");
    // }
}

impl Playable for Video {
    fn play(&self) {
        println!("now playing song: {}", self.0);
    }

    // 重写pause方法
    fn pause(&self) {
        println!("pause");
    }
}

// 定义trait行为，也就是定义接口的一些方法集合
pub trait Vehicle {
    fn get_price(&self) -> u64;
}

// 通过组合的方式实现trait继承
pub trait Car: Vehicle {
    fn get_model(&self) -> String;
}

pub struct TeslaRoadster {
    model: String,
    price: u64,
}

impl TeslaRoadster {
    // 通过new方法创建一个self实例对象，这里Self表示当前对象实例
    pub fn new(model: &str, price: u64) -> Self {
        Self {
            model: model.to_string(),
            price,
        }
    }
}

impl Car for TeslaRoadster {
    fn get_model(&self) -> String {
        format!("tesla roadster {}", self.model)
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        self.price
    }
}
