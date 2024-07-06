use schema::responses::ProgInfoResponse;

pub fn get_prog_infos() -> Result<Vec<ProgInfoResponse>, ureq::Error> {
    let resp: Vec<ProgInfoResponse> = ureq::get("http://localhost:3000/prog_info")
        .call()?
        .into_json()?;

    Ok(resp)
}
