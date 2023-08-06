use once_cell::sync::Lazy;

use std::collections::HashMap;

pub mod req;
pub mod resp;
pub mod stream;
pub mod util;

#[cfg(test)]
mod test {

    use std::pin::pin;

    use chrono::{Duration, Local};
    use futures::StreamExt;
    use itertools::Itertools;
    use serde_json::{json, Value};

    use crate::resp::SearchCreateResponse;
    use crate::stream::stream_search;

    use super::req::*;
    use super::*;

    #[tokio::test]
    pub async fn try_it() {
        let req = CreateRequest {
            query_legs: vec![QueryLeg {
                origin_place_id: "LAX".into(),
                destination_place_id: "BER".into(),
                date: Local::now().date_naive() + Duration::days(2),
            }],
            ..Default::default()
        };

        // println!("{}", serde_json::to_string_pretty(&req).unwrap());

        let mut str = pin!(stream_search(req));

        while let Some(resp) = str.next().await {
            let content = resp.unwrap();

            for itin in content.sorted(resp::LiveSortingOption::Cheapest).take(5) {
                println!("{}", content.format_itinerary(&itin));
            }

            println!("====more?...");
        }
    }
}
