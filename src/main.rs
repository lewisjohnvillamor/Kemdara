mod app;

use std::process::ExitCode;

fn main() -> ExitCode {
    match parse_command() {
        Ok(Command::Gui) => run_gui(),
        Ok(Command::Cli { iterations }) => run_cli(iterations),
        Ok(Command::Help) => {
            print_help();
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}\n");
            print_help();
            ExitCode::from(2)
        }
    }
}

enum Command {
    Gui,
    Cli { iterations: usize },
    Help,
}

fn parse_command() -> Result<Command, String> {
    let mut cli = false;
    let mut iterations = 100_usize;
    let mut arguments = std::env::args().skip(1);

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--cli" => cli = true,
            "--iterations" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| "--iterations requires a value".to_owned())?;
                iterations = value
                    .parse()
                    .map_err(|_| "--iterations must be a whole number".to_owned())?;
            }
            "--help" | "-h" => return Ok(Command::Help),
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }

    if !(1..=10_000).contains(&iterations) {
        return Err("--iterations must be between 1 and 10,000".into());
    }

    if cli {
        Ok(Command::Cli { iterations })
    } else {
        Ok(Command::Gui)
    }
}

fn run_cli(iterations: usize) -> ExitCode {
    let report = kemdara::run_benchmarks(iterations);
    match serde_json::to_string_pretty(&report) {
        Ok(json) => {
            println!("{json}");
            if report.results.iter().all(|result| result.successful) {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            eprintln!("could not serialize benchmark report: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_gui() -> ExitCode {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1_050.0, 720.0])
            .with_min_inner_size([820.0, 600.0]),
        ..Default::default()
    };

    match eframe::run_native(
        "Kemdara",
        options,
        Box::new(|context| Ok(Box::new(app::KemdaraApp::new(context)))),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("could not start Kemdara: {error}");
            ExitCode::FAILURE
        }
    }
}

fn print_help() {
    println!(
        "Kemdara — local cryptography experimentation workbench\n\n\
         Usage:\n  kemdara                         Open the GUI\n  \
         kemdara --cli [--iterations N]   Print a JSON benchmark report\n  \
         kemdara --help                    Show this help\n"
    );
}
