use clap::{ArgAction, Parser};
use cutting_optimizer::{
    cutter::RodStock,
    engine::{ascii_visualize, print_report, CuttingEngine},
};

/// Cutting optimizer CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// List of rod lengths with quantities, e.g. "100x2,80x1,60x3"
    #[arg(short, long)]
    rods: String,

    /// Cuts to perform, e.g. "20,30,50,10,40"
    #[arg(short, long)]
    cuts: String,

    /// Main rod length for DP
    #[arg(short, long, default_value_t = 100)]
    dp_length: i32,

    /// Show ASCII visualization
    #[arg(short = 'v', long = "visualize", action = ArgAction::SetTrue)]
    visualize: bool,
}

fn parse_rods(input: &str) -> Vec<RodStock> {
    input
        .split(',')
        .filter_map(|entry| {
            let mut parts = entry.split('x');
            let length = parts.next()?.parse::<i32>().ok()?;
            let qty = parts.next()?.parse::<usize>().ok()?;
            Some(RodStock { length, qty })
        })
        .collect()
}

fn parse_cuts(input: &str) -> Vec<i32> {
    input
        .split(',')
        .filter_map(|c| c.trim().parse::<i32>().ok())
        .collect()
}

fn main() {
    let cli = Cli::parse();

    let rods = parse_rods(&cli.rods);
    let cuts = parse_cuts(&cli.cuts);

    let mut engine = CuttingEngine::new(rods);

    match engine.execute(cli.dp_length, cuts) {
        Ok(result) => {
            println!("=== CUTTING REPORT ===");
            print_report(&result);

            if cli.visualize {
                ascii_visualize(&result.order, &result.rods_flat, &result.plan);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
