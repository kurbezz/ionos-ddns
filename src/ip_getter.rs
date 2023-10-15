use serde::Deserialize;


#[derive(Deserialize)]
pub struct IpData {
    pub ip: String,
}


pub async fn get_ip_string() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::Client::new()
        .get("https://api.ipify.org?format=json")
        .send()
        .await?
        .error_for_status()?;

    match response.json::<IpData>().await {
        Ok(v) => Ok(v.ip),
        Err(err) => Err(Box::new(err)),
    }
}