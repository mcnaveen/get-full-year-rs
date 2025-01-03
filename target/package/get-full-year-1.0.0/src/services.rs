use crate::dto::YearResponseDTO;
use crate::errors::YearFetchingError;
use reqwest::Client;
use std::error::Error;

pub async fn get_full_year(is_enterprise: bool) -> Result<YearResponseDTO, Box<dyn Error>> {
    let client = Client::new();
    let temporal_data_endpoint = "https://getfullyear.com/api/year";

    println!(
        "🚀 Initiating year acquisition process {}...",
        if is_enterprise { "in ENTERPRISE mode" } else { "in STANDARD mode" }
    );

    let response = client.get(temporal_data_endpoint).send().await?;
    
    if !response.status().is_success() {
        return Err(Box::new(YearFetchingError::new(&format!(
            "HTTP Status: {}",
            response.status()
        ))));
    }

    let data: YearResponseDTO = response.json().await?;

    if !is_enterprise {
        if let Some(sponsored_by) = &data.sponsored_by {
            println!("✨ Sponsored by our magnificent partner: {}", sponsored_by);
        }
    }

    println!("✅ Year acquisition completed successfully");
    Ok(data)
}
