use std::time::Duration;
use std::fmt::{self, Display};
use regex::Regex;


pub struct DelayedFormat {
    spstr: Vec<String>,
    delimiter: Vec<String>,
    len: usize,
}

impl DelayedFormat {
    pub fn new(format_str: String) -> DelayedFormat {
        // support %H %h %M %m %S %s %.f %.Nf,  Case insensitive
        let re = Regex::new(r"(%[HhMmSs])|(%\.[1-9]?f)").unwrap();
        let spstr: Vec<String> = re.split(&format_str).map(|x| x.to_string()).collect();
        let delimiter: Vec<String> = re.find_iter(&format_str).map(|m| m.as_str().to_string()).collect();
        let len = delimiter.len();
        DelayedFormat { spstr, delimiter, len }
    }
}

pub struct MyDuration<'a> {
    secs: u64,
    nanos: u64,
    display: &'a DelayedFormat,
}

impl MyDuration<'_> {
    pub fn new(duration: Duration, display: &DelayedFormat) -> MyDuration {
        MyDuration {
            secs: duration.as_secs(),
            nanos: duration.subsec_nanos() as u64,
            display,
        }
    }
    const fn s_sub(&self) -> u64 {
        self.secs % 60
    }
    const fn m(&self) -> u64 {
        self.secs / 60
    }
    const fn m_sub(&self) -> u64 {
        self.secs % 3600 / 60
    }
    const fn h(&self) -> u64 {
        self.secs / 3600
    }
    const fn h_sub(&self) -> u64 {
        (self.secs / 3600) % 24
    }
}

impl Display for MyDuration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.display.len;
        let spstr: Vec<_> = self.display.spstr.iter().map(|x| x.as_str()).collect();
        let delimiter: Vec<_> = self.display.delimiter.iter().map(|x| x.as_str()).collect();
        for i in 0..len {
            let (x, y) = (&spstr[i], &delimiter[i]);
            write!(f, "{}", x)?;
            let t = match *y {
                "%h" => { format!("{}", self.h()) }
                "%H" => { format!("{:0>2}", self.h_sub()) }
                "%m" => { format!("{}", self.m()) }
                "%M" => { format!("{:0>2}", self.m_sub()) }
                "%s" => { format!("{}", self.secs) }
                "%S" => { format!("{:0>2}", self.s_sub()) }
                "%.f" => { format!(".{}", self.nanos / 100_000_000) }
                "%.1f" => { format!(".{}", self.nanos / 100_000_000) }
                "%.2f" => { format!(".{:0>2}", self.nanos / 10_000_000) }
                "%.3f" => { format!(".{:0>3}", self.nanos / 1_000_000) }
                "%.4f" => { format!(".{:0>4}", self.nanos / 100_000) }
                "%.5f" => { format!(".{:0>5}", self.nanos / 10_000) }
                "%.6f" => { format!(".{:0>6}", self.nanos / 1_000) }
                "%.7f" => { format!(".{:0>7}", self.nanos / 100) }
                "%.8f" => { format!(".{:0>8}", self.nanos / 10) }
                "%.9f" => { format!(".{:0>9}", self.nanos) }
                _ => "Error, need check regex".to_string(),
            };
            write!(f, "{}", t)?;
        }
        write!(f, "{}", spstr[len])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_format(formatstr: &str, secs: u64, nanos: u32) {
        let time = Duration::new(secs, nanos);
        let delayformat = DelayedFormat::new(formatstr.to_string());
        let m = MyDuration::new(time, &delayformat);
        println!(r#"{secs:>6}.{nanos:0>9} {formatstr:?} => "{m}""#);
    }


    #[test]
    fn test_format1() {
        // cargo test -- --nocapture
        test_format("%s%.9f", 94028, 602718334);
        test_format("%H:%M:%S%.3f", 94028, 602718334);
        test_format("%Hh:%Mm:%S%.3fs", 94028, 602718334);
        test_format("total %h hour ,or %m minutes, or %s seconds", 94028, 602718334);
        test_format("%.f %.1f %.2f %.3f", 94028, 602718334);
        test_format("%.4f %.5f %.6f", 94028, 602718334);
        test_format("%.7f %.8f %.9f", 94028, 602718334);

        test_format("%s%.9f", 4028, 183340);
        test_format("%H:%M:%S%.3f", 4028, 183340);
        test_format("%Hh:%Mm:%S%.3fs", 4028, 183340);
        test_format("total %h hour ,or %m minutes, or %s seconds", 4028, 183340);
        test_format("%.f %.1f %.2f %.3f", 4028, 183340);
        test_format("%.4f %.5f %.6f", 4028, 183340);
        test_format("%.7f %.8f %.9f", 4028, 183340);
    }
}

