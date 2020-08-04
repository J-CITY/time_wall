extern crate chrono;
extern crate argparse;

use std::io::{stdout, BufWriter};
use argparse::{ArgumentParser, Store, Collect};
use image::{Rgb, RgbImage, ImageBuffer};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use chrono::Local;
use std::fs;
use std::fs::File;
use std::io::Read;

struct Config {
    bgColor: Vec<u8>,
    fgColor: Vec<u8>,
    w: u32,
    h: u32,
    fontSize: f32,
    textPosX: u32,
    textPosY: u32,
    fileName: String,
    fontName: String
}

fn draw(conf: &Config) {
    let mut img = ImageBuffer::from_fn(conf.w, conf.h, |x, y| {
        image::Rgb([conf.bgColor[0], conf.bgColor[0], conf.bgColor[0]])
    });

    //println!("{0}", conf.fontName);
    let mut file = File::open(&conf.fontName).expect("no file found");
    let metadata = fs::metadata(&conf.fontName).expect("unable to read metadata");
    let mut font = vec![0; metadata.len() as usize];
    file.read(&mut font).expect("buffer overflow");
    //let font = Vec::from(data as &[u8]);
    //let font = Vec::from(conf.fontName.as_bytes() as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let scale = Scale {
        x: conf.fontSize,
        y: conf.fontSize,
    };

    let date = Local::now();
    let time = date.format("%H:%M").to_string();

    draw_text_mut(
        &mut img,
        Rgb([conf.fgColor[0], conf.fgColor[0], conf.fgColor[0]]),
        conf.textPosX,
        conf.textPosY,
        scale,
        &font,
        &time[..],
    );

    let _ = img.save(&conf.fileName).unwrap();
}

fn main() {
    let mut config = Config {
        bgColor: vec![0u8, 0u8, 0u8],
        fgColor: vec![255u8, 255u8, 255u8],
        w: 1920,
        h: 1080,
        fontSize: 32.0,
        textPosX: 1366,
        textPosY: 768,
        fileName: String::from("out.png"),
        fontName: String::from("src/font.ttf")
    };
    let mut bg = Vec::new();
    let mut fg = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Time wallpaper");
        //ap.refer(&mut fg)
        //    .add_option(&["-tc"], Collect,
        //    "Time color (Color from 0 to 255)");
        //ap.refer(&mut bg)
        //    .add_option(&["-bgc"], Collect,
        //    "Bg color (Color from 0 to 255)");
        //ap.refer(&mut config.w)
        //    .add_option(&["-w"], Store,
        //    "Image width");
        //ap.refer(&mut config.h)
        //    .add_option(&["-h"], Store,
        //    "Image height");
        //ap.refer(&mut config.fontSize)
        //    .add_option(&["-fs"], Store,
        //    "Font size");
        //ap.refer(&mut config.textPosX)
        //    .add_option(&["-x"], Store,
        //    "Text pos X");
        //ap.refer(&mut config.textPosY)
        //    .add_option(&["-y"], Store,
        //    "Text pos Y");
        //ap.refer(&mut config.fileName)
        //    .add_option(&["-fn"], Store,
        //    "Output file name");
        //ap.refer(&mut config.fontName)
        //    .add_option(&["-fnt"], Store,
        //    "Font path");
        //ap.parse_args_or_exit();
    }
    if fg.len() == 3 {
        config.fgColor =vec![fg[0], fg[1], fg[2]]
    }
    
    if bg.len() == 3 {
        config.bgColor = vec![bg[0], bg[1], bg[2]]
    }

    draw(&config)
}
