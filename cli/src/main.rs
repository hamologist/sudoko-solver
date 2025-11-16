use sudoko_solver::{Solver, Outputer};

fn main() {
    let matches = clap::Command::new("sudoko-solver")
        .about("Attempts to solve a provided Sudoko puzzle.")
        .arg(
            clap::Arg::new("as_json")
                .long("as-json")
                .action(clap::ArgAction::SetTrue)
                .help("Changes output to JSON."),
        )
        .arg(
            clap::Arg::new("file")
                .default_value("-")
                .action(clap::ArgAction::Set)
                .help("Reads input from the provided file or STDIN if no value is provided"),
        )
        .get_matches();

    let as_json = matches.get_one::<bool>("as_json").unwrap();
    let filename = matches.get_one::<String>("file").unwrap();
    let mut input_reader: Box<dyn std::io::Read> = match filename {
        _ if filename == "-" => Box::new(std::io::stdin()),
        _ => match std::fs::File::open(filename) {
            Ok(file) => Box::new(file),
            Err(_) => {
                println!("Failed to open provided file.");
                return;
            }
        },
    };

    let mut input: String = String::new();
    match input_reader.read_to_string(&mut input) {
        Ok(_) => {}
        Err(_) => {
            if filename == "-" {
                println!("Failed to read input from STDIN.");
            } else {
                println!("Failed to read input from file.");
            }
            return;
        }
    };
    let solver = match Solver::from_csv(input.trim()) {
        Ok(solver) => solver,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    let solved_board = match solver.solve() {
        Ok(solved_board) => solved_board,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        },
    };
    match as_json {
        true => println!("{}", solved_board.to_json()),
        false => println!("{}", solved_board.to_csv()),
    }
}
