use anyhow::Result as AnyhowResult;
use reqwest;
use scraper::{Html, Selector};

const TARGET_URL: &str = "http://rwr.runningwithrifles.com/rwr_stats/view_players.php";
const SELECTOR_MATCH: &str = "table > tbody > tr";

fn quick_selector(exp: &str) -> Selector {
    Selector::parse(exp).unwrap()
}

// ref: https://zhuanlan.zhihu.com/p/516033159
fn main() -> AnyhowResult<()> {
    println!("Target url: {}", TARGET_URL);

    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(TARGET_URL)
        .query(&[
            ("db", "invasion"),
            ("sort", "rank_progression"),
            ("start", "0"),
        ])
        .send()?
        .text()?;

    // println!("{:?}", resp);
    //
    let fragment = Html::parse_fragment(&resp);
    let selector = quick_selector(SELECTOR_MATCH);

    for element in fragment.select(&selector) {
        println!("tr element: {:?}", element.value());
        for th in element.select(&quick_selector("th")) {
            println!("th element: {:?}", th.value());

            for div in th.select(&quick_selector("div")) {
                let property_name = div.value().classes().into_iter().next().unwrap();
                println!("div element class: {}", property_name);
            }
        }

        break;
    }


    Ok(())
}
