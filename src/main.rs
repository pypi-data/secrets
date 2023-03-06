use itertools::Itertools;

use octocrab::Octocrab;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use chrono::prelude::*;
// use futures::StreamExt;
use tinytemplate::TinyTemplate;
use url::Url;

#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
struct Alert {
    secret_type_display_name: String,
    secret: String,
    locations_url: Url,
}

#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
struct AlertLocation {
    details: AlertLocationDetails,
}

#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
struct AlertLocationDetails {
    path: String,
}

#[derive(Debug, Default)]
struct DetectedAlerts {
    secrets: HashSet<String>,
    unique_packages: HashSet<String>,
    count: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let github = octocrab::instance();
    let token = std::env::var("SECRET_SCANNING_TOKEN")
        .expect("SECRET_SCANNING_TOKEN env variable is required");

    let github = Octocrab::builder().personal_token(token).build()?;
    // let names = get_all_repo_names(&github).await?;
    let limit = github.ratelimit().get().await?;
    println!("Rate limit: {:#?}", limit.resources.core);
    let reset_time =
        NaiveDateTime::from_timestamp_opt(limit.resources.core.reset as i64, 0).unwrap();
    let reset_time: DateTime<Utc> = DateTime::from_utc(reset_time, Utc);
    let now = Utc::now();
    let reset = reset_time - now;
    println!(
        "Rate limit resetting @ {reset_time} (in {}h {}s)",
        reset.num_hours(),
        reset.num_minutes()
    );

    // let x: Result<Alert> = github.get("/orgs/pypi-data/secret-scanning/alerts", None::<&()>).await;
    let mut url: Option<Url> = Some(
        "https://api.github.com/orgs/pypi-data/secret-scanning/alerts?per_page=100&sort=created"
            .parse()?,
    );
    let mut alerts_summary: HashMap<String, DetectedAlerts> = HashMap::new();

    let mut idx = 0;

    let mut all_alerts = vec![];

    while let Some(page_url) = url {
        let page = github.get_page::<Alert>(&Some(page_url)).await?.unwrap();
        all_alerts.extend(page.items);

        url = page.next;
        idx += 1;
        println!("Fetched alert page {}", idx);
    }

    for alert in all_alerts {
        let alerts = alerts_summary
            .entry(alert.secret_type_display_name)
            .or_default();
        alerts.count += 1;
        alerts.secrets.insert(alert.secret);
    }

    // let futures = all_alerts.into_iter().map(|alert| async {
    //     let locations = github
    //         .get_page::<AlertLocation>(&Some(alert.locations_url.clone()))
    //         .await?
    //         .unwrap();
    //     Ok::<_, anyhow::Error>((alert, locations))
    // });

    // let mut stream = futures::stream::iter(futures).buffer_unordered(20);

    // Hitting API request limits!
    // println!("Fetching secret locations");
    // while let Some(v) = stream.next().await {
    //     let (alert, locations) = v?;
    //     let alerts = alerts_summary
    //         .entry(alert.secret_type_display_name)
    //         .or_default();
    //     alerts.count += 1;
    //     alerts.secrets.insert(alert.secret);
    //     alerts.unique_packages.extend(
    //         locations
    //             .items
    //             .into_iter()
    //             .map(|location| location.details.path.split('/').next().unwrap().to_string()),
    //     );
    // }

    let unique_alerts = alerts_summary.len();
    let total_alerts: u32 = alerts_summary.values().map(|a| a.count).sum();
    let table: Vec<_> = alerts_summary
        .into_iter()
        .map(|(a, c)| (a, c.count, c.secrets.len(), c.unique_packages.len()))
        .sorted_by(|a, b| a.1.cmp(&b.1).reverse())
        .collect();

    #[derive(serde::Serialize)]
    struct Context {
        unique_alerts: usize,
        total_alerts: u32,
        table: Vec<(String, u32, usize, usize)>,
    }

    let mut tt = TinyTemplate::new();
    tt.add_template("readme", include_str!("template.md"))?;
    let output = tt
        .render(
            "readme",
            &Context {
                unique_alerts,
                total_alerts,
                table,
            },
        )
        .unwrap();

    println!("{output}");

    std::fs::write("README.md", output)?;

    Ok(())
}
//
// async fn get_all_secrets(github: &Octocrab) -> anyhow::Result<Vec<String>> {
//     let names = vec![
//         "pypi-code-3",
//         "pypi-code-4",
//         "pypi-code-11",
//         "pypi-code-12",
//         "pypi-code-13",
//         "pypi-code-14",
//         "pypi-code-15",
//         "pypi-code-16",
//         "pypi-code-17",
//         "pypi-code-18",
//         "pypi-code-1",
//         "pypi-code-2",
//         "pypi-code-5",
//         "pypi-code-6",
//         "pypi-code-7",
//         "pypi-code-8",
//         "pypi-code-9",
//         "pypi-code-10",
//         "pypi-code-19",
//         "pypi-code-20",
//         "pypi-code-21",
//         "pypi-code-22",
//         "pypi-code-23",
//         "pypi-code-24",
//         "pypi-code-25",
//         "pypi-code-26",
//         "pypi-code-27",
//         "pypi-code-3",
//         "pypi-code-4",
//         "pypi-code-11",
//         "pypi-code-12",
//         "pypi-code-13",
//         "pypi-code-14",
//         "pypi-code-15",
//         "pypi-code-16",
//         "pypi-code-17",
//         "pypi-code-18",
//         "pypi-code-1",
//         "pypi-code-2",
//         "pypi-code-5",
//         "pypi-code-6",
//         "pypi-code-7",
//         "pypi-code-8",
//         "pypi-code-9",
//         "pypi-code-10",
//         "pypi-code-19",
//         "pypi-code-20",
//         "pypi-code-21",
//         "pypi-code-22",
//         "pypi-code-23",
//         "pypi-code-24",
//         "pypi-code-25",
//         "pypi-code-26",
//         "pypi-code-27",
//     ];
//
//     return Ok(names.into_iter().map(|n| n.to_string()).collect());
//
//     let mut page: u32 = 0;
//     let mut names = vec![];
//     loop {
//         let repos = github
//             .orgs("pypi-data")
//             .list_repos()
//             .page(page)
//             .sort(Sort::FullName)
//             .send()
//             .await?;
//         names.extend(
//             repos
//                 .items
//                 .into_iter()
//                 .filter(|r| r.name.starts_with("pypi-code-"))
//                 .map(|r| r.name),
//         );
//         if repos.next.is_none() {
//             break;
//         }
//         page += 1;
//     }
//
//     Ok(names)
// }
