use std::ffi::OsString;
use windows::{
    Win32::Foundation::{HWND, PWSTR, RECT},
    Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowRect},
};
use screenshots::Screen;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::process::Command;
use std::env;
use std::result::Result;
use std::io::{Error, self, Write};
use std::{thread, time};
use chrono::Local;
use image::{DynamicImage, GrayImage};
use imageproc::contrast::{otsu_level, threshold};
use image::imageops::FilterType;

fn main() {
    let ten_millis = time::Duration::from_millis(500);
    let now = time::Instant::now();
    let outpath = &format!("out/{}.csv", Local::now().format("%Y%m%d%H%M%S"));
    let mut paststats:Vec<String> = Vec::new();

    let mut head:Vec<String> = vec![
        String::from("スピード"),
        String::from("スタミナ"),
        String::from("パワー"),
        String::from("根性"),
        String::from("賢さ"),
        String::from("スキルPt")
    ];
    match file_append(outpath, head) {
        Ok(()) => {
            println!("Logging init.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    loop {
        thread::sleep(ten_millis);
        //      println!("{:?}", paststats);
        let window_name = OsString::from("umamusume");
        let id: HWND = unsafe { FindWindowW(PWSTR::default(), window_name) };

        let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if unsafe { GetWindowRect(id, &mut rect)} != false {
            //   println!("{:?}", rect);
            let left:i32 = rect.left;
            let top:i32 = rect.top;
            let right:i32 = rect.right;
            let bottom:i32 = rect.bottom;
            let width:i32 = right-left;
            let height:i32 = bottom-top;
            let scr = Screen::from_point(left, top).unwrap();
            //   let image = scr.capture_area(left, top, width as u32, (height) as u32).unwrap();
            //   let buffer = image.buffer();
            //   fs::write("./temp/scr.png", &buffer).unwrap();

            // 「育成」文字列
            let image = scr.capture_area(left, (top+height/40), (width/2) as u32, (height/40) as u32).unwrap();
            let buffer = image.buffer();
            fs::write("./temp/scr_head.png", &buffer).unwrap();

            let path = env::current_dir().unwrap();
            let mut arg = &[format!("{}{}", path.to_string_lossy(), "\\temp\\scr_head.png")];
            let mut output = Command::new("bin/ocr.exe").args(arg).output().expect("failed");
            let mut str = String::from_utf8_lossy(&output.stdout).to_string();
            let mut v:Vec<&str> = str.split_whitespace().collect();
            let ikusei = v.join("");
            //    println!("{}", ikusei);

            if ikusei.starts_with("育成") || ikusei.starts_with("トレーニング") {
                // ステータス
                //    let image = scr.capture_area(left, (top + (height / 25 * 17)), (width) as u32, ((height / 25) - (height / 25 / 3 - 5)) as u32).unwrap();
                //    let buffer = image.buffer();
                //    fs::write("./temp/scr_status.png", &buffer).unwrap();
                get_screenshot(scr,left, (top + (height / 25 * 17)), (width) as u32, ((height / 33) - (height / 33 / 3 - 2)) as u32, 2, "./temp/scr_status.png");
                let mut st = get_text(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status.png"));
                println!("{}", st);

                // ステータス
                let mut y = (top+(height/25*17)+1);
                let _w:u32 = (width/10) as u32;
                let _h:u32 = ((height / 33) - (height / 33 / 3 - 2)) as u32;
                let _l = left + (width/10);
                let _alp = (width/20);
                let _pad = (width/600);

                get_screenshot(scr,_l + _pad*5, y, (_w-5), _h, 5, "./temp/scr_status_spd.png");
                let mut spd = get_text(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_spd.png"));
                //    if spd == "" {
                //        let y = (y+(height/20));
                //        get_screenshot(scr,_l + _pad*6, y, _w-(_w/7), _h, 2, "./temp/scr_status_spd.png");
                //        spd = get_text(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_spd.png"));
                //    }
                //    if spd == "" { continue; }

                get_screenshot(scr,(_l + (_alp + (_pad*8) + (_w) as i32) * 1) as i32, y, (_w-6), _h, 2, "./temp/scr_status_stm.png");
                let stm = get_text(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_stm.png"));
                //   if stm == "" { continue; }

                get_screenshot(scr,(_l + (_alp + (_pad*6) + (_w) as i32) * 2) as i32, y, (_w-6), _h, 6, "./temp/scr_status_pow.png");
                let pow = get_text(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_pow.png"));
                //   if pow == "" { continue; }

                get_screenshot(scr,(_l + (_alp + (_pad*5) + (_w) as i32) * 3) as i32, y, (_w-4), _h, 3, "./temp/scr_status_men.png");
                let men = get_text(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_men.png"));
                //    if men == "" { continue; }

                get_screenshot(scr,(_l + (_alp + (_pad*3) + (_w) as i32) * 4) as i32, y, _w, _h, 2, "./temp/scr_status_int.png");
                let int = get_text(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_int.png"));
                //   if int == "" { continue; }

                get_screenshot(scr,(_l + (_alp - (_pad*2) + (_w) as i32) * 5) as i32, y, (_w + (_w/3)), (_h + (_h/3)), 2, "./temp/scr_status_skl.png");
                let skl = get_text(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_skl.png"));
                //   if skl == "" { continue; }

                let mut stats: Vec<String> = vec![spd, stm, pow, men, int, skl];
                println!("{:?}", stats);

                if paststats != stats {
                    paststats = stats.to_vec();
                    println!("{:?}", paststats);
                    match file_append(outpath, stats) {
                        Ok(()) => {
                            println!("appended.");
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    };
                }
            }
        }
    }
}

fn file_append(path: &str, val: Vec<String>) -> Result<(), Error> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    f.write_all((val.join(",")+"\n").as_bytes())?;
    Ok(())
    //https://euniclus.com/article/rust_file_operations/
}

fn get_screenshot(scr: Screen, x: i32, y: i32, w: u32, h: u32, mag: u32, path: &str) {
    let img = scr.capture_area(x, y, w, h).unwrap();
    let img = image::load_from_memory(img.buffer());
    let img = img.as_ref().expect("REASON");
    let resize = img.resize(img.width()*mag, img.height()*mag, FilterType::Lanczos3);
    let mut gray_image = resize.to_luma8();
    let otsu_level = imageproc::contrast::otsu_level(&gray_image);
    let binarized_image = imageproc::contrast::threshold(&gray_image, otsu_level);
    binarized_image.save(path).unwrap();
}

fn get_text(cmd: String) -> String {
    let output = Command::new("bin/ocr.exe").args(&[format!("{}", cmd)]).output().expect("failed");
    let str = String::from_utf8_lossy(&output.stdout).to_string();
    let v: Vec<&str> = str.split_whitespace().collect();
    let cs: Vec<i32> = str.split_whitespace().filter_map(|k| k.parse().ok()).collect::<Vec<i32>>();
    let mut ret:String = String::new();

    for c in cs {
        ret += &c.to_string();
    }

    return ret;
}