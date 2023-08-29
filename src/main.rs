#![windows_subsystem = "windows"]

use fltk::button::Button;
use fltk::{prelude::*, *};
use regex::Regex;


fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = window::Window::default()
        .with_size(1000, 600)
        .with_label("Latex File Remover");
    wind.make_resizable(true);
    let buf1 = text::TextBuffer::default();
    let mut editor = text::TextEditor::default()
        .with_size(wind.width() / 2 - 30, wind.height())
        .with_pos(0, 0);

    editor.set_buffer(buf1);
    editor.wrap_mode(text::WrapMode::AtBounds, 0);

    let mut txt = text::TextDisplay::default()
        .with_size(wind.width() / 2 - 30, wind.height())
        .with_pos(wind.width() / 2 + 30, 0);

    let buf2 = text::TextBuffer::default();
    buf2.line_start(0);
    txt.set_buffer(buf2);
    txt.wrap_mode(text::WrapMode::AtBounds, 0);
    let editor1 = editor.clone();
    let editor2 = editor.clone();
    let editor3 = editor.clone();
    let editor4 = editor.clone();
    let txt1 = txt.clone();
    let txt2 = txt.clone();
    let txt3 = txt.clone();
    let txt4 = txt.clone();

    let mut but = Button::new(
        wind.width() / 2 - 30,
        wind.height() / 2 - 200,
        60,
        40,
        "右边复制\n到左边",
    );
    but.set_callback(move |_| {
        let source = txt3.buffer().unwrap().text();
        editor3
            .buffer()
            .unwrap()
            .remove(0, editor3.buffer().unwrap().length());

        editor3.buffer().unwrap().append(&source);
    });

    let mut but1 = Button::new(wind.width() / 2 - 30, wind.height() / 2, 60, 40, "删除高亮");

    but1.set_callback(move |_| {
        // println!("{}", editor.buffer().unwrap().text());
        let mut source = editor1.buffer().unwrap().text();
        txt1.buffer()
            .unwrap()
            .remove(0, txt1.buffer().unwrap().length());

        txt1.buffer().unwrap().append(highlight_remove(&mut source));
    });

    let mut but2 = Button::new(
        wind.width() / 2 - 30,
        wind.height() / 2 - 100,
        60,
        40,
        "删除中文",
    );
    but2.set_callback(move |_| {
        let mut source = editor2.buffer().unwrap().text();
        txt2.buffer()
            .unwrap()
            .remove(0, txt2.buffer().unwrap().length());

        txt2.buffer().unwrap().append(&chinese_remove(&mut source));
    });

    let mut but3 = Button::new(
        wind.width() / 2 - 30,
        wind.height() / 2 + 100,
        60,
        40,
        "删除注释",
    );
    but3.set_callback(move |_| {
        let mut source = editor4.buffer().unwrap().text();
        txt4.buffer()
            .unwrap()
            .remove(0, txt4.buffer().unwrap().length());

        txt4.buffer()
            .unwrap()
            // .append(&remove_comment(&mut chinese_remove(&mut source)));
            .append(&remove_comment(&mut source));
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}

fn remove_comment(content: &mut String) -> &str {
    loop {
        match content.find("%") {
            None => {
                break;
            }
            Some(start) => {
                let bytes = content.as_bytes();
                if start != 0 && bytes[start - 1] == 47u8
                //\%
                {
                    // println!("{start}");
                    continue;
                }
                let mut end = start;
                while end < content.len() && bytes[end] != 10u8
                //newline
                {
                    end += 1;
                }
                content.replace_range(start..end, "");
            }
        }
    }
    content
}

fn chinese_remove(content: &str) -> String {
    let re = Regex::new(r"[\u4e00-\u9fa5]").unwrap();
    let after = re.replace_all(content, "").to_string();

    after
}

fn highlight_remove(content: &mut String) -> &str {
    loop {
        match content.find("\\hl{") {
            None => {
                break;
            }
            Some(mut start) => {
                content.remove(start); // remove \
                content.remove(start); // remove h
                content.remove(start); // remove l
                content.remove(start); // remove {
                                       // println!("{start}");
                let mut stack = vec![];
                stack.push('{');
                let bytes = content.as_bytes();
                while !stack.is_empty() {
                    start += 1;
                    if bytes[start] == 123u8
                    // {
                    {
                        stack.push('{');
                    }
                    if bytes[start] == 125u8
                    //}
                    {
                        stack.pop();
                    }
                }
                content.remove(start); // remove }
            }
        }
    }
    // println!("{content}");
    content
}
