use libbpf_rs::query::ProgramInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgInfoResponse {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub id: u32,
    pub tag: String,
}

impl From<ProgramInfo> for ProgInfoResponse {
    fn from(info: ProgramInfo) -> Self {
        // Convert the bytes in .tag to a hex string
        let tag = info.tag.0.map(|byte| format!("{:02x?}", byte)).join("");

        Self {
            name: info.name.to_string_lossy().to_string(),
            // TODO: This should probably be something other than the `Debug` impl, but `Dispay` is no longer implemented for `ProgramType`.
            ty: format!("{:?}", info.ty),
            id: info.id,
            tag,
        }
    }
}
