use std::env::args_os;
use std::env::current_dir;
use std::ffi::OsString;
use std::path::Path;
use std::fs::read_dir;

fn main() -> Result<(), std::io::Error>{
    if args_os().count() > 1 {
        for argument in args_os().skip(1) {
            handle_path_recursive(argument, 0)?;
        }
    } else {
        handle_path_recursive(current_dir().unwrap().into_os_string(), 0)?;
    }
    Ok(())
}

fn handle_path_recursive(path_os_string: OsString, depth: usize) -> Result<(), std::io::Error> {
    let path = Path::new(&path_os_string);
    if path.exists() {
        if path.is_dir() {
            print_path(path, depth, true);
            for entry in read_dir(path)? {
                let path = entry?.path();
                handle_path_recursive(path.into_os_string(), depth +1)?;
            }
        } else {
            print_path(path, depth, false);
        }
    } else {
        println!("\nInvalid path: {:?}\n", path);
    }
    Ok(())
}

fn print_path(path: &Path, depth: usize, is_dir: bool){
    match path.file_name(){
        Some(path) => {
            let mut f = path.to_string_lossy();
            if &f[0..1] != "." {
                f = if !is_dir { f } else { f + "/" };
                if depth == 0 { println!("") } else { print!("|") };
                println!("{:_<1$}{2}", "", depth*2, f);
            }
        },
        None => return,
    }
}
