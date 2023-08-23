use dotenv::var;
use local_ip_address::local_ipv6;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let uri = "https://api.gandi.net/v5/livedns/domains/".to_owned()
        + &var("FQDN")? + "/records/" + &var("NAME")? + "/AAAA";

    reqwest::Client::new()
        .put(uri)
        .header("Authorization", "Apikey ".to_owned() + &var("APIKEY")?)
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"rrset_values": ["{}"]}}"#, local_ipv6()?))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
