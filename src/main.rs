use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);
    if args.len() != 3 && args.len() != 2
    {
        println!("Usage main.exe input.tex output.tex.");
        return;
    }
    // let path = "input.tex";
    match fs::read_to_string(&args[1]) {
        Ok(mut content) => {
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
                            if bytes[start] == 123u8 // {
                            {
                                stack.push('{');
                            }
                            if bytes[start] == 125u8 //}
                            {
                                stack.pop();
                            }
                        }
                        content.remove(start); // remove }
                    }
                }
            }
            // println!("{content}");
            if args.len() == 3
            {
                fs::write(&args[2], content).expect("write file error!!!")
            } else {
                println!("{content}")
            }
        }
        Err(_) => {
            println!("Can't open the input file named {}.", &args[1])
        }
    };
}
