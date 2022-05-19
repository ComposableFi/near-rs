use crate::types::LightClientBlockView;

use ibc::core::ics02_client::{
    client_type::ClientType,
    header::{AnyHeader, Header},
};

impl Header for LightClientBlockView {
    fn client_type(&self) -> ClientType {
        ClientType::Tendermint
    }

    fn height(&self) -> ibc::Height {
        ibc::Height {
            // NOTE: casting epoch id into a u64 in order to fit with the IBC spec.
            revision_number: u64::from_be_bytes(
                self.inner_lite.epoch_id.0[..8].try_into().unwrap(),
            ),
            revision_height: self.inner_lite.height,
        }
    }

    fn timestamp(&self) -> ibc::timestamp::Timestamp {
        ibc::timestamp::Timestamp::from_nanoseconds(self.inner_lite.timestamp_nanosec).unwrap()
    }

    fn wrap_any(self) -> AnyHeader {
        // TODO: create our own header
        todo!()
    }
}
