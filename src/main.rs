extern crate regex;
extern crate getopts;
#[macro_use] extern crate lazy_static;

use std::env::args;
use std::fmt;
use std::io;
use std::io::Write;
use std::process;

use regex::Regex;
use getopts::Options;

/// A roll consists of:
/// - A number of dice (positive integer)
/// - A number of faces (positive integer)
/// - An extra (e.g., +3 or -4)
struct Roll {
    num_dice: f32,
    num_faces: f32,
    extra: f32,
}

impl Roll {
    /// Compute the expected value: expected value of one die
    /// multiplied by the number of dice, then add the extra.
    fn ev(&self) -> f32 {
        // Math reminder:
        // \sum_{i=1}^{n} = n(n+1) / 2
        // therefore
        // 1/n * \sum_{i=1}^{n} = (n+1) / 2
        let single_die_ev = (self.num_faces + 1.0) / 2.0;
        self.num_dice * single_die_ev + self.extra
    }

    /// Compute the minimum value.
    fn min(&self) -> f32 {
        self.num_dice + self.extra
    }

    /// Compute the maximum value.
    fn max(&self) -> f32 {
        self.num_dice * self.num_faces + self.extra
    }

    fn print(&self) -> String {
        format!("{} {} {} {}", self, self.min(), self.max(), self.ev())
    }

    fn pretty_print(&self) -> String {
        format!("{}:\n\tmin: {}\n\tmax: {}\n\tev : {}",
                self, self.min(), self.max(), self.ev())
    }
}

/// Convert a roll into the 'XdY+Z' notation.
impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{}d{}", self.num_dice, self.num_faces);
        if self.extra != 0.0 {
            write!(f, "{:+}", self.extra)
        } else {
            Ok(())
        }
    }
}

/// Display a message on stderr
fn errmsg(msg: &str) {
    let mut stderr = io::stderr();
    let _ = writeln!(stderr, "ev: {}", msg);
}

/// Show basic usage of the program
fn usage(opts: &Options, progname: &str) {
    let brief = format!("Usage: {} [options] [rolls ...]\n\t roll: XdY, XdY+Z, XdY-Z (e.g. 1d6, 2d4+1, 3d8-1)",
                        progname);
    print!("{}", opts.usage(&brief));
}

fn parse_and_print(line: &str, single_line: bool) {
    /*
    GRAMMAR (this is a regular language)
    ====================================
    non_zero_digit ::= '1' | ... | '9'
    digit          ::= '0' | non_zero_digit
    int            ::= non_zero_digit { digit }
    modifier       ::= '+' int
                     | '-' int
    roll           ::= int 'd' int [ modifier ]
    */
    lazy_static! {
        static ref ROLL_RE: Regex = Regex::new(r"(?x)
            ^
            ([1-9][0-9]*)             # Number of dice
            d                         # The literal 'd'
            ([1-9][0-9]*)             # Number of faces
            ([+-][1-9][0-9]*)?        # Optional extra
            $
        ").unwrap();
    }

    match ROLL_RE.captures(line) {
        Some(cap) => {
            let nd = cap.at(1).unwrap().parse::<f32>().unwrap();
            let nf = cap.at(2).unwrap().parse::<f32>().unwrap();
            let ex = cap.at(3).unwrap_or("0").parse::<f32>().unwrap();
            let roll = Roll {
                num_dice: nd,
                num_faces: nf,
                extra: ex,
            };
            if single_line {
                println!("{}", roll.print());
            } else {
                println!("{}", roll.pretty_print());
            }
        }

        None => {
        errmsg(&format!("invalid format: {}", line));
    }
}
}


fn main() {
    let argv: Vec<String> = args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help message");
    opts.optflag("s", "single-line", "single line display");

    let matches = match opts.parse(&argv[1..]) {
        Ok(m) => m,
        Err(e) => {
            errmsg(&format!("{}", e));
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        usage(&opts, &argv[0]);
        process::exit(0);
    }

    let single_line = matches.opt_present("s");

    if !matches.free.is_empty() {
        for arg in matches.free.iter() {
            parse_and_print(arg, single_line);
        }
    } else {
        let stdin = io::stdin();
        let mut buf = String::new();
        while stdin.read_line(&mut buf).unwrap() > 0 {
            parse_and_print(buf.trim(), single_line);
            buf.clear();
        }
    }
}
