use openbrush::contracts::psp34::Id;
use crate::impls::logion::Internal;

struct LogionInternal {}
impl Internal for LogionInternal {}

#[test]
fn it_hashes_using_sha256_and_format_with_0x() {
    // echo -n "abcd" | sha256sum => 88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589
    let logion_internal = LogionInternal {};
    let hash = logion_internal.hash(&"abcd".as_bytes().to_vec());
    assert_eq!(hash.to_string(), "0x88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589");
}

#[test]
fn it_gets_item_id_seed() {
    let logion_internal = LogionInternal {};
    let nonce = &"202210131727".to_string();
    assert_eq!(logion_internal.get_item_id_seed(nonce, Id::U8(123)), "202210131727:U8(123)".to_string());
    assert_eq!(logion_internal.get_item_id_seed(nonce, Id::U16(123)), "202210131727:U16(123)".to_string());
    assert_eq!(logion_internal.get_item_id_seed(nonce, Id::U32(123)), "202210131727:U32(123)".to_string());
    assert_eq!(logion_internal.get_item_id_seed(nonce, Id::U64(123)), "202210131727:U64(123)".to_string());
    assert_eq!(logion_internal.get_item_id_seed(nonce, Id::U128(123)), "202210131727:U128(123)".to_string());

    let token_id: Vec<u8> = "abcd".into();
    assert_eq!(logion_internal.get_item_id_seed(nonce, Id::Bytes(token_id)), "202210131727:Bytes(0x88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589)".to_string());
}
