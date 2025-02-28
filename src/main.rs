mod flash;
use flash::SST39F010A;
const ADDRPINS: [u8; 16] = [12, 1, 7, 8, 25, 24, 23, 18, 27, 22, 11, 10, 15, 17, 4, 14];
const DATAPINS: [u8; 8] = [16, 20, 21, 26, 19, 13, 6, 5];
const CEN_PIN: u8 = 0;
const OEN_PIN: u8 = 9;
const WEN_PIN: u8 = 3;

use std::io::{BufReader, Read};
use std::process::ExitCode;
use std::env;
use std::fs::File;

fn main() -> ExitCode {
    let mut flash = SST39F010A::new(
        Vec::from(ADDRPINS), Vec::from(DATAPINS), CEN_PIN, WEN_PIN, OEN_PIN
    );
    let mut data;
    let args: Vec<String> = env::args().collect();
    let validpath;
    let mut file_lenth_conut: usize = 0;
    // 参数处理，留下有效的文件路径参数
    {
        let filepath = args.get(1);
        match filepath {
            Some(path) => validpath = path.clone(),
            None => {
                println!("filepath needed!");
                return ExitCode::FAILURE;
            },
        }
    }

    // 读取文件路径，获得文件对象
    let writefile = File::open(validpath);
    let file;
    match writefile {
        Ok(handle) => file = handle,
        Err(e) => {println!("{e:#?}"); return ExitCode::FAILURE},
    }

    flash.erase_flash();
    println!("Flash erased...");

    let reader = BufReader::new(file);
    for (index,bytedata) in reader.bytes().enumerate() {
        if index % 16 == 0 {
            print!("0x{index:04X}: ");
        }
        match bytedata {
            Ok(d) => {
                print!("{d:02x} ");
                flash.write_byte(index as u16, d);
            },
            Err(e) => println!("{index:04x}: {e:#?}"),
        }
        if index % 16 == 15 {
            println!();
        }
        file_lenth_conut = index;
    }
    println!();

    println!("reading back...");
    for addr in 0x00..=file_lenth_conut as u16 {
        data = flash.read_at(addr);
        if (addr % 16) == 0 {
            print!("0x{:04X}: ", addr);
        }
        print!("{:02x} ", data);
        if (addr % 16) == 15 {
            println!();
        }
    }
    println!();

    ExitCode::SUCCESS
}
