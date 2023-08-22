use dotenv::var;
use local_ip_address::local_ipv6;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!(
        "https://api.gandi.net/v5/livedns/domains/{}/records/{}/AAAA",
        var("FQDN").unwrap(),
        var("NAME").unwrap(),
    );

    reqwest::Client::new()
        .put(uri)
        .header("Authorization", "Apikey ".to_owned() + &dotenv::var("APIKEY").unwrap())
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"rrset_values": ["{}"]}}"#, local_ipv6().unwrap()))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
