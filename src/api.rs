use crate::config;
use crate::data;
use anyhow::Result;
use base64::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Token {
    access_token: String,
}

pub async fn get_token(client: config::Client) -> Result<String> {
    let client_id = client.id;
    let client_secret = client.secret;

    let url = "https://api.ramp.com/developer/v1/token";
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(
        "Authorization",
        format!(
            "Basic {}",
            BASE64_URL_SAFE.encode(format!("{}:{}", client_id, client_secret).as_bytes()),
        )
        .parse()
        .unwrap(),
    );
    let body = "grant_type=client_credentials&scope=transactions:read reimbursements:read";

    let response = client.post(url).headers(headers).body(body).send().await?;

    let response_body = response.json::<Token>().await?;

    Ok(response_body.access_token)
}

pub async fn get_transactions(token: &String) -> Result<Vec<data::Transaction>> {
    let url = "https://api.ramp.com/developer/v1/transactions";

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    let params = [
        ("state", "CLEARED"),
        ("has_no_sync_commits", "true"),
        ("sync_ready", "true"),
    ];

    let response = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<data::Response>()
        .await?
        .data;

    let data: Vec<data::Transaction> = response.into_iter().map(data::Transaction::from).collect();

    Ok(data)
}

pub async fn get_reimbursements(token: &String) -> Result<Vec<data::ReimbursementRow>> {
    let url = "https://api.ramp.com/developer/v1/reimbursements";

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let params = [("has_no_sync_commits", "true"), ("sync_ready", "true")];

    let response = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    if response.status() != 200 {
        return Err(anyhow::anyhow!(
            "Error getting reimbursements: {}",
            response.status()
        ));
    }

    let response = response.json::<data::ReimbursementResponse>().await?.data;

    let data: Vec<data::ReimbursementRow> = response
        .into_iter()
        .map(data::ReimbursementRow::from)
        .collect();

    Ok(data)
}
