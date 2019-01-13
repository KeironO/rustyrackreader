extern crate rscam;
extern crate image;
extern crate tempfile;

use std::fs::File;
use std::io::Write;

fn main() {
    let mut camera = rscam::new("/dev/video0").unwrap();

    let width: u32 = 160;
    let height: u32 = 120;


    camera.start(&rscam::Config {
        interval: (1, 10),
        resolution: (width, height),
        format: b"MJPG",
        ..Default::default()
    }).unwrap();

    let frame = camera.capture().unwrap();


    // Not ideal, but need to get real camera to see what output I will be working with
    let mut file = File::create("/tmp/frame.jpg").unwrap();
    file.write_all(&frame[..]).unwrap();

    let img = image::open("/tmp/frame.jpg").unwrap();
    let img = img.grayscale();

    println!("{:?}", img.color());
    // Convert to Ysomething and get into image file.
    // Write watershed algorithm in Rust, lol
    // Then test said algorithm using a photo taken from the last HDHBCRC meeting
    // Get QR codes read in via Rust.

    //println!("dimensions {:?}", img.to_rgb().dimensions());

    
}

