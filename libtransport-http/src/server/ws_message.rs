
use bytes::Bytes;

use bincode::serialize;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InternodeMessage {
    SyncRequest,
    SyncResponse,
}

impl Into<Bytes> for InternodeMessage {
    fn into(self) -> Bytes {
        match serialize(&self) {
            Ok(encoded) => Bytes::from(encoded),
            Err(e) => {
                error!("{}", e);
                Bytes::from(vec![])
            }
        }
    }
}
