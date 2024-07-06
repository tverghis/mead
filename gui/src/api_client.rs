use schema::responses::ProgInfoResponse;
use ureq::serde::de::DeserializeOwned;

pub type ApiResult<T> = Result<T, ureq::Error>;

pub fn get_prog_infos() -> ApiResult<Vec<ProgInfoResponse>> {
    api_request("/prog_info")
}

fn api_request<T: DeserializeOwned>(path: &str) -> ApiResult<T> {
    let url = format!("http://localhost:3000{}", path);
    let resp: T = ureq::get(&url).call()?.into_json()?;

    Ok(resp)
}
