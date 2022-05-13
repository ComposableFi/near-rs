#![cfg_attr(not(feature = "std"), no_std)]
use ibc::core::{
    ics02_client::{client_def::ClientDef, context::ClientReader},
    ics24_host::identifier::ClientId,
};

#[derive(Debug, Clone)]
struct NearLiteClient {}

impl ClientDef for NearLiteClient {
    type Header;

    type ClientState;

    type ConsensusState;

    fn check_header_and_update_state(
        &self,
        ctx: &dyn ClientReader,
        client_id: ClientId,
        client_state: Self::ClientState,
        header: Self::Header,
    ) -> Result<(Self::ClientState, Self::ConsensusState), Error> {
        todo!()
    }

    fn verify_upgrade_and_update_state(
        &self,
        client_state: &Self::ClientState,
        consensus_state: &Self::ConsensusState,
        proof_upgrade_client: MerkleProof,
        proof_upgrade_consensus_state: MerkleProof,
    ) -> Result<(Self::ClientState, Self::ConsensusState), Error> {
        todo!()
    }

    fn verify_client_consensus_state(
        &self,
        client_state: &Self::ClientState,
        height: ibc::Height,
        prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        consensus_height: ibc::Height,
        expected_consensus_state: &ibc::core::ics02_client::client_consensus::AnyConsensusState,
    ) -> Result<(), Error> {
        todo!()
    }

    fn verify_connection_state(
        &self,
        client_state: &Self::ClientState,
        height: ibc::Height,
        prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        connection_id: &ibc::core::ics24_host::identifier::ConnectionId,
        expected_connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
    ) -> Result<(), Error> {
        todo!()
    }

    fn verify_channel_state(
        &self,
        client_state: &Self::ClientState,
        height: ibc::Height,
        prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        port_id: &ibc::core::ics24_host::identifier::PortId,
        channel_id: &ibc::core::ics24_host::identifier::ChannelId,
        expected_channel_end: &ibc::core::ics04_channel::channel::ChannelEnd,
    ) -> Result<(), Error> {
        todo!()
    }

    fn verify_client_full_state(
        &self,
        client_state: &Self::ClientState,
        height: ibc::Height,
        prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        expected_client_state: &ibc::core::ics02_client::client_state::AnyClientState,
    ) -> Result<(), Error> {
        todo!()
    }

    fn verify_packet_data(
        &self,
        ctx: &dyn ibc::core::ics04_channel::context::ChannelReader,
        client_state: &Self::ClientState,
        height: ibc::Height,
        connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        port_id: &ibc::core::ics24_host::identifier::PortId,
        channel_id: &ibc::core::ics24_host::identifier::ChannelId,
        sequence: ibc::core::ics04_channel::packet::Sequence,
        commitment: alloc::string::String,
    ) -> Result<(), Error> {
        todo!()
    }

    fn verify_packet_acknowledgement(
        &self,
        ctx: &dyn ibc::core::ics04_channel::context::ChannelReader,
        client_state: &Self::ClientState,
        height: ibc::Height,
        connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        port_id: &ibc::core::ics24_host::identifier::PortId,
        channel_id: &ibc::core::ics24_host::identifier::ChannelId,
        sequence: ibc::core::ics04_channel::packet::Sequence,
        ack: ibc::core::ics04_channel::msgs::acknowledgement::Acknowledgement,
    ) -> Result<(), Error> {
        todo!()
    }

    fn verify_next_sequence_recv(
        &self,
        ctx: &dyn ibc::core::ics04_channel::context::ChannelReader,
        client_state: &Self::ClientState,
        height: ibc::Height,
        connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        port_id: &ibc::core::ics24_host::identifier::PortId,
        channel_id: &ibc::core::ics24_host::identifier::ChannelId,
        sequence: ibc::core::ics04_channel::packet::Sequence,
    ) -> Result<(), Error> {
        todo!()
    }

    fn verify_packet_receipt_absence(
        &self,
        ctx: &dyn ibc::core::ics04_channel::context::ChannelReader,
        client_state: &Self::ClientState,
        height: ibc::Height,
        connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        port_id: &ibc::core::ics24_host::identifier::PortId,
        channel_id: &ibc::core::ics24_host::identifier::ChannelId,
        sequence: ibc::core::ics04_channel::packet::Sequence,
    ) -> Result<(), Error> {
        todo!()
    }
}
