/*!
A simple command-line utility to compute
the minimum, maximum and expected value
of a dice roll expressed in D&D notation.
*/

extern crate regex;
extern crate getopts;
#[macro_use] extern crate lazy_static;

use std::env::args;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use std::process;

use regex::Regex;
use getopts::Options;

/// An output style.
///
/// Dictates whether the min/max/ev output should
/// be displayed on a single line (useful for Unix
/// pipe-lines) or on multiple lines (more readable).
pub enum OutputStyle {
    SingleLine,
    MultiLine,
}

/// The types of errors that can occur in ev:
///
/// - InvalidFormat:
///     a valid roll format is XdY, XdY+Z, or XdY-Z;
///     trying to parse a roll specification that has
///     the wrong format returns this error;
/// - TooManyDice:
///     if a roll has a correct format, but the number
///     of dice exceeds 2^16.
/// - TooManySides:
///     if a roll has a correct format, but the number
///     of sides exceeds 2^16.
/// - ExtraTooLarge:
///     if the bonus/malus of a roll is not between
///     -2^15 and 2^15 - 1.
#[derive(Debug, PartialEq)]
pub enum EvError {
    InvalidFormat,
    TooManyDice,
    TooManySides,
    ExtraTooLarge,
}

impl Error for EvError {
    fn description(&self) -> &'static str {
        match *self {
            EvError::InvalidFormat => "invalid format",
            EvError::TooManyDice => "too many dice",
            EvError::TooManySides => "too many sides",
            EvError::ExtraTooLarge => "bonus too large",
        }
    }
}

impl fmt::Display for EvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

/// A dice roll.
///
/// A dice roll has three components:
///
/// - A number of dice (positive integer);
/// - A number of faces (positive integer);
/// - An extra (e.g., +3 or -4).
#[derive(Debug, PartialEq)]
pub struct Roll {
    num_dice: u16,
    num_faces: u16,
    extra: i16,
}

impl Roll {
    /// Create a new roll.
    pub fn new(num_dice: u16, num_faces: u16, extra: i16) -> Self {
        Roll {
            num_dice: num_dice,
            num_faces: num_faces,
            extra: extra,
        }
    }

    // A small helper method to extract the integer
    // fields as floats for making calculations.
    fn float_values(&self) -> (f32, f32, f32) {
        (self.num_dice as f32,
         self.num_faces as f32,
         self.extra as f32)
    }

    /// Compute the expected value: expected value of one die
    /// multiplied by the number of dice, then add the extra.
    pub fn ev(&self) -> f32 {
        // Math reminder:
        // 1 + 2 + ... + n = n(n+1) / 2
        // therefore
        // 1/n * (1 + 2 + ... + n) = (n+1) / 2
        let (nd, nf, extra) = self.float_values();
        let single_die_ev = (nf + 1.0) / 2.0;
        nd * single_die_ev + extra
    }

    /// Compute the minimum value.
    pub fn min(&self) -> f32 {
        let (nd, _, extra) = self.float_values();
        nd + extra
    }

    /// Compute the maximum value.
    pub fn max(&self) -> f32 {
        let (nd, nf, extra) = self.float_values();
        nd * nf + extra
    }

    /// Display the roll statistics on a single line.
    /// Useful for usage in a Unix pipe-line.
    pub fn print(&self) -> String {
        format!("{} {} {} {}", self, self.min(), self.max(), self.ev())
    }

    /// Display the roll statistics on multiple lines.
    /// Prettier to look at for a human.
    pub fn pretty_print(&self) -> String {
        format!("{}:\n\tmin: {}\n\tmax: {}\n\tev : {}",
                self, self.min(), self.max(), self.ev())
    }
}

/// Convert a roll into the `XdY+Z` notation.
impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.num_dice, self.num_faces)?;
        if self.extra != 0 {
            write!(f, "{:+}", self.extra)?;
        }
        return Ok(());
    }
}

/// Display a message on stderr
fn errmsg(msg: &str) {
    let mut stderr = io::stderr();
    let _ = writeln!(stderr, "ev: {}", msg);
}

/// Show basic usage of the program
fn usage(opts: &Options, progname: &str) {
    let brief = format!(
        concat!(
            "Usage: {} [options] [rolls ...]\n",
            "\troll: XdY, XdY+Z, XdY-Z (e.g. 1d6, 2d4+1, 3d8-1)"),
        progname
    );
    print!("{}", opts.usage(&brief));
}

