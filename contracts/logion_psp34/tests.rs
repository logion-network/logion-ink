use ink::env::DefaultEnvironment;
use ink::env::test::set_callee;
use ink::primitives::{AccountId};
use openbrush::contracts::psp34::Id;

use crate::logion_psp34::LogionPsp34;

const COLLECTION_LOC_ID: &str = "334801581596596632473758891935041239976";

#[test]
fn it_gets_collection_loc_id() {
    set_up();
    let contract = new_contract();
    assert_eq!(contract.get_collection_loc_id(), COLLECTION_LOC_ID.to_string());
}

#[test]
fn it_gets_item_id_seed() {
    set_up();
    let contract = new_contract();
    assert_eq!(contract.get_item_id_seed(Id::U8(123)), "202210131727:U8(123)".to_string());
    assert_eq!(contract.get_item_id_seed(Id::U16(123)), "202210131727:U16(123)".to_string());
    assert_eq!(contract.get_item_id_seed(Id::U32(123)), "202210131727:U32(123)".to_string());
    assert_eq!(contract.get_item_id_seed(Id::U64(123)), "202210131727:U64(123)".to_string());
    assert_eq!(contract.get_item_id_seed(Id::U128(123)), "202210131727:U128(123)".to_string());

    let token_id: Vec<u8> = "abcd".into();
    // echo -n "abcd" | sha256sum => 88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589
    assert_eq!(contract.get_item_id_seed(Id::Bytes(token_id)), "202210131727:Bytes(0x88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589)".to_string());
}

#[test]
fn it_gets_item_id() {
    set_up();
    let contract = new_contract();
    let token_id = Id::U8(123);
    // echo -n "202210131727:U8(123)" | sha256sum => 71e240ef02a005a86b70ad321687f95afb8c6519122962d9144e943cd04311cf
    let expected_hash = "0x71e240ef02a005a86b70ad321687f95afb8c6519122962d9144e943cd04311cf".to_string();
    assert_eq!(contract.get_item_id(token_id), expected_hash);
}

#[test]
fn it_gets_certificate_url() {
    set_up();
    let contract = new_contract();
    let token_id = Id::U8(123);
    assert_eq!(contract.get_certificate_url(token_id), "https://certificate.logion.network/public/certificate/334801581596596632473758891935041239976/0x71e240ef02a005a86b70ad321687f95afb8c6519122962d9144e943cd04311cf".to_string());
}

fn new_contract() -> LogionPsp34 {
    LogionPsp34::new(
        "202210131727".to_string(),
        COLLECTION_LOC_ID.to_string(),
        "certificate.logion.network".to_string(),
    )
}

fn set_up() {
    let callee: AccountId = AccountId::from([0xFFu8; 32]);
    set_callee::<DefaultEnvironment>(callee);
}
