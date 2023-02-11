use core::slice;

//RFC 4648
const TABLE: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];
const PADDING: u8 = b'=';

pub fn base64_encode(buffer: &[u8]) -> String {
    let in_len = buffer.len();
    let out_len = (in_len + 2) / 3 * 4;
    let mut output = Vec::with_capacity(out_len);

    let slices = in_len / 3;

    for i in 0..slices {
        encode_slice(&buffer[i * 3..=i * 3 + 2], &mut output);
    }
    // padding
    let remains = in_len % 3;
    match remains {
        0 => {}
        1 => {
            let a = buffer[slices * 3];
            let char1 = a >> 2;
            let char2 = (a << 4) & 0b00111111;
            output.push(TABLE[char1 as usize]);
            output.push(TABLE[char2 as usize]);
            output.push(PADDING);
            output.push(PADDING);
        }
        2 => {
            let a = buffer[slices * 3];
            let b = buffer[slices * 3 + 1];
            let char1 = a >> 2;
            let char2 = (a << 4 | b >> 4) & 0b00111111;
            let char3 = b << 2 & 0b00111111;
            output.push(TABLE[char1 as usize]);
            output.push(TABLE[char2 as usize]);
            output.push(TABLE[char3 as usize]);
            output.push(PADDING);
        }
        _ => unreachable!(),
    }
    assert_eq!(out_len, output.len());
    // will not panic if the algorithm is correct.
    String::from_utf8(output).expect("Invalid UTF-8")
}

enum Error{
    InvalidLength,
    UnexpectedCharacter(usize),
}

/// TODO add error types
pub fn base64_decode(s: &str) -> Vec<u8> {
    let has_padding = s.ends_with('=');
    let s = s.as_bytes();
    let in_len = s.len();
    let mut output = Vec::with_capacity(in_len * 3);
    let slices = in_len / 4;
    let unpadding_slices = if has_padding { slices - 1 } else { slices };
    for i in 0..unpadding_slices {
        decode_slice(&s[i * 4..=i * 4 + 3], &mut output);
    }
    // handle padding
    if has_padding {
        let slice = &s[in_len - 4..in_len];
        let c1 = char_to_byte(slice[0]); //would never be '='
        let c2 = char_to_byte(slice[1]); // would never be '='
        output.push(c1 << 2 | c2 >> 4);
        if slice[2] != PADDING {
            let c3 = char_to_byte(slice[2]); // could be '='

            output.push((c2 & 0b1111) << 4 | c3 >> 2);
            if slice[3] != PADDING {
                let c4 = char_to_byte(slice[3]); // counld be '='
    
                output.push(c3 << 6 | c4);
            }
        }
        // if slice[2] is padding slice[3] will always be padding
        
    }
    output
}

/// panic if slice length isn't 3
fn encode_slice(slice: &[u8], output: &mut Vec<u8>) {
    let a = slice[0];
    let b = slice[1];
    let c = slice[2];
    let char1 = a >> 2;
    let char2 = (a << 4 | b >> 4) & 0b00111111;
    let char3 = (b << 2 | c >> 6) & 0b00111111;
    let char4 = c & 0b00111111;
    output.push(TABLE[char1 as usize]);
    output.push(TABLE[char2 as usize]);
    output.push(TABLE[char3 as usize]);
    output.push(TABLE[char4 as usize]);
    // TODO i don't know why but using ascii is slower
    // str.push(u8_to_char(char1));
    // str.push(u8_to_char(char2));
    // str.push(u8_to_char(char3));
    // str.push(u8_to_char(char4));
}

/// panic if slice length isn't 4
fn decode_slice(bytes: &[u8], output: &mut Vec<u8>) {
    let c1 = char_to_byte(bytes[0]);
    let c2 = char_to_byte(bytes[1]);
    let c3 = char_to_byte(bytes[2]);
    let c4 = char_to_byte(bytes[3]);
    let a = c1 << 2 | c2 >> 4;
    let b = (c2 & 0b1111) << 4 | c3 >> 2;
    let c = c3 << 6 | c4;
    output.push(a);
    output.push(b);
    output.push(c);
}
fn decode_padding(bytes: &[u8], output: &mut Vec<u8>) {}

// fn u8_to_char(num:u8)->char{
//     match num {
//         0..=25 => (num + 0x41).into(),
//         26..=51 => (num + 71).into(),
//         52..=61 => (num - 4).into(),
//         62 => '+',
//         63 => '/',
//         _=>unreachable!()
//     }
// }
/// decode base64 char to byte
fn char_to_byte(i: u8) -> u8 {
    match i {
        b'A'..=b'Z' => i as u8 - 0x41,
        b'a'..=b'z' => i as u8 - 71,
        b'0'..=b'9' => i as u8 + 4,
        b'+' => 62,
        b'/' => 63,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_encode() {
        assert_eq!(base64_encode(b"abc"), "YWJj");
        assert_eq!(base64_encode(b"gsgcbirmsdkgmer"), "Z3NnY2Jpcm1zZGtnbWVy")
    }
    #[test]
    fn padding_encode() {
        assert_eq!(base64_encode(b"abcd"), "YWJjZA==");
        assert_eq!(base64_encode(b"abcde"), "YWJjZGU=");
        assert_eq!(base64_encode(b"sageskjkbvnmiksjgtkgeskjgkgesGEKSAGNSGMSJKGKMVLKSJKGNKSNGLAJLKGHKSNKBAL;AJKKLGHSKNGALJHKNBZ.MOSGM.A.[91328I"),"c2FnZXNramtidm5taWtzamd0a2dlc2tqZ2tnZXNHRUtTQUdOU0dNU0pLR0tNVkxLU0pLR05LU05HTEFKTEtHSEtTTktCQUw7QUpLS0xHSFNLTkdBTEpIS05CWi5NT1NHTS5BLls5MTMyOEk=")
    }

    #[test]
    fn basic_decode() {
        assert_eq!(base64_decode("YWJj"), b"abc");
        assert_eq!(base64_decode("Z3NnY2Jpcm1zZGtnbWVy"), b"gsgcbirmsdkgmer");
    }
    #[test]
    fn padding_decode() {
        assert_eq!(base64_decode("YWJjZA=="), b"abcd");
        assert_eq!(base64_decode("YWJjZGU="), b"abcde");
        assert_eq!(base64_decode("c2FnZXNramtidm5taWtzamd0a2dlc2tqZ2tnZXNHRUtTQUdOU0dNU0pLR0tNVkxLU0pLR05LU05HTEFKTEtHSEtTTktCQUw7QUpLS0xHSFNLTkdBTEpIS05CWi5NT1NHTS5BLls5MTMyOEk="),b"sageskjkbvnmiksjgtkgeskjgkgesGEKSAGNSGMSJKGKMVLKSJKGNKSNGLAJLKGHKSNKBAL;AJKKLGHSKNGALJHKNBZ.MOSGM.A.[91328I")
    }
}
