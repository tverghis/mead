use schema::responses::ProgInfoResponse;

pub struct State {
    pub prog_infos: Vec<ProgInfoResponse>,
}

impl State {
    pub fn new() -> Self {
        Self { prog_infos: vec![] }
    }
}
