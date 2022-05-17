use ibc::{
    clients::ics07_tendermint::header::Header as TendermintHeader,
    core::ics02_client::{
        client_type::ClientType,
        header::{AnyHeader, Header},
    },
    timestamp::Timestamp,
};

#[derive(Debug, Clone)]
pub struct NearHeader {
    timestamp: Timestamp,
    inner: TendermintHeader,
}

impl Header for NearHeader {
    fn client_type(&self) -> ClientType {
        ClientType::Tendermint
    }

    fn height(&self) -> ibc::Height {
        self.inner.trusted_height
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn wrap_any(self) -> AnyHeader {
        AnyHeader::Tendermint(self.inner)
    }
}
