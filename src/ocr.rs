winrt::import!(
    dependencies
        os
    types
        windows::storage::*
        windows::graphics::imaging::*
        windows::media::ocr::*
);

use self::windows::globalization::Language;
use self::windows::storage::{StorageFile, FileAccessMode};
use self::windows::graphics::imaging::{BitmapDecoder, SoftwareBitmap};
use self::windows::media::ocr::OcrEngine;

const LANG: &'static str = "ja";

fn open_image(path: &str) -> winrt::Result<SoftwareBitmap> {
    let file = StorageFile::get_file_from_path_async(path)?.get()?;

    let bitmap = BitmapDecoder::create_with_id_async(
        BitmapDecoder::png_decoder_id()?,
        file.open_async(FileAccessMode::Read)?.get()?
    )?.get()?;

    bitmap.get_software_bitmap_async()?.get()
}

fn ocr(bitmap: SoftwareBitmap) -> winrt::Result<winrt::HString> {
    let lang = Language::create_language(LANG)?;
    let engine = OcrEngine::try_create_from_language(lang)?;
    let result = engine.recognize_async(bitmap)?.get()?;
    return Ok(result.text()?);
}

fn get_ocr(path: &str) -> winrt::Result<winrt::HString> {
    let bitmap = open_image(path)?;
    let r = ocr(bitmap);

    return r;
}

pub fn get_ocr_text(path: &str) -> String {
    let ocr = get_ocr(path);
    let text = format!("{:?}", ocr);
    
    return text.replace("Ok(", "").replace(")", "").trim().to_owned();
}