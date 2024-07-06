use schema::responses::ProgInfoResponse;

use crate::api_client;

pub struct State {
    pub prog_infos: Vec<ProgInfoResponse>,
}

impl State {
    pub fn new() -> Self {
        Self { prog_infos: vec![] }
    }

    pub fn new_from_api() -> Self {
        let prog_infos = api_client::get_prog_infos().unwrap();
        Self { prog_infos }
    }
}
