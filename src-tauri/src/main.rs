#![cfg_attr(
  all(not(debug_assertions), target_os = "linux"),
  // windows_subsystem = "windows"
)]
extern crate repng;
extern crate scrap;
extern crate image;

use scrap::{Capturer, Display};
use std::{io::{ErrorKind::WouldBlock, Cursor}};
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use image::{io::Reader, imageops::crop};
use tauri::Window;
use reqwest::Client;
use anyhow::{Result, format_err};
use base64;
use serde::Deserialize;

async fn take_screenshot(window: Window) -> Result<String> {
  println!("screenshot!");

  let top = 100;
  let left = 100;
  let width = 200;
  let height = 200;

  let one_second = Duration::new(1, 0);
  let one_frame = one_second / 60;

  let display = Display::primary().expect("Couldn't find primary display.");
  let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
  let (w, h) = (capturer.width(), capturer.height());

  let window_position = window.inner_position().unwrap();
  let window_size = window.inner_size().unwrap();
  println!("window top: {:?}", window.inner_position().unwrap());
  println!("window size: {:?}", window.inner_size().unwrap());

  loop {
      // Wait until there's a frame.

      let buffer = match capturer.frame() {
          Ok(buffer) => buffer,
          Err(error) => {
              if error.kind() == WouldBlock {
                  // Keep spinning.
                  thread::sleep(one_frame);
                  continue;
              } else {
                  panic!("Error: {}", error);
              }
          }
      };

      println!("Captured! Saving...");

      // Flip the ARGB image into a BGRA image.

      let mut bitflipped = Vec::with_capacity(width * height * 4);
      let stride = buffer.len() / h;

      let top = window_position.y as usize;
      let left = window_position.x as usize;
      let width = window_size.width as usize;
      let height = window_size.height as usize;

      for y in top..(top + height) {
        for x in left..(left + width) {
            let i = stride * y + 4 * x;
            bitflipped.extend_from_slice(&[
                buffer[i + 2],
                buffer[i + 1],
                buffer[i],
                255,
            ]);
        }
      } 

      // Save the image.
      let mut buf_png = vec![];

      repng::encode(
          // File::create("screenshot.png").unwrap(),
          &mut buf_png,
          width as u32,
          height as u32,
          &bitflipped,
      ).unwrap();

      return Ok(base64::encode(buf_png));
  }
}


#[derive(Deserialize)]
struct UploadResult {
  url: String,
  id: String,
}

#[derive(Deserialize)]
struct UploadResponse {
  code: u32,
  result: UploadResult,
}

async fn upload_screenshot(base64: &str) -> Result<String> {
  let mut map = HashMap::new();
  map.insert("src_url", "google.com");
  map.insert("image_type", "png");
  map.insert("image_content", base64);
  
  let client = Client::new();
  let res = client.post("http://localhost:3001/upload")
    .json(&map)
    .send()
    .await?
    .json::<UploadResponse>()
    .await?;

  // Ok(res.result.url)
  Ok("http://snap/123".to_string())
}

#[tauri::command]
async fn handle_screenshot_capture(window: Window) -> std::result::Result<(), String> {
  let base64_image = take_screenshot(window).await;
  
  if base64_image.is_err() {
    println!("error {:?}", base64_image.err());
    return Err("Cannot take screenshot".to_string());
  }
  println!("image {}", &base64_image.unwrap());
  // let url = upload_screenshot(&base64_image.unwrap()).await;
  let url = upload_screenshot("").await;

  if url.is_err() {
    return Err("cannot post the screenshot".to_string());
  }

  Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![handle_screenshot_capture])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
