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

fn main() {
    let ten_millis = time::Duration::from_millis(500);
    let now = time::Instant::now();
    let outpath = &format!("out/{}.csv", Local::now().format("%Y%m%d%H%M%S"));
    let mut paststats = Vec::new();

    loop {
        thread::sleep(ten_millis);
        println!("{:?}", paststats);
        let window_name = OsString::from("umamusume");
        let id: HWND = unsafe { FindWindowW(PWSTR::default(), window_name) };

        let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if unsafe { GetWindowRect(id, &mut rect)} != false {
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
            let image = scr.capture_area(left, (top+height/40), (width/5) as u32, (height/40) as u32).unwrap();
            let buffer = image.buffer();
            fs::write("./temp/scr_head.png", &buffer).unwrap();

            let path = env::current_dir().unwrap();
            let mut arg = &[format!("{}{}", path.to_string_lossy(), "\\temp\\scr_head.png")];
            let mut output = Command::new("bin/ocr.exe").args(arg).output().expect("failed");
            let mut str = String::from_utf8_lossy(&output.stdout).to_string();
            let mut v:Vec<&str> = str.split_whitespace().collect();
            let ikusei = v.join("");

            if "育成" == ikusei || "トレーニング" == ikusei {
                // ステータス
                let image = scr.capture_area(left, (top + (height / 25 * 17)), (width) as u32, ((height / 25) - (height / 25 / 3 - 5)) as u32).unwrap();
                let buffer = image.buffer();
                fs::write("./temp/scr_status.png", &buffer).unwrap();

                let mut arg = &[format!("{}{}", path.to_string_lossy(), "\\temp\\scr_status.png")];
                let mut output = Command::new("bin/ocr.exe").args(arg).output().expect("failed");
                let mut str = String::from_utf8_lossy(&output.stdout).to_string();
                let mut v: Vec<&str> = str.split(" ").collect();
                //   println!("{}", str);
                //   println!("{:?}", v);
                //   println!("{}", v.len());
                //   println!("{}", v[0]);

                if v.len() > 2 {
                    v.push("temp"); //tmp
                    let mut stats: Vec<String> = Vec::new();
                    let mut tmp = v[0].to_string();
                    v.remove(0);

                    while stats.len() < 6 {
                        if tmp == "1" || tmp == "11" || tmp == "111" {
                            tmp = format!("{}{}", tmp, v.first().expect("REASON").to_string());
                            v.remove(0);
                            continue;
                        } else {
                            //    println!("{:?}", stats);
                            //    println!("{}", tmp);
                            //    println!("{:?}", v);
                            //    println!("{}", "----------");
                            stats.push(tmp);
                            tmp = v.first().expect("REASON").to_string();

                            if v.len() > 0 {
                                v.remove(0);

                                if tmp == "!" {
                                    tmp = v.first().expect("REASON").to_string();
                                    v.remove(0);
                                }
                            }
                        }
                    }

                    if paststats != stats {
                        paststats = stats.to_vec();
                        //   println!("{}", outpath);
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
}

fn file_append(path: &str, val: Vec<String>) -> Result<(), Error> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    f.write_all(val.join(",").as_bytes())?;
    Ok(())
    //https://euniclus.com/article/rust_file_operations/
}
