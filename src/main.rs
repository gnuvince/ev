use std::env::args;
use std::process::exit;

/*
TODO(vfoley):
- Implement D&D-like dice syntax (c.f.: GRAMMAR)
- Display: minimum, maximum, expected value
- Flags: --min, --max, --ev, --all [default]

GRAMMAR
=======
non_zero_digit ::= '1' | ... | '9'
digit          ::= '0' | non_zero_digit
int            ::= non_zero_digit { digit }
roll           ::= int 'd' int [ modifier ]
modifier       ::= '+' int
                 | '-' int
*/


fn usage() -> ! {
    println!("Usage: ev NUM_DICE NUM_FACES");
    exit(1);
}

fn ev(num_dice: u32, num_faces: u32) -> f64 {
    let nd = num_dice as f64;
    let nf = num_faces as f64;
    let single_die_ev = (nf + 1.0) / 2.0;
    nd * single_die_ev
}

fn main() {
    let argv: Vec<String> = args().collect();
    if argv.len() != 3 {
        usage();
    }

    match (argv[1].parse::<u32>(), argv[2].parse::<u32>()) {
        (Ok(nd), Ok(nf)) => println!("{}", ev(nd, nf)),
        (_, _) => usage(),
    }
}
