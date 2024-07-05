use libbpf_rs::query::ProgramInfo;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ProgInfoResponse {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

impl From<ProgramInfo> for ProgInfoResponse {
    fn from(info: ProgramInfo) -> Self {
        Self {
            name: info.name.to_string_lossy().to_string(),
            ty: info.ty.to_string(),
        }
    }
}
