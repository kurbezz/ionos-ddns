mod config;
mod api;
mod ip_getter;

use tokio::time::{self, Duration};

use api::{get_zones, Zone, get_zone_info, Record, create_record, CreateRecord, update_record, UpdateRecord};
use config::CONFIG;
use ip_getter::get_ip_string;


async fn update(current_ip: String) {
    let zones = get_zones()
        .await
        .unwrap_or_else(
            |err| panic!("Can't get zones! Err: {}", err)
        );

    let zone_id = zones
        .iter()
        .filter(|zone| zone.name.eq(&CONFIG.zone_name))
        .collect::<Vec<&Zone>>()
        .get(0)
        .unwrap_or_else(|| panic!("Can't find zone with name `{}`", CONFIG.zone_name))
        .id
        .clone();

    let zone_detail = get_zone_info(zone_id.clone(), CONFIG.record_name.join(","))
        .await
        .unwrap_or_else(|err| panic!("Can't get zone info! Err: {}", err));

    for record_name in CONFIG.record_name.iter() {
        let record = zone_detail.records
            .iter()
            .filter(|record| record.name.eq(record_name))
            .collect::<Vec<&Record>>()
            .get(0)
            .cloned();

        match record {
            Some(v) => {
                if !v.content.eq(&current_ip.clone()) {
                    update_record(
                        zone_id.clone(),
                        v.id.clone(),
                        UpdateRecord {
                            content: current_ip.clone(),
                            ttl: CONFIG.check_interval_seconds.try_into().unwrap(),
                            prio: 0,
                            disabled: false,
                        }
                    )
                        .await
                        .unwrap_or_else(|err| panic!("Can't update record! Err: {}", err));
                }
            },
            None => {
                create_record(
                    zone_id.clone(),
                    CreateRecord {
                        name: record_name.to_string(),
                        type_: "A".to_string(),
                        content: current_ip.clone(),
                        ttl: CONFIG.check_interval_seconds.try_into().unwrap(),
                        prio: 0,
                        disabled: false,
                    }
                )
                    .await
                    .unwrap_or_else(|err| panic!("Can't create record! Err: {}", err));
            },
        }
    }
}


#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(CONFIG.check_interval_seconds));

    let mut prev_ip: String = "".to_string();

    loop {
        let current_ip = get_ip_string()
            .await
            .unwrap_or_else(|err| panic!("Can't get current ip! Err: {}", err));

        if !current_ip.eq(&prev_ip) {
            prev_ip = current_ip.clone();

            update(current_ip).await;
            println!("Updated!");
        }

        interval.tick().await;
    }
}
