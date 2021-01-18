include!(concat!(env!("OUT_DIR"), "/krpc.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let input = ConnectionRequest {
            r#type: Some(connection_request::Type::Rpc),
            client_name: Some("test".to_string()),
            client_identifier: None,
        };
        let mut encoded = vec![];
        input.encode(&mut encoded).unwrap();
        let output = [
            // type
            1 << 3,
            0,
            // client_name
            2 << 3 | 2,
            input.client_name.as_ref().map(|x| x.len()).unwrap_or(0) as u8,
            b't',
            b'e',
            b's',
            b't',
        ];
        assert_eq!(encoded, output);

        let len = output.len();
        let mut packet = vec![];
        input.encode_with_len(&mut packet).unwrap();
        let packet_output = [
            len as u8,
            // type
            1 << 3,
            0,
            // client_name
            2 << 3 | 2,
            input.client_name.as_ref().map(|x| x.len()).unwrap_or(0) as u8,
            b't',
            b'e',
            b's',
            b't',
        ];
        assert_eq!(packet, packet_output);
    }

    #[test]
    fn decode() {
        let input = [
            // type
            1 << 3,
            0,
            // client_name
            2 << 3 | 2,
            4,
            b't',
            b'e',
            b's',
            b't',
        ];
        let decoded = ConnectionRequest::decode(&input[..]).unwrap();
        let output = ConnectionRequest {
            r#type: Some(connection_request::Type::Rpc),
            client_name: Some("test".to_string()),
            client_identifier: None,
        };
        assert_eq!(decoded, output);

        let len = input.len();
        let packet_input = [
            len as u8,
            // type
            1 << 3,
            0,
            // client_name
            2 << 3 | 2,
            4,
            b't',
            b'e',
            b's',
            b't',
        ];
        let packet =
            ConnectionRequest::decode_with_len(&packet_input[..]).unwrap();
        assert_eq!(packet, output);
    }
}
