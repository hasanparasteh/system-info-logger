use crate::structs::SystemInfo;

pub async fn insert_into_influx(
    info: SystemInfo,
    bucket: &str,
    tag: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let token = std::env::var("INFLUX_TOKEN").expect("CAN'T READ TOKEN");

    match client
        .post(url_builder(bucket, tag))
        .body(json_to_influx(info, "example"))
        .header("Authorization", String::from("Token ") + &token)
        .send()
        .await
    {
        Ok(it) => Ok(it),
        Err(err) => Err(err),
    }
}

pub fn url_builder(bucket: &str, organization: &str) -> String {
    let api_host = std::env::var("INFLUX_API").expect("CAN'T READ API_HOST");

    format!(
        "http://{}/api/v2/write?bucket={}&org={}",
        api_host, bucket, organization
    )
}

pub fn json_to_influx(info: SystemInfo, tag: &str) -> String {
    let tags = [
        tag.to_string(),
        format!("machine_id={}", "\"W11PC\""),
        format!("region=simin"),
    ];

    let fields = [
        format!("mem={}", info.used_memory),
        format!("free={}", info.free_memory),
        format!("cpu={}", info.cpu_usage),
        format!("swap={}", info.used_swap),
    ];

    tags.join(",") + " " + &fields.join(",") + " " + &info.get_time_stamp().to_string()
}
