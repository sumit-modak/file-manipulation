#!/usr/bin/env -S cargo +nightly -Zscript

//! Used for formatting kitty themes

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let dirpath = PathBuf::from("/home/sumit/dev/main/nixos-config/config/kitty/themes");
    let walker = std::fs::read_dir(dirpath).unwrap().into_iter();
    let _ = std::fs::create_dir("./themes/");
    for entry in walker {
        let mut m = HashMap::<&str, &str>::new();
        let filepath = PathBuf::from(entry.unwrap().path());
        let f = std::fs::read_to_string(&filepath).unwrap();
        for line in f.lines() {
            let mut it = line.split_whitespace();
            if let Some(first_entry) = it.next() {
                if first_entry.starts_with("#")
                    || first_entry == "background"
                    || first_entry == "foreground"
                {
                    continue;
                }
                m.entry(first_entry).or_insert(it.next().unwrap());
            }
        }
        // serialize to new file
        let mut newf = File::create(format!(
            "./themes/{}",
            filepath.file_name().unwrap().to_str().unwrap()
        ))
        .unwrap();
        let mut buf = String::new();
        buf.push_str(&format!(
            "cursor                  {}\n",
            m.get("cursor").unwrap_or(&"none")
        ));
        if let Some(cursor_text_color) = m.get("cursor_text_color") {
            buf.push_str(&format!("cursor_text_color       {}\n", cursor_text_color));
        } else {
            buf.push_str("\n");
        }
        buf.push_str(&format!(
            "selection_background    {}\n",
            m.get("selection_background").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color0                  {}\n",
            m.get("color0").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color8                  {}\n",
            m.get("color8").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color1                  {}\n",
            m.get("color1").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color9                  {}\n",
            m.get("color9").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color2                  {}\n",
            m.get("color2").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color10                 {}\n",
            m.get("color10").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color3                  {}\n",
            m.get("color3").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color11                 {}\n",
            m.get("color11").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color4                  {}\n",
            m.get("color4").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color12                 {}\n",
            m.get("color12").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color5                  {}\n",
            m.get("color5").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color13                 {}\n",
            m.get("color13").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color6                  {}\n",
            m.get("color6").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color14                 {}\n",
            m.get("color14").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color7                  {}\n",
            m.get("color7").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "color15                 {}\n",
            m.get("color15").unwrap_or(&"none")
        ));
        buf.push_str(&format!(
            "selection_foreground    {}\n",
            m.get("selection_foreground").unwrap_or(&"none")
        ));
        newf.write(&buf.into_bytes()).unwrap();
    }
}