fn parse(roll_desc: &str) -> Result<Roll, EvError> {
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
            ([1-9][0-9]{0,4})             # Number of dice
            d                             # The literal 'd'
            ([1-9][0-9]{0,4})             # Number of faces
            ([+-][1-9][0-9]{0,4})?        # Optional extra
            $
        ").unwrap();
    }

    let cap = try!(ROLL_RE.captures(roll_desc).ok_or(EvError::InvalidFormat));
    let nd = try!(cap.at(1).unwrap().parse::<u16>().or(Err(EvError::TooManyDice)));
    let nf = try!(cap.at(2).unwrap().parse::<u16>().or(Err(EvError::TooManySides)));
    let ex = try!(cap.at(3).unwrap_or("0").parse::<i16>().or(Err(EvError::ExtraTooLarge)));
    Ok(Roll::new(nd, nf, ex))

}

fn parse_and_print(line: &str, output_style: &OutputStyle) {
    match parse(line) {
        Ok(roll) => {
            match *output_style {
                OutputStyle::SingleLine => {
                    println!("{}", roll.print());
                }
                OutputStyle::MultiLine => {
                    println!("{}", roll.pretty_print());
                }
            }
        }
        Err(ev_error) => {
            errmsg(&format!("{}: {}", ev_error, line));
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

    let output_style =
        if matches.opt_present("s") {
            OutputStyle::SingleLine
        } else {
            OutputStyle::MultiLine
        };

    // Read the rolls from the positional command-line
    // arguments if there are any, otherwise read rolls
    // from stdin.
    if !matches.free.is_empty() {
        for arg in matches.free.iter() {
            parse_and_print(arg, &output_style);
        }
    } else {
        let stdin = io::stdin();
        let mut buf = String::with_capacity(32);
        while stdin.read_line(&mut buf).unwrap() > 0 {
            parse_and_print(buf.trim(), &output_style);
            buf.clear();
        }
    }
}

#[test]
fn test_roll() {
    let r = Roll::new(1, 6, 0);
    assert_eq!(r.min(), 1.0);
    assert_eq!(r.max(), 6.0);
    assert_eq!(r.ev(), 3.5);

    let r = Roll::new(2, 6, 0);
    assert_eq!(r.min(), 2.0);
    assert_eq!(r.max(), 12.0);
    assert_eq!(r.ev(), 7.0);

    let r = Roll::new(1, 6, 1);
    assert_eq!(r.min(), 2.0);
    assert_eq!(r.max(), 7.0);
    assert_eq!(r.ev(), 4.5);

    let r = Roll::new(1, 6, -1);
    assert_eq!(r.min(), 0.0);
    assert_eq!(r.max(), 5.0);
    assert_eq!(r.ev(), 2.5);
}

#[test]
fn test_print() {
    let r = Roll::new(1, 6, 0);
    assert_eq!(format!("{}", r), "1d6");
    let r = Roll::new(2, 4, 1);
    assert_eq!(format!("{}", r), "2d4+1");
    let r = Roll::new(3, 10, -1);
    assert_eq!(format!("{}", r), "3d10-1");
}

#[test]
fn test_parse() {
    assert_eq!(parse(""), Err(EvError::InvalidFormat));
    assert_eq!(parse("d"), Err(EvError::InvalidFormat));
    assert_eq!(parse("5d"), Err(EvError::InvalidFormat));
    assert_eq!(parse("d5"), Err(EvError::InvalidFormat));
    assert_eq!(parse("+5"), Err(EvError::InvalidFormat));
    assert_eq!(parse("-5"), Err(EvError::InvalidFormat));
    assert_eq!(parse("XdY"), Err(EvError::InvalidFormat));
    assert_eq!(parse("123456d2"), Err(EvError::InvalidFormat));
    assert_eq!(parse("1d123456"), Err(EvError::InvalidFormat));
    assert_eq!(parse("1d2+123456"), Err(EvError::InvalidFormat));
    assert_eq!(parse("1d2-123456"), Err(EvError::InvalidFormat));

    assert_eq!(parse("99999d2"), Err(EvError::TooManyDice));
    assert_eq!(parse("2d99999"), Err(EvError::TooManySides));
    assert_eq!(parse("1d6+99999"), Err(EvError::ExtraTooLarge));
    assert_eq!(parse("1d6-99999"), Err(EvError::ExtraTooLarge));
}
