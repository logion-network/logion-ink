#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[cfg(test)]
pub mod tests;

#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Metadata)]
#[openbrush::contract]
pub mod logion_psp34 {
    use core::fmt::{Display, Error, Formatter};
    use ink_env::format;
    use ink_env::hash::Sha2x256;
    use openbrush::traits::Storage;

    const ID_FOR_LOGION_METADATA: &Id = &Id::U8(1);
    const NONCE_KEY: &'static str = "nonce";
    const COLLECTION_LOC_ID_KEY: &'static str = "collection_loc_id";
    const CERT_HOST_KEY: &'static str = "cert_host";

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

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum LogionError {
        /// Returned if some Logion Metadata is missing.
        MissingLogionMetadata,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LogionPsp34 {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl LogionPsp34 {
        #[ink(constructor)]
        pub fn new(nonce: String, collection_loc_id: String, cert_host: String) -> Self {
            let mut instance = Self::default();

            metadata::Internal::_set_attribute(&mut instance, ID_FOR_LOGION_METADATA.clone(), String::from(NONCE_KEY), nonce);
            metadata::Internal::_set_attribute(&mut instance, ID_FOR_LOGION_METADATA.clone(), String::from(COLLECTION_LOC_ID_KEY), collection_loc_id);
            metadata::Internal::_set_attribute(&mut instance, ID_FOR_LOGION_METADATA.clone(), String::from(CERT_HOST_KEY), cert_host);

            instance
        }

        #[ink(message)]
        pub fn get_collection_loc_id(&self) -> Result<String, LogionError> {
            self.get_logion_metadata(COLLECTION_LOC_ID_KEY)
        }

        #[ink(message)]
        pub fn get_item_id(&self, token_id: Id) -> Result<String, LogionError> {
            let item_id_seed = self.get_item_id_seed(token_id)?;
            let item_id = self.hash(&item_id_seed.as_bytes().to_vec());
            Ok(format!("{}", item_id))
        }

        pub(crate) fn get_item_id_seed(&self, token_id: Id) -> Result<String, LogionError> {
            let nonce = self.get_logion_metadata(NONCE_KEY)?;

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

            Ok(format!("{}:{}({})", nonce, id_type, id_value))
        }

        #[ink(message)]
        pub fn get_certificate_url(&self, token_id: Id) -> Result<String, LogionError> {
            let cert_host = self.get_logion_metadata(CERT_HOST_KEY)?;
            let collection_loc_id = self.get_collection_loc_id()?;
            let item_id = self.get_item_id(token_id)?;
            let url = format!("https://{}/public/certificate/{}/{}", cert_host, collection_loc_id, item_id);
            Ok(url)
        }

        fn hash(&self, input: &Vec<u8>) -> LogionHash {
            let value = self.env().hash_bytes::<Sha2x256>(input);
            LogionHash { value: value.into() }
        }

        fn get_logion_metadata(&self, key: &'static str) -> Result<String, LogionError> {
            let attribute = PSP34MetadataImpl::get_attribute(self, ID_FOR_LOGION_METADATA.clone(), String::from(key));
            match attribute {
                Some(value) => Ok(value),
                _ => Err(LogionError::MissingLogionMetadata)
            }
        }
    }
}