use std::collections::{HashMap, HashSet};
use crate::constants::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TournamentDetails {
    pub tournament_name: String,
    pub tournament_entrants: u32
}

pub enum PROrder {
    LoFirst,
    HiFirst
}

#[derive(Debug)]
pub struct PowerRankings {
    tournaments: Vec<TournamentDetails>,
    players: HashSet<Player>
}

impl PowerRankings {
    pub fn new() -> Self {
        PowerRankings {
            tournaments: vec![],
            players: HashSet::new()
        }
    }

    pub fn add_tournament(&mut self, name: impl Into<String>, num_players: u32) -> u32 {
        let details = TournamentDetails {
            tournament_name: name.into(),
            tournament_entrants: num_players
        };
        let new_tournament_id = self.tournaments.len() as u32;

        self.tournaments.push(details);
        new_tournament_id
    }

    pub fn add_player(&mut self, name: impl Into<String>) {
        let player = Player::new(name);
        self.players.insert(player);
    }

    pub fn add_placement_to_player(&mut self, player_name: impl Into<String>,
            tournament_id: u32, placement: u32) -> Result<(), String> {
        
        let player_finder = Player::new(player_name);
        let player_ref = match self.players.get(&player_finder) {
            Some(p) => p,
            None => {
                return Err(format!("No such player by the name \"{}\"!", player_finder.name));
            }
        };

        let mut edited_player = player_ref.clone();
        edited_player.add_player_to_tournament(tournament_id, placement);
        self.players.replace(edited_player);

        Ok(())
    }

    pub fn get_only_qualified_players(&mut self, bar: &indicatif::ProgressBar) {
        bar.set_length(self.players.len() as u64);
        let mut tmp_hash_set: HashSet<Player> = HashSet::new();
        for p in &self.players {
            if p.qualifies_for_pr() {
                tmp_hash_set.insert(p.clone());
            }
            bar.inc(1);
        }

        self.players = tmp_hash_set;
    }

    pub fn calculate_scoring(&mut self, score_type: CalculationMethods, bar: &indicatif::ProgressBar) {
        bar.set_length(self.players.len() as u64);
        match score_type {
            CalculationMethods::AveragePlacement => {
                let mut tmp_vec: Vec<Player> = self.players.drain().collect();
                for mut p in tmp_vec.iter_mut() {
                    self.average_placement_score_fn(&mut p);
                    bar.inc(1);
                }
                self.players = tmp_vec.drain(0..).collect();
            },
            CalculationMethods::WeightedPoints => {
                let mut tmp_vec: Vec<Player> = self.players.drain().collect();
                for mut p in tmp_vec.iter_mut() {
                    self.weighted_points_score_fn(&mut p);
                    bar.inc(1);
                }
                self.players = tmp_vec.drain(0..).collect();
            },
            CalculationMethods::MedianPoints => {
                let mut tmp_vec: Vec<Player> = self.players.drain().collect();
                for mut p in tmp_vec.iter_mut() {
                    self.median_points_score_fn(&mut p);
                    bar.inc(1);
                }
                self.players = tmp_vec.drain(0..).collect();
            },
            CalculationMethods::MeanPoints => {
                let mut tmp_vec: Vec<Player> = self.players.drain().collect();
                for mut p in tmp_vec.iter_mut() {
                    self.mean_points_score_fn(&mut p);
                    bar.inc(1);
                }
                self.players = tmp_vec.drain(0..).collect();
            },
            CalculationMethods::UnweightedPoints => {
                let mut tmp_vec: Vec<Player> = self.players.drain().collect();
                for mut p in tmp_vec.iter_mut() {
                    self.unweighted_points_score_fn(&mut p);
                    bar.inc(1);
                }
                self.players = tmp_vec.drain(0..).collect();
            },
            CalculationMethods::OverallPRPlacement => {
                let mut tmp_vec: Vec<Player> = self.players.drain().collect();
                for mut p in tmp_vec.iter_mut() {
                    self.pr_category_score_fn(&mut p);
                    bar.inc(1);
                }
                self.players = tmp_vec.drain(0..).collect();
            },
        }
    }

    fn average_placement_score_fn(&mut self, player: &mut Player) {
        let mut total: f64 = 0.0;
        let worst_placement: u32 = player.get_worst_placement();
        for placement in &player.placements {
            total += *placement.1 as f64;
        }

        if player.get_num_tournaments_entered() > 4 {
            total -= worst_placement as f64;
        }
    
        total = total / ((player.get_num_tournaments_entered() - 1) as f64).max(4.0);
        player.score = total;
    }

    fn weighted_points_score_fn(&mut self, player: &mut Player) {
        let mut total: f64 = 0.0;
        let mut worst_score: f64 = 10000.0;
        for (tournament_id, placement) in &player.placements {
            let val: f64 =
                (self.tournaments.get(*tournament_id as usize).unwrap().tournament_entrants as f64
                    / MINIMUM_ENTRANT_COUNT as f64) * point_values(*placement);
            if val < worst_score {
                worst_score = val;
            }
            total += val;
        }

        if player.get_num_tournaments_entered() > 4 {
            total -= worst_score;
        }
    
        total = total / ((player.get_num_tournaments_entered() - 1) as f64).max(4.0);
        player.score = total;
    }

