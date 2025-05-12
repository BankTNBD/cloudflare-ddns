use std::error::Error;
use config::Config;
use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Settings {
    zone_id: String,
    record_name: String,
    auth_email: String,
    auth_key: String,
}

#[derive(Debug, Deserialize)]
struct DnsRecord {
    id: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct RecordResponse {
    result: Vec<DnsRecord>,
}

#[derive(Serialize)]
struct UpdateRecord<'a> {
    #[serde(rename = "type")]
    record_type: &'a str,
    name: &'a str,
    content: &'a str,
    ttl: u32,
    proxied: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load config from file
    let settings: Settings = Config::builder()
        .add_source(config::File::with_name("/etc/cloudflare-ddns/config.toml")) // will read config.toml
        .build()?
        .try_deserialize()?;

    let zone_id = settings.zone_id;
    let record_name = settings.record_name;
    let auth_email = settings.auth_email;
    let auth_key = settings.auth_key;

    // 1. Get public IP
    let client1 = Client::builder().use_rustls_tls().build()?;
    let ip = client1.get("https://api.ipify.org").send().await?.text().await?;
    println!("Current public IP: {}", ip);

    let mut headers = HeaderMap::new();
    headers.insert("X-Auth-Email", HeaderValue::from_str(&auth_email)?);
    headers.insert("X-Auth-Key", HeaderValue::from_str(&auth_key)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::builder().default_headers(headers).use_rustls_tls().build()?;

    // 2. Get DNS record
    let dns_url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}",
        zone_id, record_name
    );

    let dns_raw = client.get(&dns_url).send().await?.text().await?;
    let dns_resp: RecordResponse = serde_json::from_str(&dns_raw)?;

    if dns_resp.result.is_empty() {
        eprintln!("No DNS record found for {}", record_name);
        return Ok(());
    }

    let record_id = &dns_resp.result[0].id;
    let current_ip = &dns_resp.result[0].content;

    // 3. Compare and update
    if &ip != current_ip {
        println!("IP changed: {} -> {}", current_ip, ip);
        let update = UpdateRecord {
            record_type: "A",
            name: &record_name,
            content: &ip,
            ttl: 300,
            proxied: false,
        };

        let update_url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        );
        let resp = client.put(&update_url).json(&update).send().await?;

        println!("Updated DNS: {:?}", resp.status());
    } else {
        println!("IP unchanged, no update needed.");
    }

    Ok(())
}