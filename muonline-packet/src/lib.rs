#[macro_use]
extern crate lazy_static;
extern crate byteorder;
extern crate smallvec;

#[cfg(feature = "serialize")]
pub use serialize::{PacketType, PacketEncodable, PacketDecodable};
pub use kind::PacketKind;
pub use packet::Packet;
pub use crypto::Crypto;

mod kind;
mod packet;

#[cfg(feature = "serialize")]
pub mod serialize;
pub mod crypto;

/// Default XOR cipher extracted from the client.
pub static XOR_CIPHER: [u8; 32] = [
    0xE7, 0x6D, 0x3A, 0x89, 0xBC, 0xB2, 0x9F, 0x73,
    0x23, 0xA8, 0xFE, 0xB6, 0x49, 0x5D, 0x39, 0x5D,
    0x8A, 0xCB, 0x63, 0x8D, 0xEA, 0x7D, 0x2B, 0x5F,
    0xC3, 0xB1, 0xE9, 0x83, 0x29, 0x51, 0xE8, 0x56,
];

#[cfg(test)]
mod tests {
    use super::*;

    const ENCRYPTED: [u8; 6] = [0xC1, 0x06, 0xA9, 0x20, 0x9C, 0x2F];
    const DECRYPTED: [u8; 6] = [0xC1, 0x06, 0xA9, 0x00, 0x00, 0x01];

    #[test]
    fn xor_decoding() {
        let (packet, _, _) = Packet::from_bytes_ex(&ENCRYPTED, Some(&XOR_CIPHER), None).unwrap();

        assert_eq!(packet.kind(), PacketKind::C1);
        assert_eq!(packet.len(), DECRYPTED.len());
        assert_eq!(packet.code(), 0xA9);
        assert_eq!(packet.data(), &DECRYPTED[packet.kind().offset()..]);
    }

    #[test]
    fn xor_encoding() {
        let mut packet = Packet::new(PacketKind::C1, 0xA9);
        packet.append(&DECRYPTED[PacketKind::C1.offset()..]);

        assert_eq!(packet.kind(), PacketKind::C1);
        assert_eq!(packet.len(), ENCRYPTED.len());
        assert_eq!(packet.code(), 0xA9);

        let data = packet.to_bytes_ex(Some(&XOR_CIPHER), None);
        assert_eq!(&data, &ENCRYPTED);
    }

    #[test]
    fn decrypt_encrypt() {
        let (packet, _, _) = Packet::from_bytes_ex(&ENCRYPTED, Some(&XOR_CIPHER), None).unwrap();
        assert_eq!(packet.to_bytes(), &DECRYPTED);
        assert_eq!(packet.to_bytes_ex(Some(&XOR_CIPHER), None), &ENCRYPTED);
    }

    #[test]
    fn packet_c2() {
        let bytes = [0xC2, 0x00, 0x0B, 0xF4, 0x06, 0x00, 0x01, 0x00, 0x00, 0x05, 0x77];
        let packet = Packet::from_bytes(&bytes).unwrap();

        assert_eq!(packet.kind(), PacketKind::C2);
        assert_eq!(packet.len(), 0x0B);
        assert_eq!(packet.code(), 0xF4);
        assert_eq!(packet.data().first(), Some(&0x06));
    }

    #[test]
    fn packet_c1_to_c3() {
        let bytes = [0xC1, 0x06, 0xF4, 0x03, 0x00, 0x00];
        let packet = Packet::from_bytes(&bytes).unwrap();

        let encoded = packet.to_bytes_ex(None, Some((&crypto::CLIENT, 0)));
        assert_eq!(encoded, [0xC3, 0x0D, 0xE3, 0xB3, 0x53, 0x9A, 0x4F, 0xC8, 0x32, 0x7D, 0x04, 0x37, 0x0F]);
    }

    #[test]
    fn packet_c3_to_c1() {
        let bytes = [0xC3, 0x0D, 0xE3, 0xB3, 0x53, 0x9A, 0x4F, 0xC8, 0x32, 0x7D, 0x04, 0x37, 0x0F, 0x00];
        let (packet, len, cc) = Packet::from_bytes_ex(&bytes, None, Some(&crypto::CLIENT)).unwrap();

        assert_eq!(len, bytes.len() - 1);
        assert_eq!(cc.unwrap(), 0);

        let decoded = packet.to_bytes();
        assert_eq!(decoded, [0xC1, 0x06, 0xF4, 0x03, 0x00, 0x00]);
    }
}
