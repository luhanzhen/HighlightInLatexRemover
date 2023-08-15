#![windows_subsystem = "windows"]

use fltk::button::Button;
use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = window::Window::default()
        .with_size(1000, 600)
        .with_label("Highlight(\\hl{}) In Latex Remover");
    wind.make_resizable(true);
    let buf1 = text::TextBuffer::default();
    let mut editor = text::TextEditor::default()
        .with_size(wind.width() / 2 - 20, wind.height())
        .with_pos(0, 0);

    editor.set_buffer(buf1);
    editor.wrap_mode(text::WrapMode::AtBounds, 0);

    let mut txt = text::TextDisplay::default()
        .with_size(wind.width() / 2 - 20, wind.height())
        .with_pos(wind.width() / 2 + 20, 0);

    let buf2 = text::TextBuffer::default();
    buf2.line_start(0);
    txt.set_buffer(buf2);
    txt.wrap_mode(text::WrapMode::AtBounds, 0);

    let mut but = Button::new(wind.width() / 2 - 20, wind.height() / 2, 40, 40, "转换");
    but.set_callback(move |_| {
        // println!("{}", editor.buffer().unwrap().text());
        let mut source = editor.buffer().unwrap().text();
        txt.buffer()
            .unwrap()
            .remove(0, txt.buffer().unwrap().length());

        txt.buffer().unwrap().append(transform(&mut source));
    });
    wind.end();
    wind.show();
    app.run().unwrap();
}

fn transform(content: &mut String) -> &str {
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
