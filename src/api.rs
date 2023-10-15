use crate::config::CONFIG;

use serde::{Deserialize, Serialize};


pub fn get_api_key() -> String {
    format!("{}.{}", CONFIG.ionos_public_prefix, CONFIG.ionos_secret)
}


#[derive(Deserialize)]
pub struct Zone {
    pub name: String,
    pub id: String
}


pub async fn get_zones() -> Result<Vec<Zone>, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::Client::new()
        .get("https://api.hosting.ionos.com/dns/v1/zones")
        .header("User-Agent", "Reqwest")
        .header("X-API-KEY", get_api_key())
        .send()
        .await?
        .error_for_status()?;

    match response.json::<Vec<Zone>>().await {
        Ok(v) => Ok(v),
        Err(err) => Err(Box::new(err))
    }
}


#[derive(Deserialize)]
pub struct Record {
    pub name: String,
    pub content: String,
    pub id: String
}


#[derive(Deserialize)]
pub struct ZoneDetail {
    pub name: String,
    pub id: String,
    pub records: Vec<Record>
}


pub async fn get_zone_info(
    zone_id: String,
    record_name: String
) -> Result<ZoneDetail, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::Client::new()
        .get(format!("https://api.hosting.ionos.com/dns/v1/zones/{}", zone_id))
        .query(&[("recordName", record_name)])
        .header("User-Agent", "Reqwest")
        .header("X-API-KEY", get_api_key())
        .send()
        .await?
        .error_for_status()?;

    match response.json::<ZoneDetail>().await {
        Ok(v) => Ok(v),
        Err(err) => Err(Box::new(err))
    }
}


#[derive(Serialize)]
pub struct CreateRecord {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub content: String,
    pub ttl: u32,
    pub prio: u32,
    pub disabled: bool
}


pub async fn create_record(
    zone_id: String,
    create_data: CreateRecord
) -> Result<Vec<Record>, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::Client::new()
        .post(format!("https://api.hosting.ionos.com/dns/v1/zones/{}/records", zone_id))
        .header("User-Agent", "Reqwest")
        .header("X-API-KEY", get_api_key())
        .json(&vec![create_data])
        .send()
        .await?
        .error_for_status()?;

    match response.json::<Vec<Record>>().await {
        Ok(v) => Ok(v),
        Err(err) => Err(Box::new(err))
    }
}


#[derive(Serialize)]
pub struct UpdateRecord {
    pub content: String,
    pub ttl: u32,
    pub prio: u32,
    pub disabled: bool
}


pub async fn update_record(
    zone_id: String,
    record_id: String,
    update_data: UpdateRecord
) -> Result<Record, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::Client::new()
        .put(format!("https://api.hosting.ionos.com/dns/v1/zones/{}/records/{}", zone_id, record_id))
        .header("User-Agent", "Reqwest")
        .header("X-API-KEY", get_api_key())
        .json(&update_data)
        .send()
        .await?
        .error_for_status()?;

    match response.json::<Record>().await {
        Ok(v) => Ok(v),
        Err(err) => Err(Box::new(err))
    }
}
