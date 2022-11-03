use std::ffi::OsString;
use windows::{
    Win32::Foundation::{HWND, PWSTR, RECT},
    Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowRect},
};
use std::fs;
use screenshots::Screen;
use chrono::Local;

pub fn screenshot() {
    let window_name = OsString::from("umamusume");
    let id: HWND = unsafe { FindWindowW(PWSTR::default(), window_name) };
    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    let filename = &format!("{}", Local::now().format("%Y%m%d%H%M%S"));
 
    if unsafe { GetWindowRect(id, &mut rect)} != false {
        let left:i32 = rect.left;
        let top:i32 = rect.top;
        let right:i32 = rect.right;
        let bottom:i32 = rect.bottom;
        let width:i32 = right-left;
        let height:i32 = bottom-top;
        let scr = Screen::from_point(left, top).unwrap();
        let image = scr.capture_area(left, top, width as u32, (height) as u32).unwrap();
        let buffer = image.buffer();
        fs::write(&format!("./screenshot/{}.png", filename), &buffer).unwrap();
    }
}