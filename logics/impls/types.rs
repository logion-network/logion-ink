use openbrush::traits::String;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {

    pub nonce: String,
    pub collection_loc_id: u128,
    pub cert_host: String
}

impl Data {

    pub fn init(& mut self, nonce: String, collection_loc_id: u128, cert_host: String) {
        self.nonce = nonce;
        self.collection_loc_id = collection_loc_id;
        self.cert_host = cert_host;
    }

}