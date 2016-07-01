extern crate regex;

use std::env::args;

use regex::Regex;


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

    fn print(&self) {
        println!("\tmin: {}", self.min());
        println!("\tmax: {}", self.max());
        println!("\tev : {}", self.ev());
    }
}

fn main() {
    let argv: Vec<String> = args().collect();

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
    let roll_re = Regex::new(r"(?x)
      ^
      ([1-9][0-9]*)             # Number of dice
      d                         # The literal 'd'
      ([1-9][0-9]*)             # Number of faces
      ([+-][1-9][0-9]*)?        # Optional extra
      $
    ").unwrap();

    for arg in &argv[1..] {
        match roll_re.captures(arg) {
            Some(cap) => {
                let nd = cap.at(1).unwrap().parse::<f32>().unwrap();
                let nf = cap.at(2).unwrap().parse::<f32>().unwrap();
                let ex = cap.at(3).unwrap_or("0").parse::<f32>().unwrap();
                let roll = Roll {
                    num_dice: nd,
                    num_faces: nf,
                    extra: ex,
                };
                println!("{}", *arg);
                roll.print();
            }

            None => {
                let mut stderr = io::stderr();
                writeln!(stderr, "ev: invalid format: {}", arg);
            }
        }
    }
}
