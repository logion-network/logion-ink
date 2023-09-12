use openbrush::contracts::psp34::Id;
use openbrush::traits::String;

#[openbrush::wrapper]
pub type LogionContractRef = dyn LogionContract;

#[openbrush::trait_definition]
pub trait LogionContract {

    #[ink(message)]
    fn get_collection_loc_id(&self) -> u128;

    #[ink(message)]
    fn get_item_id(&self, token_id: Id) -> String;

    #[ink(message)]
    fn get_certificate_url(&self, token_id: Id) -> String;
}