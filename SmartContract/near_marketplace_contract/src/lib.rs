use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Marketplace {
    listed_products: UnorderedMap<String, Product>,
}

#[near_bindgen]
impl Marketplace {
    #[init]
    pub fn init() -> Self {
        Self {
            listed_products: UnorderedMap::new(b"product".to_vec()),
        }
    }

    pub fn set_product(&mut self, payload: Payload) {
        let product: Product = Product::from_payload(payload);
        self.listed_products.insert(&product.id, &product);
    }

    pub fn get_product(&self, id: &String) -> Option<Product> {
        self.listed_products.get(id)
    }

    pub fn get_products(&self) -> Vec<Product> {
        self.listed_products.values_as_vector().to_vec()
    }
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Product {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String,
    owner: AccountId,
    sold: u32,
}

#[near_bindgen]
impl Product {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            id: payload.id,
            name: payload.name,
            description: payload.description,
            image: payload.image,
            location: payload.location,
            price: payload.price,
            owner: env::signer_account_id(),
            sold: 0,
        }
    }

    pub fn increment_sold_amount(&mut self) {
        self.sold = self.sold + 1;
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String,
}
