mod fmt_duration;

use fmt_duration::{MyDuration, DelayedFormat};
use std::{panic, process};
use std::io::{self, BufRead, Write};
use std::time::Instant;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use clap::{Arg, ArgAction, Command};


fn verify_formatstr(formatstr: &str) {
    // All available specifiers: https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers
    // As chrono issue https://github.com/chronotope/chrono/issues/956,
    // invalid-format-strings cause panic, so we have to verify it before real-used
    panic::set_hook(Box::new(|_| ()));
    let r = panic::catch_unwind(|| {
        chrono::Local::now().format(formatstr).to_string();
    });
    let _ = panic::take_hook();
    if r.is_err() {
        eprintln!("Error: can not format timestamp with `{formatstr}`\nTry 'tstee --help' for more information.");
        process::exit(1);
    }
}


fn tstee(mut files: Vec<Box<dyn Write>>, formatstr: &str, relative_flag: bool, incre_flag: bool, utc_flag: bool) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    files.push(Box::new(stdout));
    let mut line: Vec<u8> = Vec::new();
    match (relative_flag, incre_flag) {
        (true, false) => {
            let delayformat = DelayedFormat::new(formatstr.to_string());
            let start_time = Instant::now();
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            loop {
                match stdin.read_until(b'\n', &mut line) {
                    Ok(n_bytes) => {
                        if n_bytes == 0 { break; }
                        for f in &mut files {
                            let m = MyDuration::new(start_time.elapsed(), &delayformat);
                            write!(f, "{} ", m)?;
                            f.write_all(&line)?;
                        }
                        line.clear();
                    }
                    Err(e) => {
                        eprintln!("Error when read stdin: {e}");
                        process::exit(1);
                    }
                }
            }
        }
        (false, true) => {
            let delayformat = DelayedFormat::new(formatstr.to_string());
            let mut start_time = Instant::now();
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            loop {
                match stdin.read_until(b'\n', &mut line) {
                    Ok(n_bytes) => {
                        if n_bytes == 0 { break; }
                        for f in &mut files {
                            let m = MyDuration::new(start_time.elapsed(), &delayformat);
                            write!(f, "{} ", m)?;
                            f.write_all(&line)?;
                        }
                        start_time = Instant::now();
                        line.clear();
                    }
                    Err(e) => {
                        eprintln!("Error when read stdin: {e}");
                        process::exit(1);
                    }
                }
            }
        }
        (false, false) => {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            loop {
                match stdin.read_until(b'\n', &mut line) {
                    Ok(n_bytes) => {
                        if n_bytes == 0 { break; }
                        let ts = if utc_flag {
                            chrono::Utc::now().format(formatstr)
                        } else {
                            chrono::Local::now().format(formatstr)
                        };
                        for f in &mut files {
                            write!(f, "{} ", ts)?;
                            f.write_all(&line)?;
                        }
                        line.clear();
                    }
                    Err(e) => {
                        eprintln!("Error when read stdin: {e}");
                        process::exit(1);
                    }
                }
            }
        }
        _ => ()
    };
    Ok(())
}


