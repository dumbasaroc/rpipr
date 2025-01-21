use crate::prcalc::*;
use std::error::Error;
use graphql_client::{GraphQLQuery, Response};
use lazy_static::lazy_static;
use reqwest::Client;

const STARTGG_ENDPOINT: &str = "https://api.start.gg/gql/alpha";

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/testquery.graphql",
    response_derives = "Debug"
)]
pub struct TournamentQuery;

pub type TournamentQueryVariables = tournament_query::Variables;

lazy_static!{
    pub static ref CLIENT_THREADPOOL: Client = Client::new();
}

fn add_standings_to_pr(data: &Response<tournament_query::ResponseData>, pr: &mut PowerRankings) {
    let inner_data = match &data.data {
        Some(d) => d,
        None => {
            println!("There is no data in this response!");
            return;
        }
    };

    let event = match &inner_data.event {
        Some(e) => e,
        None => {
            println!("There is no event in this response!");
            return;
        }
    };

    // Add tournament

    let tournament_name = event.name.as_ref().unwrap().clone();
    let tournament_entrants = event.num_entrants.unwrap() as u32;
    let tournament_id = pr.add_tournament(tournament_name, tournament_entrants);

    // End Add Tournament

    let standings = match &event.standings {
        Some(s) => s,
        None => {
            println!("There are no standings in this response!");
            return;
        }
    };

    for opt_player in standings.nodes.as_ref().unwrap() {
        if let Some(player) = opt_player {
            let player_name = player.player.as_ref().unwrap().gamer_tag.as_ref().unwrap().clone();
            let placement = player.placement.unwrap() as u32;
            // println!("Player: {:20}   Placement: {:4}",
                // player_name,
                // placement);

            pr.add_player(player_name.clone());
            pr.add_placement_to_player(player_name.clone(), tournament_id, placement).unwrap();
        }
    }
}


pub async fn do_query(variables: tournament_query::Variables, pr: &mut PowerRankings) -> Result<(), Box<dyn Error>> {
    // this is the important line
    let request_body = TournamentQuery::build_query(variables);

    let res = CLIENT_THREADPOOL.post(STARTGG_ENDPOINT)
        .bearer_auth("c3b341cc8234d141f75fee5e48ccb953")
        .json(&request_body)
        .send();

    let res = res.await?;
    let response_body: Response<tournament_query::ResponseData> = res.json().await?;

    add_standings_to_pr(&response_body, pr);
    Ok(())
}