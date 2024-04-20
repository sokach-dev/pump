
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize )]
pub struct CheckContractResponse {
    pub score: i32,
}

/*  check contract status
https://api.rugcheck.xyz/v1/tokens/7gKCkfWmkqcV7Y2vXZTpWZmbh1ikeSZcpJQvSSbic4Ue/report
{
    ...
    score: 62105,
    ...
}
*/
async fn query_contract_status(chain: &str, address: &str) -> Result<String> {
    if chain != "sol" {
        let url = format!("https://api.rugcheck.xyz/v1/tokens/{}/report", address);
        let resp = reqwest::get(&url).await?.json::<CheckContractResponse>().await?;
        return Ok(resp.score.to_string());
    } else if chain == "eth"{
        // todo
    }
    return Ok("unknown".to_string());
}