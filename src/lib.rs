pub mod base64 {
    pub fn encode(data: &str) -> String {
        const ENCODE_TABLE: [char; 64] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
        ];

        let in_len: usize = data.len();
        let out_len: usize = 4 * ((in_len + 2) / 3);
        let mut ret: String = String::with_capacity(out_len);

        let data_nth = |n| data.chars().nth(n).unwrap() as usize;
        let data_nth_back = |n| data.chars().nth_back(n).unwrap() as usize;

        for i in (0..in_len - 2).step_by(3) {
            let b1 = data_nth(i);
            let b2 = data_nth(i + 1);
            let b3 = data_nth(i + 2);

            let concat_bytes: usize = (b1 << 16) | (b2 << 8) | (b3 << 0);

            ret.push(ENCODE_TABLE[(concat_bytes >> 18) & 0b0011_1111]);
            ret.push(ENCODE_TABLE[(concat_bytes >> 12) & 0b0011_1111]);
            ret.push(ENCODE_TABLE[(concat_bytes >> 6) & 0b0011_1111]);
            ret.push(ENCODE_TABLE[(concat_bytes >> 0) & 0b0011_1111]);
        }

        // Last bytes
        match in_len % 3 {
            0 => (),
            1 => {
                let b1 = data_nth_back(0) as usize;
                let concat_bytes: usize = (b1 << 16) | (0 << 8) | (0 << 0);
                ret.push(ENCODE_TABLE[(concat_bytes >> 18) & 0b0011_1111]);
                ret.push(ENCODE_TABLE[(concat_bytes >> 12) & 0b0011_1111]);
                ret.push('=');
                ret.push('=');
            }
            2 => {
                let b1 = data_nth_back(1) as usize;
                let b2 = data_nth_back(0) as usize;
                let concat_bytes: usize = (b1 << 16) | (b2 << 8) | (0 << 0);
                ret.push(ENCODE_TABLE[(concat_bytes >> 18) & 0b0011_1111]);
                ret.push(ENCODE_TABLE[(concat_bytes >> 12) & 0b0011_1111]);
                ret.push(ENCODE_TABLE[(concat_bytes >> 6) & 0b0011_1111]);
                ret.push('=');
            }
            _ => panic!("Impossible mod result!"),
        }

        ret
    }

    pub fn decode(encoded_data: &str) -> Result<String, &'static str> {
        const DECODE_TABLE: [usize; 128] = [
            0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64,
            0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64,
            0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64,
            0x64, 0x3E, 0x64, 0x64, 0x64, 0x3F, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B,
            0x3C, 0x3D, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x00, 0x01, 0x02, 0x03, 0x04,
            0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
            0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x1A,
            0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
            0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x64, 0x64, 0x64,
            0x64, 0x64,
        ];

        let in_len = encoded_data.len();
        if in_len % 4 != 0 {
            return Err("Encoded bytes are not a multiple of 4!");
        }

        let data_nth = |n| encoded_data.chars().nth(n).unwrap() as usize;
        let data_nth_back = |n| encoded_data.chars().nth_back(n).unwrap() as usize;

        let mut out_len = in_len / 4 * 3;
        if data_nth_back(0) == '=' as usize {
            out_len -= 1;
        }
        if data_nth_back(1) == '=' as usize {
            out_len -= 1;
        }
        let mut ret = String::with_capacity(out_len);

        for i in (0..in_len).step_by(4) {
            let c1 = DECODE_TABLE[data_nth(i)];
            let c2 = DECODE_TABLE[data_nth(i + 1)];
            let c3 = DECODE_TABLE[data_nth(i + 2)];
            let c4 = DECODE_TABLE[data_nth(i + 3)];

            let concat_bytes = ((c1 << 18) | (c2 << 12) | (c3 << 6) | (c4 << 0)) as u32;
            
            ret.push(u8::try_from((concat_bytes >> 16) & 0b1111_1111).unwrap() as char);
            ret.push(u8::try_from((concat_bytes >> 8) & 0b1111_1111).unwrap() as char);
            ret.push(u8::try_from((concat_bytes >> 0) & 0b1111_1111).unwrap() as char);
        }

        ret.truncate(out_len);
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_errors() {
        let encoded_data = "QmVzdCB0ZXN0IGV2ZXI";
        let decoded = base64::decode(encoded_data);
        assert!(decoded.is_err());
        if let Err(e) = decoded {
            println!("{e}");
        }
    }

    #[test]
    fn encode_decode() {
        let data = "I should have coded this in C++";
        let encoded = base64::encode(data);
        let decoded = base64::decode(&encoded);
        assert!(decoded.is_ok());
        let decoded = decoded.unwrap();
        assert!(decoded == data);
        println!("data=\"{data}\", encoded=\"{encoded}\", decoded=\"{decoded}\"");
        
        let data = "Because I suck at Rust!";
        let encoded = base64::encode(data);
        let decoded = base64::decode(&encoded);
        assert!(decoded.is_ok());
        let decoded = decoded.unwrap();
        assert!(decoded == data);
        println!("data=\"{data}\", encoded=\"{encoded}\", decoded=\"{decoded}\"");

        let data = "But you also suck!";
        let encoded = base64::encode(data);
        let decoded = base64::decode(&encoded);
        assert!(decoded.is_ok());
        let decoded = decoded.unwrap();
        assert!(decoded == data);
        // let decoded = String::from("");
        println!("data=\"{data}\", encoded=\"{encoded}\", decoded=\"{decoded}\"");
    }
}
