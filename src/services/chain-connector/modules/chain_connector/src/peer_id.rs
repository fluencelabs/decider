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

#[cfg(test)]
mod tests {
    use crate::hex::decode_hex;
    use crate::peer_id::{parse_peer_id, serialize_peer_id};

    #[test]
    fn parse_peerid() {
        let hex = "0x7cd8a742d826c3183e817d44d6c54140bddaf9c0545d144165cab1ad9fbe167d";

        let bytes = decode_hex(hex).expect("parse hex");
        let peer_id = parse_peer_id(bytes).expect("parse peerid");
        assert_eq!(
            peer_id.to_string(),
            "12D3KooWJDiLFLmWstcFpAofWkYJzuvwuquNTQQkB9xzKjRyqqFJ"
        );
        let bytes = serialize_peer_id(peer_id);
        let peer_id_hex = hex::encode(bytes);
        assert_eq!(peer_id_hex.as_str(), hex.trim_start_matches("0x"));
    }
}
