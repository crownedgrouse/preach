use encode_064::to_phi_64;
use encode_128::to_phi_128;
use encode_192::to_phi_192;
use encode_256::to_phi_256;

pub fn to_phi(byte: u8, rand: u8) -> Vec<u8> {
    let v: Vec<_> = if byte < 64 {
        to_phi_64(byte, rand)
    } else if byte < 128 {
        to_phi_128(byte, rand)
    } else if byte < 192 {
        to_phi_192(byte, rand)
    } else {
        to_phi_256(byte, rand)
    };
    if byte != 0 && v == vec![0, 0, 0] {
            panic!("Error vec![0, 0, 0] returnd for byte={:?} rand={:?}", byte, rand) ;
    }
    return v;
}
