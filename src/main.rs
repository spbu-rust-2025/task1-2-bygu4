use std::io;

fn add(opt: Option<i32>, num: i32) -> Option<i32> {
    match opt {
        Some(val) => return Some(val + num),
        None => return None,
    }
}

fn main() {
    let mut buf = String::new();
    let mut acc = Some(0);

    loop {
        match io::stdin().read_line(&mut buf) {
            Ok(_) => match buf.as_str().trim().parse::<i32>() {
                Ok(-1) => break,
                Ok(num) if num >= 0 => acc = add(acc, num),
                _ => acc = None,
            },
            _ => acc = None,
        }

        buf.clear();
    }

    match acc {
        Some(val) => println!("{}", val),
        None => println!("NaN"),
    }
}