fn main() {
    let cli = Command::new("tstee")
        .version("0.5")
        .about("Hybrid of moreutils/ts and coreutils/tee\nread from standard input, add a timestamp to the beginning of each line, write to standard output and files")
        .arg(Arg::new("FILE")
            .action(ArgAction::Append)
            .help("Copy standard input to each FILE, and also to standard output.")
        )
        .arg(Arg::new("file")
            .short('a')
            .long("append")
            .action(clap::ArgAction::Append)
            .help("append to the given FILEs, do not overwrite")
        )
        .arg(Arg::new("formatstr")
            .short('f')
            .long("format")
            .action(ArgAction::Set)
            .long_help(
                r#"this parameter controls how the timestamp is formatted, default format "%Y-%m-%d %H:%M:%S%.3f".
  most of common timestamp formats are supported.
  if the -r/-i switch is passed, only support %H %h %M %m %S %s %.f %.Nf:
  for example, time elapsed is 94028.602718334 seconds
      %s    =>  94028       Number of seconds
      %S    =>  08          Second number (00–59), zero-padded to 2 digits.
      %m    =>  1567        Number of minutes
      %M    =>  07          Minute number (00–59), zero-padded to 2 digits.
      %h    =>  26          Number of hours
      %H    =>  02          Hour number (00–23), zero-padded to 2 digits.
      %.f   =>  .6          Decimal fraction of a second with a fixed length of 1
      %.1f  =>  .6          Decimal fraction of a second with a fixed length of 1
      %.2f  =>  .60         Decimal fraction of a second with a fixed length of 2
      %.6f  =>  .602718     Decimal fraction of a second with a fixed length of 6
      %.9f  =>  .602718334  Decimal fraction of a second with a fixed length of 9
      "%Hh:%Mm:%S%.3fs" => "02h:07m:08.602s"
      "total %h hours ,or %m minutes, or %s seconds" => "total 26 hours ,or 1567 minutes, or 94028 seconds""#)
        )
        .arg(Arg::new("relative")
            .short('r')
            .long("relative")
            .action(clap::ArgAction::SetTrue)
            .help(r#"use the time elapsed since start of the program. default format "%H:%M:%S%.3f""#)
        )
        .arg(Arg::new("incremental")
            .short('i')
            .long("incremental")
            .action(clap::ArgAction::SetTrue)
            .help(r#"use the time elapsed since the last timestamp. default format "%H:%M:%S%.3f""#)
        )
        .arg(Arg::new("utc")
            .short('u')
            .long("utc")
            .action(clap::ArgAction::SetTrue)
            .help("use UTC+00:00, NOT the current timezone of the OS. if the -r/-i switch is passed, this flag will not take effect")
        )
        .after_help("Examples: ping www.google.com | tstee ping.log")
        .get_matches();

    let incre_flag = *cli.get_one::<bool>("incremental").unwrap();
    let relative_flag = *cli.get_one::<bool>("relative").unwrap();
    let utc_flag = *cli.get_one::<bool>("utc").unwrap();
    let arg_files = cli
        .get_many::<String>("FILE")
        .unwrap_or_default()
        .collect::<Vec<_>>();
    let arg_append_files = cli
        .get_many::<String>("file")
        .unwrap_or_default()
        .collect::<Vec<_>>();
    if incre_flag && relative_flag {
        eprintln!("Error: -r -i switch can not passed at the same time");
        process::exit(1);
    }
    let formatstr = match cli.get_one::<String>("formatstr") {
        None => {
            if relative_flag || incre_flag {
                "%H:%M:%S%.3f"
            } else {
                "%Y-%m-%d %H:%M:%S%.3f"
            }
        }
        Some(s) => s,
    };
    if !relative_flag && !incre_flag {
        verify_formatstr(formatstr);
    }
    let mut files: Vec<Box<dyn Write>> = Vec::new();
    for x in arg_files.iter() {
        let p = Path::new(x);
        if x.ends_with('/') || x.ends_with('\\') || p.is_dir() {
            eprintln!("Error: `{x}` Is a directory");
            process::exit(1);
        }
        match File::create(p) {
            Ok(f) => files.push(Box::new(f)),
            Err(e) => {
                eprintln!("Error: fail to open `{x}`: {e}");
                process::exit(1);
            }
        }
    }
    for x in arg_append_files.iter() {
        let p = Path::new(x);
        if x.ends_with('/') || x.ends_with('\\') || p.is_dir() {
            eprintln!("Error: `{x}` Is a directory");
            process::exit(1);
        }
        if p.exists() {
            match File::options().append(true).open(p) {
                Ok(f) => files.push(Box::new(f)),
                Err(e) => {
                    eprintln!("Error: fail to open `{x}`: {e}");
                    process::exit(1);
                }
            }
        } else {
            match File::create(p) {
                Ok(f) => files.push(Box::new(f)),
                Err(e) => {
                    eprintln!("Error: fail to open `{x}`: {e}");
                    process::exit(1);
                }
            }
        }
    }


    let _ = tstee(files, formatstr, relative_flag, incre_flag, utc_flag);
}

