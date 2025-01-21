mod constants;
mod prcalc;
mod query;

use futures::executor::block_on;
use indicatif::{MultiProgress, ProgressStyle};
use prcalc::PowerRankings;
use query::do_query;
use std::{fs::File, process::exit};
use std::io::{self, BufRead, Write};
use std::path::Path;

// borrowed from the rust handbook
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn main() {
    let procbars = MultiProgress::new();
    let mut pr: PowerRankings = PowerRankings::new();

    let inputs = match read_lines("tournaments.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Error reading from input file: {}", e);
            exit(1);
        }
    };

    let lines: Vec<String> = inputs.flatten().collect();
    let tournament_gather_bar = procbars.add(indicatif::ProgressBar::new(lines.len() as u64));
    tournament_gather_bar.set_style(ProgressStyle::with_template(
        "Getting Tournament Data  {pos:>3}/{len:3} {bar:>30.cyan}"
    ).unwrap());
    for line in lines {
        if line == "" { continue; }
        // println!("Tournament: {}", line);

        block_on(do_query(query::TournamentQueryVariables {
            event_slug: Some(line.clone())
        },
        &mut pr)).unwrap();
        tournament_gather_bar.inc(1);
    }
    tournament_gather_bar.finish();

    let mut outfile = match File::create("output.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Error creating output file: {}", e);
            exit(1);
        }
    };

    /* ---- SECTION 0: QUALIFICATION CHECK ---- */

    let qualified_player_bar = procbars.add(indicatif::ProgressBar::new(1));
    qualified_player_bar.set_style(ProgressStyle::with_template(
        "Checking who qualified   {pos:>3}/{len:3} {bar:>30.cyan}"
    ).unwrap());

    pr.get_only_qualified_players(&qualified_player_bar);
    qualified_player_bar.finish();

    /* ---- SECTION 1: AVERAGE PLACEMENTS ---- */

    let score_avg_placement_bar = procbars.add(indicatif::ProgressBar::new(1));
    score_avg_placement_bar.set_style(ProgressStyle::with_template(
        "Scoring: Average Placement   {pos:>3}/{len:3} {bar:>30.cyan}"
    ).unwrap());

    pr.calculate_scoring(constants::CalculationMethods::AveragePlacement, &score_avg_placement_bar);

    score_avg_placement_bar.finish();

    writeln!(outfile, "Average Placement Statistics").unwrap();
    writeln!(outfile, "============================").unwrap();
    pr.export_ordered(&mut outfile, prcalc::PROrder::LoFirst, 12, |placement, p, f| {
        writeln!(f, "{:>3}  {:30}  AVG = {:5.4}", placement, p.get_name(), p.get_score()).unwrap();
        // writeln!(f, "{:?}", p).unwrap();
    });

    writeln!(outfile, "").unwrap();
    writeln!(outfile, "").unwrap();

    /* ---- SECTION 2: WEIGHTED POINTS ---- */

    writeln!(outfile, "Weighted Points (based on minimum # entrants)").unwrap();
    writeln!(outfile, "=============================================").unwrap();

    let weighted_score_bar = procbars.add(indicatif::ProgressBar::new(1));
    weighted_score_bar.set_style(ProgressStyle::with_template(
        "Scoring: Weighted Points   {pos:>3}/{len:3} {bar:>30.cyan}"
    ).unwrap());

    pr.calculate_scoring(constants::CalculationMethods::WeightedPoints, &weighted_score_bar);

    weighted_score_bar.finish();

    pr.export_ordered(&mut outfile, prcalc::PROrder::HiFirst, 12, |placement, p, f| {
        writeln!(f, "{:>3}  {:30}  AVG = {:5.4}", placement, p.get_name(), p.get_score()).unwrap();
        // writeln!(f, "{:?}", p).unwrap();
    });

    writeln!(outfile, "").unwrap();
    writeln!(outfile, "").unwrap();

    /* ---- SECTION 3: MEDIAN POINTS ---- */

    writeln!(outfile, "Median Points Statistics").unwrap();
    writeln!(outfile, "========================").unwrap();

    let median_score_bar = procbars.add(indicatif::ProgressBar::new(1));
    median_score_bar.set_style(ProgressStyle::with_template(
        "Scoring: Median Points   {pos:>3}/{len:3} {bar:>30.cyan}"
    ).unwrap());

    pr.calculate_scoring(constants::CalculationMethods::MedianPoints, &median_score_bar);

    median_score_bar.finish();

    pr.export_ordered(&mut outfile, prcalc::PROrder::HiFirst, 12, |placement, p, f| {
        writeln!(f, "{:>3}  {:30}  AVG = {:5.4}", placement, p.get_name(), p.get_score()).unwrap();
        // writeln!(f, "{:?}", p).unwrap();
    });

    writeln!(outfile, "").unwrap();
    writeln!(outfile, "").unwrap();

    /* ---- SECTION 4: MEAN POINTS ---- */

    writeln!(outfile, "Mean Points Statistics").unwrap();
    writeln!(outfile, "======================").unwrap();

    let mean_score_bar = procbars.add(indicatif::ProgressBar::new(1));
    mean_score_bar.set_style(ProgressStyle::with_template(
        "Scoring: Mean Points   {pos:>3}/{len:3} {bar:>30.cyan}"
    ).unwrap());

    pr.calculate_scoring(constants::CalculationMethods::MeanPoints, &mean_score_bar);

    mean_score_bar.finish();

    pr.export_ordered(&mut outfile, prcalc::PROrder::HiFirst, 12, |placement, p, f| {
        writeln!(f, "{:>3}  {:30}  AVG = {:5.4}", placement, p.get_name(), p.get_score()).unwrap();
        // writeln!(f, "{:?}", p).unwrap();
    });

    writeln!(outfile, "").unwrap();
    writeln!(outfile, "").unwrap();

    /* ---- SECTION 5: UNWEIGHTED POINTS ---- */

    writeln!(outfile, "Unweighted Points Statistics").unwrap();
    writeln!(outfile, "============================").unwrap();

    let unweighted_score_bar = procbars.add(indicatif::ProgressBar::new(1));
    unweighted_score_bar.set_style(ProgressStyle::with_template(
        "Scoring: Unweighted Points   {pos:>3}/{len:3} {bar:>30.cyan}"
    ).unwrap());

    pr.calculate_scoring(constants::CalculationMethods::UnweightedPoints, &unweighted_score_bar);

    unweighted_score_bar.finish();

    pr.export_ordered(&mut outfile, prcalc::PROrder::HiFirst, 12, |placement, p, f| {
        writeln!(f, "{:>3}  {:30}  AVG = {:5.4}", placement, p.get_name(), p.get_score()).unwrap();
        // writeln!(f, "{:?}", p).unwrap();
    });

    writeln!(outfile, "").unwrap();
    writeln!(outfile, "").unwrap();

    /* ---- SECTION 6: OVERALL PR ORDERING ---- */

    writeln!(outfile, "Overall PR Ordering").unwrap();
    writeln!(outfile, "===================").unwrap();

    let pr_placement_bar = procbars.add(indicatif::ProgressBar::new(1));
    pr_placement_bar.set_style(ProgressStyle::with_template(
        "Scoring: PR Placement   {pos:>3}/{len:3} {bar:>30.cyan}"
    ).unwrap());

    pr.calculate_scoring(constants::CalculationMethods::OverallPRPlacement, &pr_placement_bar);

    pr_placement_bar.finish();

    pr.export_ordered(&mut outfile, prcalc::PROrder::LoFirst, 20, |placement, p, f| {
        writeln!(f, "{:>3}  {:30}  AVG = {:5.4}", placement, p.get_name(), p.get_score()).unwrap();
        // writeln!(f, "{:?}", p).unwrap();
    });

    writeln!(outfile, "").unwrap();
    writeln!(outfile, "").unwrap();
}
