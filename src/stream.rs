use std::{collections::HashMap, sync::Mutex};

use futures::{Stream, StreamExt, TryStreamExt};
use itertools::Either;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

use crate::{
    req::CreateRequest,
    resp::{
        FlightsLiveSearchContent, ResultAction, ResultStatus, SearchCreateResponse,
        SearchPollResponse,
    },
};

static SKYSCANNER_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .default_headers(
            (&[(
                "x-api-key",
                std::env::var("SKYSCANNER_API_KEY").expect("needs SKYSCANNER_API_KEY"),
            )]
            .into_iter()
            .map(|x| (x.0.to_owned(), x.1))
            .collect::<HashMap<_, _>>())
                .try_into()
                .expect("bad headers"),
        )
        .build()
        .expect("bad client")
});

pub async fn create_search(req: CreateRequest) -> anyhow::Result<SearchCreateResponse> {
    let resp = SKYSCANNER_CLIENT
        .post("https://partners.api.skyscanner.net/apiservices/v3/flights/live/search/create")
        .json(&json!({"query": req}))
        .send()
        .await?;

    let resp = if true {
        let resp = resp.json::<Value>().await.unwrap();

        // println!("{}", serde_json::to_string_pretty(&resp).unwrap());

        std::fs::write(
            "skyscanner.json",
            serde_json::to_string_pretty(&resp).unwrap(),
        )?;

        serde_json::from_value::<SearchCreateResponse>(resp)?
    } else {
        resp.json::<SearchCreateResponse>().await?
    };

    Ok(resp)
}

pub async fn poll_search(token: &str) -> anyhow::Result<SearchPollResponse> {
    let resp = SKYSCANNER_CLIENT
        .post(&format!(
            "https://partners.api.skyscanner.net/apiservices/v3/flights/live/search/poll/{}",
            token
        ))
        .send()
        .await?;

    let resp = if true {
        let resp = resp.json::<Value>().await.unwrap();

        // println!("{}", serde_json::to_string_pretty(&resp).unwrap());

        std::fs::write(
            "skyscanner.json",
            serde_json::to_string_pretty(&resp).unwrap(),
        )?;

        serde_json::from_value::<SearchPollResponse>(resp)?
    } else {
        resp.json::<SearchPollResponse>().await?
    };

    Ok(resp)
}

pub fn stream_search(
    req: CreateRequest,
) -> impl Stream<Item = anyhow::Result<FlightsLiveSearchContent>> {
    futures::stream::try_unfold(Either::Left(req), move |req_or_token| async move {
        match req_or_token {
            Either::Left(req) => {
                let resp = create_search(req).await?;

                // println!("{:?} {:?}", resp.status, resp.action);

                let token = resp.session_token;

                let content = if resp.action == ResultAction::ResultActionReplaced {
                    Some(resp.content)
                } else {
                    None
                };

                Ok(Some((content, Either::Right((resp.status, token)))))
            }
            Either::Right::<_, (_, String)>((status, token)) => {
                if status != ResultStatus::ResultStatusIncomplete {
                    return Ok(None);
                }

                let resp = poll_search(&token).await?;

                // println!("{:?} {:?}", resp.status, resp.action);

                let content = if resp.action == ResultAction::ResultActionReplaced {
                    Some(resp.content)
                } else {
                    None
                };

                Ok(Some((
                    content,
                    Either::Right((resp.status, resp.session_token)),
                )))
            }
        }
    })
    .try_filter_map(|v| async move { Ok(v) })
}
