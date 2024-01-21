use crate::Error;
use awc::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct HetznerRecord {
    id: String,
    zone_id: String,
    name: String,
    value: String,
    r#type: String,
}

impl HetznerRecord {
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }
}

pub async fn get_record(record_id: &str) -> Result<HetznerRecord, Error> {
    let client = Client::new();

    let mut res = client
        .get(format!(
            "https://dns.hetzner.com/api/v1/records/{}",
            record_id
        ))
        .insert_header(("Auth-API-Token", env::var("HETZNER_TOKEN").unwrap()))
        .send()
        .await?;

    let record_container = res
        .json::<serde_json::Value>()
        .await?
        .get("record")
        .unwrap()
        .to_owned();

    let record = serde_json::from_value(record_container)?;

    Ok(record)
}

pub async fn update_record(record: &HetznerRecord) -> Result<(), Error> {
    let client = Client::new();

    let res = client
        .put(format!(
            "https://dns.hetzner.com/api/v1/records/{}",
            record.id
        ))
        .insert_header(("Auth-API-Token", env::var("HETZNER_TOKEN").unwrap()))
        .send_json(record)
        .await?;

    if res.status().is_success() == false {
        log::error!("Error updating record: {:?}", res);
    }

    Ok(())
}
