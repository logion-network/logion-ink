#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable)]
#[openbrush::contract]
pub mod logion_psp34 {
    use logion_contract_pkg::impls::logion::*;
    use logion_contract_pkg::impls::types as logion;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LogionPsp34 {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        logion: logion::Data,
    }

    impl Logion for LogionPsp34 {}

    impl LogionPsp34 {
        #[ink(constructor)]
        pub fn new(nonce: String, collection_loc_id: u128, cert_host: String) -> Self {
            let mut instance = Self::default();
            instance.logion.init(nonce, collection_loc_id, cert_host);
            instance
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        use openbrush::contracts::psp34::Id;

        const COLLECTION_LOC_ID: u128 = 334801581596596632473758891935041239976;

        #[ink::test]
        fn it_gets_collection_loc_id() {
            let contract = new_contract();
            assert_eq!(contract.get_collection_loc_id(), COLLECTION_LOC_ID);
        }

        #[ink::test]
        fn it_gets_item_id() {
            let contract = new_contract();
            let token_id = Id::U8(123);
            // echo -n "202210131727:U8(123)" | sha256sum => 71e240ef02a005a86b70ad321687f95afb8c6519122962d9144e943cd04311cf
            let expected_hash = "0x71e240ef02a005a86b70ad321687f95afb8c6519122962d9144e943cd04311cf".to_string();
            assert_eq!(contract.get_item_id(token_id), expected_hash);
        }

        #[ink::test]
        fn it_gets_certificate_url() {
            let contract = new_contract();
            let token_id = Id::U8(123);
            assert_eq!(contract.get_certificate_url(token_id), "https://certificate.logion.network/public/certificate/334801581596596632473758891935041239976/0x71e240ef02a005a86b70ad321687f95afb8c6519122962d9144e943cd04311cf".to_string());
        }

        fn new_contract() -> LogionPsp34 {
            LogionPsp34::new(
                "202210131727".to_string(),
                COLLECTION_LOC_ID,
                "certificate.logion.network".to_string(),
            )
        }
    }
}