mod media;

use media::Car;
use media::Playable;
use media::TeslaRoadster; // 实例实现了Car,Playable
use media::Vehicle;

fn main() {
    println!("Hello, world!");
    let audio = media::new_audio(String::from("beyond"));
    let video = media::new_video(String::from("hello,abc"));
    audio.play();
    video.play();

    video.pause();

    let r = TeslaRoadster::new("tesla res1", 123);
    println!("model: {}", r.get_model());
    println!("price: {}", r.get_price()); // 调用Vehicle接口上的方法
}

/*
Hello, world!
now playing: beyond
now playing song: hello,abc
pause
model: tesla roadster tesla res1
price: 123
 */
