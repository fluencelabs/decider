use libp2p_identity::{ParseError, PeerId};

/// Static prefix of the PeerId. Protobuf encoding + multihash::identity + length and so on.
pub const PEER_ID_PREFIX: &[u8] = &[0, 36, 8, 1, 18, 32];

pub fn parse_peer_id(bytes: Vec<u8>) -> Result<PeerId, ParseError> {
    let peer_id = &[PEER_ID_PREFIX, &bytes].concat();

    PeerId::from_bytes(&peer_id)
}

pub fn serialize_peer_id(peer_id: PeerId) -> Vec<u8> {
    let peer_id = peer_id.to_bytes();
    let peer_id = peer_id.into_iter().skip(PEER_ID_PREFIX.len()).collect();
    peer_id
}