    fn median_points_score_fn(&mut self, player: &mut Player) {
        let mut tournament_entrants: Vec<u32> = self.tournaments.iter()
            .map(|o| {o.tournament_entrants})
            .collect();
        tournament_entrants.sort();
        let med_entrants: f64 = match tournament_entrants.len() % 2 {
            0 => {
                (*tournament_entrants.get(tournament_entrants.len() / 2).unwrap() as f64 +
                *tournament_entrants.get(tournament_entrants.len() / 2 + 1).unwrap() as f64) / 2.0
            },
            1 => {
                *tournament_entrants.get(tournament_entrants.len() / 2).unwrap() as f64
            }
            _ => 0.0
        };
        // println!("median is {:5.4}", med_entrants);
        drop(tournament_entrants);

        let mut total: f64 = 0.0;
        let mut worst_score: f64 = 10000.0;
        for (tournament_id, placement) in &player.placements {
            let val: f64 =
                (self.tournaments.get(*tournament_id as usize).unwrap().tournament_entrants as f64
                    / med_entrants) * point_values(*placement);
            if val < worst_score {
                worst_score = val;
            }
            total += val;
        }

        if player.get_num_tournaments_entered() > 4 {
            total -= worst_score;
        }
    
        total = total / ((player.get_num_tournaments_entered() - 1) as f64).max(4.0);
        player.score = total;
    }

    fn mean_points_score_fn(&mut self, player: &mut Player) {
        let tournament_entrants: Vec<u32> = self.tournaments.iter()
            .map(|o| {o.tournament_entrants})
            .collect();
            let mean: f64 = tournament_entrants.iter().sum::<u32>() as f64 / tournament_entrants.len() as f64;
        // println!("mean is {:5.4}", mean);
        drop(tournament_entrants);

        let mut total: f64 = 0.0;
        let mut worst_score: f64 = 10000.0;
        for (tournament_id, placement) in &player.placements {
            let val: f64 =
                (self.tournaments.get(*tournament_id as usize).unwrap().tournament_entrants as f64
                    / mean) * point_values(*placement);
            if val < worst_score {
                worst_score = val;
            }
            total += val;
        }

        if player.get_num_tournaments_entered() > 4 {
            total -= worst_score;
        }
    
        total = total / ((player.get_num_tournaments_entered() - 1) as f64).max(4.0);
        player.score = total;
    }

    fn unweighted_points_score_fn(&mut self, player: &mut Player) {
        let mut total: f64 = 0.0;
        let mut worst_score: f64 = 10000.0;
        for (_, placement) in &player.placements {
            let val: f64 = point_values(*placement);
            if val < worst_score {
                worst_score = val;
            }
            total += val;
        }

        if player.get_num_tournaments_entered() > 4 {
            total -= worst_score;
        }
    
        total = total / ((player.get_num_tournaments_entered() - 1) as f64).max(4.0);
        player.score = total;
    }

    fn pr_category_score_fn(&mut self, player: &mut Player) {
        let avg_placement: f64 = player.pr_category_placements.iter().sum::<u32>() as f64 / 5.0;
        player.score = avg_placement;
    }

    pub fn export_ordered(&mut self, outfile: &mut dyn std::io::Write,
            order: PROrder,
            number: u32,
            callback: impl Fn(u32, &Player, &mut dyn std::io::Write) -> ()) {
        let mut tmp_vec: Vec<Player> = self.players.drain().collect();
        match order {
            PROrder::HiFirst => {
                tmp_vec.sort_by(|p, p2| {p2.partial_cmp(p).unwrap()});
            },
            PROrder::LoFirst => {
                tmp_vec.sort_by(|p, p2| {p.partial_cmp(p2).unwrap()});
            }
        }
        let mut placement: u32 = 1;
        for p in tmp_vec.iter_mut() {
            p.pr_category_placements.push(placement);
            if placement <= number {
                callback(placement, &p, outfile);
            }
            placement += 1;
        }
        self.players = tmp_vec.drain(0..).collect();
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    placements: HashMap<u32, u32>,
    pub pr_category_placements: Vec<u32>,
    score: f64
}

impl Player {

    pub fn new(name: impl Into<String>) -> Self {
        Player {
            name: name.into(),
            placements: HashMap::new(),
            pr_category_placements: vec![],
            score: 0.0 // low score, so that this score will never win without edits
        }
    }

    pub fn add_player_to_tournament(&mut self, tournament_id: u32, placement: u32) {
        self.placements.insert(tournament_id, placement);
    }

    pub fn get_worst_placement(&self) -> u32 {
        let mut lowest: u32 = 0;
        for (_, placement) in &self.placements {
            if *placement > lowest {
                lowest = *placement;
            }
        }
        lowest
    }

    pub fn get_num_tournaments_entered(&self) -> u32 {
        self.placements.len() as u32
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_score(&self) -> f64 {
        self.score
    }

    pub fn qualifies_for_pr(&self) -> bool {
        self.get_num_tournaments_entered() >= 4
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Player {
    fn ge(&self, other: &Self) -> bool {
        self.score >= other.score
    }

    fn gt(&self, other: &Self) -> bool {
        self.score > other.score
    }

    fn le(&self, other: &Self) -> bool {
        self.score <= other.score
    }

    fn lt(&self, other: &Self) -> bool {
        self.score < other.score
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self < other { return Some(std::cmp::Ordering::Less); }
        if self > other { return Some(std::cmp::Ordering::Greater); }
        if self <= other && self >= other { return Some(std::cmp::Ordering::Equal); }
        None
    }
}

impl Eq for Player {}

impl std::hash::Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
