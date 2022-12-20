use std::collections::{HashMap, HashSet};

use anyhow::Result as AnyhowResult;
use reqwest;
use scraper::{Html, Selector};

const TARGET_URL: &str = "http://rwr.runningwithrifles.com/rwr_stats/view_players.php";
const SELECTOR_MATCH: &str = "table > tbody > tr";
const PAGE_SIZE: u8 = 100;

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

            // data
            for (index, td) in element.select(&quick_selector("td")).enumerate() {

                match td.text().next() {
                    Some(t) => {
                        let key = property_map.iter().nth(index);
                        println!("data: {:?}: {}", key, t);
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
