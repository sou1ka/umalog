use windows::{
    Win32::Foundation::{HWND, RECT},
    Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowRect},
    core::PCWSTR,
};
use std::time::Duration;
use std::ffi::OsString;
use std::fs;
use std::fs::OpenOptions;
use std::process::Command;
use std::env;
use std::path::Path;
use std::result::Result;
use std::io::{Error, Write};
use std::{thread, time};
use chrono::Local;
use image::{imageops::FilterType, DynamicImage, Rgb};
mod screenshot;

mod ocr;

fn main() {
    let (sleeptime, tempdir, outpath, mut paststats) = initialize();

    loop {
        umalog(sleeptime, tempdir.to_string(), outpath.to_string(), &mut paststats);
    }
}

fn initialize() -> (Duration, String, String, Vec<String>) {
    let sleeptime = time::Duration::from_millis(500);
    let args: Vec<String> = env::args().collect();
    let tempdir = String::from(env::temp_dir().to_str().unwrap().to_string() + "umalog\\");
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
        String::from("スキルPt"),
        String::from("やる気")
    ];

    if !Path::new(&tempdir).exists() {
        fs::create_dir_all(&tempdir).unwrap();
    }

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

    return (sleeptime, tempdir, outpath, paststats);
}

fn umalog(sleeptime: Duration, tempdir: String, outpath: String, paststats: &mut Vec<String>) {
    //      println!("{:?}", paststats);
    let target = "umamusume";
    let id: HWND = unsafe { FindWindowW(PCWSTR::default(), OsString::from(target)) };
    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };

    if unsafe { GetWindowRect(id, &mut rect)} != false {
//       println!("{:?}", rect);
//        let left:i32 = rect.left;
//        let top:i32 = rect.top;
//        let right:i32 = rect.right;
//        let bottom:i32 = rect.bottom;
//        let width:i32 = right-left;
//        let height:i32 = bottom-top;
//        let screen = Screen::from_point(left, top);

//        if screen.is_none() {
//            return;
//        }

    //    let scrs = screen.unwrap();println!("{}", scrs.len());
//        let scr = screen.unwrap();
    //    let image = scr.capture_area(left, top, width as u32, (height) as u32).unwrap();
    //    let image = scr.capture().unwrap();
    //    println!("w:{}, h:{}", image.width(), image.height());
    //    let buffer = image.buffer();
    //    fs::write("./temp/scr.png", &buffer).unwrap();
    //    let image = scrs[1].capture().unwrap();
    //    let buffer = image.buffer();
    //    fs::write("./temp/scr1.png", &buffer).unwrap();
        screenshot::capture(target, &(tempdir.clone() + "scr.png"));
        let img = &image::open(&(tempdir.clone() + "scr.png")).unwrap();
        let left:i32 = 0;
        let top:i32 = 0;
        let width:i32 = img.width().try_into().unwrap();
        let height:i32 = img.height().try_into().unwrap();

//            let start = Instant::now();
        // 「育成」文字列
    //    let image = scr.capture_area(left+10, top+33, (width/4) as u32, (height/38) as u32).unwrap();
    //    let buffer = image.buffer();
    //    fs::write(tempdir.clone() + "scr_head.png", &buffer).unwrap();
    //    let path = env::current_dir().unwrap();
        get_screenshot(img.clone(), left+10, top+48, (width/4) as u32, (height/37) as u32, 2, tempdir.clone() + "scr_head.png");
    //    let resize = image::open(tempdir.clone() + "scr_head.png").unwrap().resize(560 as u32, 110 as u32, FilterType::Lanczos3);
    //    resize.save(tempdir.clone() + "scr_head.png").unwrap();
        let mut ikusei = get_text(tempdir.clone() + "scr_head.png");println!("ikusei1:{}", ikusei);
        if ikusei == "" {
            ikusei = get_text_tesseract_jp(tempdir.clone() + "scr_head.png", tempdir.clone());
        }
        //println!("ikusei2:{}", ikusei);//println!("{}", width/4);

        // ステータス
//        let mut y = top+(height/25*16)+1;
        let mut y = (height as f32 * 0.6715587).floor() as i32;
        let _w:u32 = (width as f32 * 0.0952813).ceil() as u32;
        let _h:u32 = (height as f32 * 0.0227732).floor() as u32;
        let _l = (width as f32 * 0.0934664).floor() as i32;
        let _alp = width/20;
        let _pad = width/600;

        if (ikusei.starts_with("育成") && !ikusei.starts_with("育成完")) || ikusei.starts_with("トレーニング") || ikusei.starts_with("Fla-2‘/7") || ikusei.starts_with("SSマッチ") {
            get_screenshot(img.clone(),(width as f32 * 0.22686).floor() as i32, (height as f32 * 0.05364).floor() as i32, (width as f32 * 0.34482).floor() as u32, (height as f32 * 0.022773).floor() as u32 as u32, 2, tempdir.clone() + "scr_season.png");
            let season = get_text(tempdir.clone() + "scr_season.png");
                //println!("sea:{}", season);

            get_screenshot_grayscale(img.clone().clone(), (width as f32 * 0.154264).floor() as i32, (height as f32 * 0.205465).floor() as i32, (width as f32 * 0.594373).ceil() as u32, (height as f32 * 0.036437).floor() as u32, 2, tempdir.clone() + "scr_ikusei_event.png");
            let event = get_text(tempdir.clone() + "scr_ikusei_event.png");
                //println!("eve:{}", event);
            save_file(tempdir.clone() + "scr_ikusei_event.txt", event);

            get_screenshot(img.clone(),_l, y, _w, _h, 2, tempdir.clone() + "scr_status_spd.png");
            let mut spd = get_text_tesseract(tempdir.clone() + "scr_status_spd.png", tempdir.clone());
            if !is_status_str(&spd) {
                y = (height as f32 * 0.720141).floor() as i32;
                get_screenshot(img.clone(),_l, y, _w, _h, 2, tempdir.clone() + "scr_status_spd.png");
                spd = get_text_tesseract(tempdir.clone() + "scr_status_spd.png", tempdir.clone());
            }
                //println!("spd:{}", spd);
            if !is_status_str(&spd) { return; }

            get_screenshot(img.clone(),(width as f32 * 0.245009).floor() as i32, y, _w, _h, 2, tempdir.clone() + "scr_status_stm.png");
            let stm = get_text_tesseract(tempdir.clone() + "scr_status_stm.png", tempdir.clone());
                //println!("stm:{}", stm);
            if !is_status_str(&stm) { return; }

            get_screenshot(img.clone(),(width as f32 * 0.4010889).floor() as i32, y, _w, _h, 2, tempdir.clone() + "scr_status_pow.png");
            let pow = get_text_tesseract(tempdir.clone() + "scr_status_pow.png", tempdir.clone());
                //println!("pow:{}", pow);
            if !is_status_str(&pow) { return; }

            get_screenshot(img.clone(),(width as f32 * 0.55263).floor() as i32, y, _w, _h, 2, tempdir.clone() + "scr_status_men.png");
            let men = get_text_tesseract(tempdir.clone() + "scr_status_men.png", tempdir.clone());
                //println!("men:{}", men);
            if !is_status_str(&men) { return; }

            get_screenshot(img.clone(),(width as f32 * 0.70508).floor() as i32, y, _w, _h, 2, tempdir.clone() + "scr_status_int.png");
            let int = get_text_tesseract(tempdir.clone() + "scr_status_int.png", tempdir.clone());
                //println!("int:{}", int);
            if !is_status_str(&int) { return; }

            get_screenshot(img.clone(),(width as f32 * 0.816696).floor() as i32, y, (width as f32 * 0.1279491).ceil() as u32, (height as f32 * 0.03238).ceil() as u32, 2, tempdir.clone() + "scr_status_skl.png");
            let skl = get_text_tesseract(tempdir.clone() + "scr_status_skl.png", tempdir.clone());
                //println!("skl:{}", skl);
            if !is_skillpt_str(&skl) { return; }

            get_screenshot_color(img.clone(),(width as f32 * 0.775862).floor() as i32, (height as f32 * 0.132085).floor() as i32, (width as f32 * 0.090744).floor() as u32, (height as f32 * 0.02834).floor() as u32, 2, tempdir.clone() + "scr_status_con.png");
            let con = get_condition(tempdir.clone() + "scr_status_con.png");
                //println!("condition {}", con);
            if &con == "" { return; }

            //    let mut stats:Vec<&str> = st.split_whitespace().collect();
        //    stats.push(&skl);

            let checker: Vec<String> = vec![spd.clone(), stm.clone(), pow.clone(), men.clone(), int.clone(), skl.clone(), con.clone()];
                //println!("FULL: {:?}", checker);

            if *paststats != checker {
                *paststats = checker;
                let stats: Vec<String> = vec![season, spd, stm, pow, men, int, skl, con];
                match file_append(&outpath, stats) {
                    Ok(()) => {
//                           let end = start.elapsed();
//                           println!("appended.[{}.{:3}sec]", end.as_secs(), end.subsec_nanos() / 1_000_000);
                        println!("{:?}", &paststats);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                };
            }

        } else if ikusei.starts_with("育成完") || ikusei.starts_with("育完了") {
            let mut results:Vec<String> = Vec::new();
            results.push(String::from("育成完了"));
            get_screenshot(img.clone(),(width as f32 * 0.760435).floor() as i32, (height as f32 * 0.306174).floor() as i32, (width as f32 * 0.127041).ceil() as u32, (height as f32 * 0.144736).ceil() as u32, 2, tempdir.clone() + "scr_status_comp.png");
            let stats:Vec<String> = get_text_tesseract_v(tempdir.clone() + "scr_status_comp.png", tempdir.clone());
                //println!("stats:{:?}", stats);

            if stats.len() == 5 {

                for s in stats {
                    results.push(s);
                }

                get_screenshot(img.clone(),(width as f32 * 0.181488).floor() as i32, (height as f32 * 0.84008).floor() as i32, (width as f32 * 0.068058).ceil() as u32, (height as f32 * 0.0161943).ceil() as u32, 2, tempdir.clone() + "scr_status_comp_skill.png");
                let mut skill = get_text_tesseract(tempdir.clone() + "scr_status_comp_skill.png", tempdir.clone());
                if !is_skillpt_str(&skill) {
                    skill = get_text2num(tempdir.clone() + "scr_status_comp_skill.png");
                    //println!("{}", skill);
                }
                if !is_skillpt_str(&skill) { // グランドライブ以外
                    get_screenshot(img.clone(),(width as f32 * 0.303085).floor() as i32, (height as f32 * 0.84008).floor() as i32, (width as f32 * 0.08892).ceil() as u32, (height as f32 * 0.020242).ceil() as u32, 2, tempdir.clone() + "scr_status_comp_skill.png");
                    skill = get_text2num(tempdir.clone() + "scr_status_comp_skill.png");
                    //println!("グラライ以外, {}", &skill);
                }
                //println!("{}", skill);
                if !is_skillpt_str(&skill) { return; }
                
                results.push(skill);
                //println!("{:?}", results);
                results.push("".to_string()); // やる気
                
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

fn file_append(path: &str, val: Vec<String>) -> Result<(), Error> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    f.write_all((val.join("\t")+"\n").as_bytes())?;
    Ok(())
}

fn get_screenshot(image: DynamicImage, x: i32, y: i32, w: u32, h: u32, mag: u32, path: String) {
    let image = run_screenshot(image, x, y, w, h, mag);
    let gray_image = image.to_luma8();
    let otsu_level = imageproc::contrast::otsu_level(&gray_image);
    let binarized_image = imageproc::contrast::threshold(&gray_image, otsu_level);
    binarized_image.save(path.as_str()).unwrap();
}

fn get_screenshot_grayscale(image: DynamicImage, x: i32, y: i32, w: u32, h: u32, mag: u32, path: String) {
    let image = run_screenshot(image, x, y, w, h, mag);
    let gray_image = image.to_luma8();
    gray_image.save(path.as_str()).unwrap();
}

fn get_screenshot_color(image: DynamicImage, x: i32, y: i32, w: u32, h: u32, mag: u32, path: String) {
    let image = run_screenshot(image, x, y, w, h, mag);
    image.save(path.as_str()).unwrap();
}

fn run_screenshot(image: DynamicImage, x: i32, y: i32, w: u32, h: u32, mag: u32) -> DynamicImage {
    //println!("x:{}, y:{}, w:{}, h:{}", x, y, w, h);
//    let img = scr.capture_area(x, y, w, h).unwrap();
//    let img = image::load_from_memory(img.buffer());
//    let img = img.as_ref().unwrap();
//    let resize = img.resize(img.width()*mag, img.height()*mag, FilterType::Lanczos3);
    let mut imgtemp = image.clone();
    let resize = imgtemp.crop(x as u32, y as u32, w, h);
    return resize;
}

fn get_text(path: String) -> String {
//    let cd = env::current_dir().unwrap();
//    let p = cd.to_string_lossy().to_string() + &path;
//    let str = ocr::get_ocr_text(&path.as_str());
    let output = Command::new("bin/ocr.exe").args(&[format!("{}", path)]).output().expect("failed");
    let str = String::from_utf8_lossy(&output.stdout);
    let v:Vec<&str> = str.split_whitespace().collect();
    let r = v.join("").replace("音成", "育成");

    return r;
}

fn get_text2num(path: String) -> String {
    let r = get_text(path);
    let v: Vec<i32> = r.split("").filter_map(|k| k.parse().ok()).collect::<Vec<i32>>();
    let mut ret:String = String::new();

    for c in v {
        ret += &c.to_string();
    }

    return ret;
}

fn get_text_tesseract(cmd: String, temppath: String) -> String {
    Command::new("bin/Tesseract-OCR/tesseract.exe").args(&[format!("{}", cmd), temppath.clone() + "ret", "-l eng".to_string()]).output().expect("failed");
//    String::from_utf8_lossy(&output.stdout);
    let str = fs::read_to_string(temppath + "ret.txt");
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

fn get_text_tesseract_jp(cmd: String, temppath: String) -> String {
    Command::new("bin/Tesseract-OCR/tesseract.exe").args(&[format!("{}", cmd), temppath.clone() + "ret", "-l jpn".to_string()]).output().expect("failed");
    let str = fs::read_to_string(temppath + "ret.txt");
    let temp = str.unwrap();
    let v:Vec<&str> = temp.split_whitespace().collect();
    let r = v.join("");

    return r;
}

fn get_text_tesseract_v(cmd: String, temppath: String) -> Vec<String> {
    Command::new("bin/Tesseract-OCR/tesseract.exe").args(&[format!("{}", cmd), temppath.clone() + "ret", "-l eng".to_string()]).output().expect("failed");
//    String::from_utf8_lossy(&output.stdout);
    let str = fs::read_to_string(temppath + "ret.txt").unwrap();
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
    if cnt == 2 && first_str != "5" && first_str != "6" && first_str != "7" && first_str != "8" && first_str != "9" { return false; }

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

fn save_file(path: String, output: String) {
    let mut event_txt = fs::File::create(path).unwrap();
    event_txt.write_all(output.as_bytes()).unwrap();
}

fn get_rgb(path: String) -> Rgb<u8> {
    let img = image::open(path).unwrap().to_rgb8();
    return *img.get_pixel(5, 5);
}

fn get_condition(path: String) -> String {
    let rgb = get_rgb(path); // GRB to RGB
    let zekkocho: Rgb<u8> = image::Rgb([158, 127, 245]); //絶好調
    let kocho: Rgb<u8> = image::Rgb([65, 170, 245]); // 好調
    let futuu: Rgb<u8> = image::Rgb([24, 214, 245]); // 普通
    let fucho: Rgb<u8> = image::Rgb([245, 171, 15]); // 不調
    let zeffucho: Rgb<u8> = image::Rgb([245, 128, 200]); // 絶不調
    let div: u8 = 10;
    let mut res = String::from("");
    //println!("RGB: {:?}", rgb);
    //println!("{:?}", zekkocho[0] - div);

    if rgb[0] >= (zekkocho[0]-div) && rgb[0] <= (zekkocho[0]+div) && rgb[1] >= (zekkocho[1]-div) && rgb[1] <= (zekkocho[1]+div) && rgb[2] >= (zekkocho[2]-div) && rgb[2] <= (zekkocho[2]+div) {
        res = "絶好調".to_string();
    } else if rgb[0] >= (kocho[0]-div) && rgb[0] <= (kocho[0]+div) && rgb[1] >= (kocho[1]-div) && rgb[1] <= (kocho[1]+div) && rgb[2] >= (kocho[2]-div) && rgb[2] <= (kocho[2]+div) {
        res = "好調".to_string();
    } else if rgb[0] >= (futuu[0]-div) && rgb[0] <= (futuu[0]+div) && rgb[1] >= (futuu[1]-div) && rgb[1] <= (futuu[1]+div) && rgb[2] >= (futuu[2]-div) && rgb[2] <= (futuu[2]+div) {
        res = "普通".to_string();
    } else if rgb[0] >= (fucho[0]-div) && rgb[0] <= (fucho[0]+div) && rgb[1] >= (fucho[1]-div) && rgb[1] <= (fucho[1]+div) && rgb[2] >= (fucho[2]-div) && rgb[2] <= (fucho[2]+div) {
        res = "不調".to_string();
    } else if rgb[0] >= (zeffucho[0]-div) && rgb[0] <= (zeffucho[0]+div) && rgb[1] >= (zeffucho[1]-div) && rgb[1] <= (zeffucho[1]+div) && rgb[2] >= (zeffucho[2]-div) && rgb[2] <= (zeffucho[2]+div) {
        res = "絶不調".to_string();
    }

    return res;
}