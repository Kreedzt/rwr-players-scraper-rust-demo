use std::collections::{HashMap, HashSet};

use anyhow::Result as AnyhowResult;
use reqwest;
use scraper::{Html, Selector};

const TARGET_URL: &str = "http://rwr.runningwithrifles.com/rwr_stats/view_players.php";
const SELECTOR_MATCH: &str = "table > tbody > tr";
const PAGE_SIZE: u8 = 100;

const DROP_TABLE_SQL: &str = "DROP TABLE IF EXISTS rwr_players";

const CREATE_TABLE_SQL: &str = "CREATE TABLE \"rwr_players\" (
	\"id\" INTEGER NOT NULL,
	\"username\" TEXT NOT NULL,
	\"kills\" BIGINT NOT NULL,
	\"deaths\" BIGINT NOT NULL,
	\"score\" BIGINT NOT NULL,
	\"time_played\" BIGINT NOT NULL,
	\"longest_kill_streak\" BIGINT NOT NULL,
	\"targets_destroyed\" BIGINT NOT NULL,
	\"soldiers_healed\" BIGINT NOT NULL,
	\"teamkills\" BIGINT NOT NULL,
	\"distance_moved\" REAL NOT NULL,
	\"shots_fired\" BIGINT NOT NULL,
	\"throwables_thrown\" BIGINT NOT NULL,
	\"rank_progression\" BIGINT NOT NULL,
	\"rank_name\" TEXT NOT NULL,
	PRIMARY KEY (\"id\")
);";

struct Player {
    username: String,
    kills: i128,
    deaths: i128,
    score: i128,
    // N minutes count
    time_played: i128,
    longest_kill_streak: i128,
    targets_destroyed: i128,
    soldiers_healed: i128,
    teamkills: i128,
    // x.y km
    distance_moved: f64,
    shots_fired: i128,
    throwables_thrown: i128,
    // XP
    rank_progression: i128,
    // Private / True rank name
    rank_name: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            kills: 0,
            deaths: 0,
            score: 0,
            time_played: 0,
            longest_kill_streak: 0,
            targets_destroyed: 0,
            soldiers_healed: 0,
            teamkills: 0,
            distance_moved: 0.0,
            shots_fired: 0,
            throwables_thrown: 0,
            rank_progression: 0,
            rank_name: "".to_string()
        }
    }
}

fn quick_selector(exp: &str) -> Selector {
    Selector::parse(exp).unwrap()
}

// ref: https://zhuanlan.zhihu.com/p/516033159
fn main() -> AnyhowResult<()> {
    println!("Target url: {}", TARGET_URL);

    let client = reqwest::blocking::Client::new();

    // End: 148900
    // No data: 149000
    let mut current_start = 148800;
    // let mut current_start = 0;

    loop {
        let resp = client
            .get(TARGET_URL)
            .query(&[
                ("db", "invasion"),
                ("sort", "rank_progression"),
                ("start", &current_start.to_string()),
            ])
            .send()?
            .text()?;

        let fragment = Html::parse_fragment(&resp);
        let selector = quick_selector(SELECTOR_MATCH);

        let mut property_map: Vec<String> = vec![];

        let mut data_size: i128 = -1;

        for element in fragment.select(&selector) {
            println!("Start Parsing... start:{}, data(before):{}", current_start, data_size);
            // println!("tr element: {:?}", element.value());

            // column name
            for th in element.select(&quick_selector("th")) {
                // println!("th element: {:?}", th.value());

                for div in th.select(&quick_selector("div")) {
                    let property_name = div.value().classes().into_iter().next().unwrap();
                    println!("Parsing... column head: {}", property_name);

                    // println!("div element class: {}", property_name);

                    property_map.push(property_name.to_string());
                }
            }

            let mut player = Player::default();

            // data
            for (index, td) in element.select(&quick_selector("td")).enumerate() {

                match td.text().next() {
                    Some(t) => {
                        let key = property_map.iter().nth(index);

                        // println!("data: {:?}: {}", k, t);

                        if let Some(k) = key {
                            println!("data: {:?}: {}", k, t);
                            match k.as_str() {
                                "username" => {
                                    player.username = String::from(t);
                                    println!("username: {}", t);
                                }
                                "kills" => {
                                    player.kills = t.parse()?;
                                    println!("kills: {}", t);
                                }
                                "deaths" => {
                                    player.deaths = t.parse()?;
                                    println!("deaths: {}", t);
                                }
                                "score" => {
                                    player.score = t.parse()?;
                                    println!("score: {}", t);
                                }
                                "time_played" => {
                                    // TODO
                                    // player.time_played = t.parse()?;
                                    println!("time_played: {}", t);
                                }
                                "longest_kill_streak" => {
                                    player.longest_kill_streak = t.parse()?;
                                    println!("longest_kill_streak: {}", t);
                                }
                                "targets_destroyed" => {
                                    player.targets_destroyed = t.parse()?;
                                    println!("targets_destroyed: {}", t);
                                }
                                "soldiers_healed" => {
                                    player.soldiers_healed = t.parse()?;
                                    println!("soldiers_healed: {}", t);
                                }
                                "teamkills" => {
                                    player.teamkills = t.parse()?;
                                    println!("teamkills: {}", t);
                                }
                                "distance_moved" => {
                                    // TODO
                                    // player.teamkills = t.parse()?;
                                    println!("distance_moved: {}", t);
                                }
                                "shots_fired" => {
                                    player.shots_fired = t.parse()?;
                                    println!("shots_fired: {}", t);
                                }
                                "throwables_thrown" => {
                                    player.throwables_thrown = t.parse()?;
                                    println!("throwables_thrown: {}", t);
                                }
                                "rank_progression" => {
                                    player.rank_progression = t.parse()?;
                                    println!("rank_progression: {}", t);
                                }
                                "rank_name" => {
                                    player.rank_name = String::from(t);
                                    println!("rank_name: {}", t);
                                }
                                _ => {
                                    println!("Not Found match: {}", t);
                                }
                            }
                        }
                    }
                    _ => {
                        // img, ignore it
                    }
                }
            }

            data_size = data_size + 1;
        }

        println!("Parsing completed, start:{}, data(after):{}", current_start, data_size);

        if data_size < PAGE_SIZE.into() {
            println!("Parsing End");
            if data_size != -1 {
                println!("Total data: {}", current_start + data_size);
            } else {
                println!("Total data: {}", current_start);
            }
            break;
        }

        current_start = current_start + PAGE_SIZE as i128;
    }

    Ok(())
}

// INSERT INTO rwr (username, kills, deaths, score,time_played,longest_kill_streak,targets_destroyed,soldiers_healed,teamkills,distance_moved,shots_fired,throwables_thrown,rank_progression,rank_name)
// VALUES('TEST222',0,0,0, 0,0,0,0,0,0.0,0,0,0,'Private')

// INSERT INTO rwr (username, kills, deaths, score,time_played,longest_kill_streak,targets_destroyed,soldiers_healed,teamkills,distance_moved,shots_fired,throwables_thrown,rank_progression,rank_name)
// VALUES('TEST111',220720,20853,199867, 103128,625,5056,3089,4571,2134.3,2051204,19434,3005651,'General of the Army')
