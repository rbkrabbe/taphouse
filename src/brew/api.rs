use reqwest::Client;
use crate::brew::types::{CaskInfo, FormulaInfo};

pub async fn fetch_formulae(client: &Client) -> Result<Vec<FormulaInfo>, String> {
    client
        .get("https://formulae.brew.sh/api/formula.json")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<FormulaInfo>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_casks(client: &Client) -> Result<Vec<CaskInfo>, String> {
    client
        .get("https://formulae.brew.sh/api/cask.json")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<CaskInfo>>()
        .await
        .map_err(|e| e.to_string())
}
