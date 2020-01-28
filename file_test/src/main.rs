use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;

fn main() -> Result<(), std::io::Error> {
    let file = OpenOptions::new().read(true).write(true).create(true).open("foo.txt")?;
    let file = write_toto(file)?;
    let mut file = write_titi(file)?;
    let mut buffer = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

fn write_toto(mut f: File) -> Result<File, std::io::Error> {
    f.write(b"toto\n")?;
    Ok(f)
}

fn write_titi(mut f: File) -> Result<File, std::io::Error> {
    f.write(b"titi\n")?;
    Ok(f)
}
