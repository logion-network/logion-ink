use core::fmt::{Display, Error, Formatter};
use ink::primitives::Hash;
use ink_env::hash::{HashOutput, Sha2x256};
use ink_prelude::format;
use ink::prelude::vec::Vec;
use openbrush::contracts::psp34::Id;
use openbrush::traits::{Storage};
use openbrush::traits::String;

use crate::impls::types::Data;
pub use crate::traits::logion::LogionContract;

pub struct LogionHash {
    value: Hash
}

impl Display for LogionHash {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "0x")?;
        for i in self.value.as_ref() {
            write!(f, "{:02x?}", i)?;
        }
        Ok(())
    }
}

pub trait Internal {

    fn hash(&self, input: &Vec<u8>) -> LogionHash;

    fn get_item_id_seed(&self, token_id: Id) -> String;
}

#[openbrush::trait_definition]
pub trait Logion: Storage<Data>
{

    #[ink(message)]
    fn get_collection_loc_id(&self) -> u128 {
        self.data::<Data>().collection_loc_id
    }

    #[ink(message)]
    fn get_item_id(&self, token_id: Id) -> String {
        let item_id_seed = self.get_item_id_seed(token_id);
        let item_id = self.hash(&item_id_seed.as_bytes().to_vec());
        format!("{}", item_id)
    }

    #[ink(message)]
    fn get_certificate_url(&self, token_id: Id) -> String {
        let cert_host = &self.data::<Data>().cert_host;
        let collection_loc_id = self.data::<Data>().collection_loc_id;
        let item_id = self.get_item_id(token_id);
        format!("https://{}/public/certificate/{}/{}", cert_host, collection_loc_id, item_id)
    }

}

impl<T> Internal for T
    where
        T: Storage<Data>
{
    fn hash(&self, input: &Vec<u8>) -> LogionHash {
        let mut output = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
        ink_env::hash_bytes::<Sha2x256>(input, &mut output);
        LogionHash { value: output.into() }
    }

    fn get_item_id_seed(&self, token_id: Id) -> String {
        let nonce = &self.data::<Data>().nonce;

        let id_type = String::from(match token_id {
            Id::U8(_) => "U8",
            Id::U16(_) => "U16",
            Id::U32(_) => "U32",
            Id::U64(_) => "U64",
            Id::U128(_) => "U128",
            Id::Bytes(_) => "Bytes",
        });

        let id_value: String = match token_id {
            Id::U8(value) => format!("{}", value),
            Id::U16(value) => format!("{}", value),
            Id::U32(value) => format!("{}", value),
            Id::U64(value) => format!("{}", value),
            Id::U128(value) => format!("{}", value),
            Id::Bytes(value) => format!("{}", self.hash(&value)),
        };

        format!("{}:{}({})", nonce, id_type, id_value)
    }

}