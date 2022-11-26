use std::ffi::OsString;
use windows::{
    Win32::Foundation::{HWND, PWSTR, RECT},
    Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowRect},
};
use screenshots::Screen;
use std::fs;
use std::fs::OpenOptions;
use std::process::Command;
use std::env;
use std::path::Path;
use std::result::Result;
use std::io::{Error, Write};
use std::{thread, time};
use chrono::Local;
use imageproc::contrast::{otsu_level, threshold};
use image::{imageops::FilterType, DynamicImage};
use std::time::{Duration, Instant};

//use regex::Regex; // toml -> regex = "1.6.0"

mod ocr;

fn main() {
    let sleeptime = time::Duration::from_millis(500);
    let args: Vec<String> = env::args().collect();
    let mut outpath = format!("out/{}.tsv", Local::now().format("%Y%m%d%H%M%S"));
    if args.len() == 2 {
        outpath = env::args().nth(1).expect("");
    }

    let mut paststats:Vec<String> = Vec::new();
    let head:Vec<String> = vec![
        String::from("時期"),
        String::from("スピード"),
        String::from("スタミナ"),
        String::from("パワー"),
        String::from("根性"),
        String::from("賢さ"),
        String::from("スキルPt")
    ];

    if !Path::new(&outpath).exists() {
        match file_append(&outpath, head) {
            Ok(()) => {
                println!("Logging init. {}", outpath);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    } else {
        println!("Append file for {}", outpath);
    }

    loop {
  //      println!("{:?}", paststats);
        let id: HWND = unsafe { FindWindowW(PWSTR::default(), OsString::from("umamusume")) };
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
        //    let image = scr.capture_area(left, top, width as u32, (height) as u32).unwrap();
        //    let buffer = image.buffer();
        //    fs::write("./temp/scr.png", &buffer).unwrap();

//            let start = Instant::now();
            // 「育成」文字列
            let image = scr.capture_area(left, top+height/40, (width/4) as u32, (height/38) as u32).unwrap();
            let buffer = image.buffer();
            fs::write("./temp/scr_head.png", &buffer).unwrap();
            let path = env::current_dir().unwrap();
            let ikusei = get_text(&"\\temp\\scr_head.png");
        //    println!("{}", ikusei);

            // ステータス
            let mut y = (top+(height/25*17)+1);
            let _w:u32 = (width/10) as u32;
            let _h:u32 = ((height / 33) - (height / 33 / 3 - 2)) as u32;
            let _l = left + (width/10);
            let _alp = (width/20);
            let _pad = (width/600);

            if (ikusei.starts_with("育成") && !ikusei.starts_with("育成完了")) || ikusei.starts_with("トレーニング") {
                get_screenshot(scr,left+width/36*8, (top+height/19), (width/3) as u32, (height/40) as u32, 2, "./temp/scr_season.png");
                let season = get_text("\\temp\\scr_season.png");

                get_screenshot_grayscale(scr,left+width/30*5, (top+height/5+(_pad*6)), (width-(width/2)) as u32, (height/34) as u32, 2, "./temp/scr_ikusei_event.png");

                get_screenshot(scr,_l + _pad*3, y, _w-5, _h, 2, "./temp/scr_status_spd.png");
                let mut spd = get_text_tesseract(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_spd.png"));
                if !is_status_str(&spd) {
                    y = (y+(height/20));
                    get_screenshot(scr, _l + _pad*5, y, _w-(_w/7), _h, 2, "./temp/scr_status_spd.png");
                    spd = get_text_tesseract(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_spd.png"));
                }
                if !is_status_str(&spd) { continue; }

                get_screenshot(scr,(_l + (_alp + (_pad*8) + (_w) as i32) * 1) as i32, y, (_w-6), _h, 2, "./temp/scr_status_stm.png");
                let stm = get_text_tesseract(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_stm.png"));
                if !is_status_str(&stm) { continue; }

                get_screenshot(scr,(_l + (_alp + (_pad*5) + (_w) as i32) * 2) as i32, y, (_w-4), _h, 2, "./temp/scr_status_pow.png");
                let pow = get_text_tesseract(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_pow.png"));
                if !is_status_str(&pow) { continue; }

                get_screenshot(scr,(_l + (_alp + (_pad*4) + (_w) as i32) * 3) as i32, y, (_w-2), _h, 2, "./temp/scr_status_men.png");
                let men = get_text_tesseract(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_men.png"));
                if !is_status_str(&men) { continue; }

                get_screenshot(scr,(_l + (_alp + (_pad*5) + (_w) as i32) * 4) as i32, y, _w-6, _h, 2, "./temp/scr_status_int.png");
                let int = get_text_tesseract(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_int.png"));
                if !is_status_str(&int) { continue; }

                get_screenshot(scr,(_l + (_alp - (_pad*2) + (_w) as i32) * 5) as i32, y, (_w + (_w/4)), (_h + (_h/3)), 2, "./temp/scr_status_skl.png");
                let skl = get_text_tesseract(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_skl.png"));
                if !is_skillpt_str(&skl) { continue; }

                //    let mut stats:Vec<&str> = st.split_whitespace().collect();
            //    stats.push(&skl);

                let checker: Vec<String> = vec![spd.clone(), stm.clone(), pow.clone(), men.clone(), int.clone(), skl.clone()];
            //    println!("FULL: {:?}", checker);

                if paststats != checker {
                    paststats = checker.to_vec();
                    let stats: Vec<String> = vec![season, spd, stm, pow, men, int, skl];
                    match file_append(&outpath, stats) {
                        Ok(()) => {
 //                           let end = start.elapsed();
 //                           println!("appended.[{}.{:3}sec]", end.as_secs(), end.subsec_nanos() / 1_000_000);
                            println!("{:?}", paststats);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    };
                }

            } else if ikusei.starts_with("育成完了") {
                let mut results:Vec<String> = Vec::new();
                results.push(String::from("育成完了"));
                get_screenshot(scr,(left + width / 24 * 19) as i32, top + height / 24 * 7, _w + (_w/4), _h * 8, 2, "./temp/scr_status_comp.png");
                let stats:Vec<String> = get_text_tesseract_v(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_comp.png"));
                
                if stats.len() == 5 {

                    for s in stats {
                        results.push(s);
                    }

                    get_screenshot(scr,(left + (width / 24 * 5) - 8) as i32, top + height / 48 * 41, _w-_w/3, _h, 2, "./temp/scr_status_comp_skill.png");
                    let mut skill = get_text_tesseract(format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status_comp_skill.png"));
                    if !is_skillpt_str(&skill) {
                        skill = get_text("\\temp\\scr_status_comp_skill.png");
                        println!("{}", skill);
                    }
                    if !is_skillpt_str(&skill) { continue; }
                    
                    results.push(skill);
                    println!("{:?}", results);
                    
                    match file_append(&outpath, results) {
                        Ok(()) => {
                            println!("Complete!");
                            return;
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    };
                }
            } else {
                thread::sleep(sleeptime);
            }
        } else {
            thread::sleep(sleeptime);
        }
    }
}

fn file_append(path: &str, val: Vec<String>) -> Result<(), Error> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    f.write_all((val.join("\t")+"\n").as_bytes())?;
    Ok(())
}

fn get_screenshot(scr: Screen, x: i32, y: i32, w: u32, h: u32, mag: u32, path: &str) {
    let image = run_screenshot(scr, x, y, w, h, mag);
    let gray_image = image.to_luma8();
    let otsu_level = imageproc::contrast::otsu_level(&gray_image);
    let binarized_image = imageproc::contrast::threshold(&gray_image, otsu_level);
    binarized_image.save(path).unwrap();
}

fn get_screenshot_grayscale(scr: Screen, x: i32, y: i32, w: u32, h: u32, mag: u32, path: &str) {
    let image = run_screenshot(scr, x, y, w, h, mag);
    let gray_image = image.to_luma8();
    gray_image.save(path).unwrap();
}

fn run_screenshot(scr: Screen, x: i32, y: i32, w: u32, h: u32, mag: u32) -> DynamicImage {
    let img = scr.capture_area(x, y, w, h).unwrap();
    let img = image::load_from_memory(img.buffer());
    let img = img.as_ref().unwrap();
    let resize = img.resize(img.width()*mag, img.height()*mag, FilterType::Lanczos3);
    return resize;
}

fn get_text(path: &str) -> String {
    let cd = env::current_dir().unwrap();
    let path = &format!("{}{}", cd.to_string_lossy(), path);
    let str = ocr::get_ocr_text(path);
    let v:Vec<&str> = str.split_whitespace().collect();
    let r = v.join("").replace("音成", "育成");

    return r;
}

fn get_text_tesseract(cmd: String) -> String {
    let output = Command::new("bin/Tesseract-OCR/tesseract.exe").args(&[format!("{}", cmd), "temp/ret".to_string(), "-l jpn".to_string()]).output().expect("failed");
    String::from_utf8_lossy(&output.stdout);
    let str = fs::read_to_string("temp/ret.txt");
    let temp = str.expect("REASON").replace("O", "0").replace("o", "0").replace("H", "1").replace("z", "2").replace("Z", "2").replace("?", "7");
//    let v: Vec<&str> = temp.split_whitespace().collect();
    let v: Vec<i32> = temp.split("").filter_map(|k| k.parse().ok()).collect::<Vec<i32>>();
    let mut ret:String = String::new();
//    let re = Regex::new(r"(\d+)").unwrap();

    for c in v {
      //  let n:Vec<&str> = c.matches("01234567689").collect();
      //  let cap = re.captures(c).unwrap();
      //  println!("{:?}", cap);
        ret += &c.to_string();
    }

    return ret;
}

fn get_text_tesseract_v(cmd: String) -> Vec<String> {
    let output = Command::new("bin/Tesseract-OCR/tesseract.exe").args(&[format!("{}", cmd), "temp/ret".to_string(), "-l jpn".to_string()]).output().expect("failed");
    String::from_utf8_lossy(&output.stdout);
    let str = fs::read_to_string("temp/ret.txt").unwrap();
    let v:Vec<&str> = str.split_whitespace().collect();
    let mut ret = Vec::new();

    for s in v {
        if !is_status_str(s) { continue; }

        ret.push(s.to_string());
    }

    return ret;
}

fn is_status_str(status: &str) -> bool {
    if status == "" { return false; }

    let n:Vec<&str> = status.matches(char::is_numeric).collect();
    if status != n.join("") { return false; }

    let cnt = status.chars().count();
    if cnt > 4 { return false; }
    if cnt < 2 { return false; }

    let n:Vec<char> = status.chars().collect();
    let first_str:String = n[0].to_string();
    if cnt == 4 && first_str != "1" { return false; }
    if cnt == 2 && first_str != "8" && first_str != "9" { return false; }
 
    return true;
}

fn is_skillpt_str(skillpt: &str) -> bool {
    if skillpt == "" { return false; }

    let n:Vec<&str> = skillpt.matches(char::is_numeric).collect();
    if skillpt != n.join("") { return false; }

    let cnt = skillpt.chars().count();
    if cnt > 4 { return false; }
    if cnt < 3 { return false; }
 
    return true;
}
