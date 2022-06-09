use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};
use near_sdk::serde::{Serialize, Deserialize};
use srd::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize,BorshSerialize,Serialize,Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Msg(String);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Packages {
    pub sub_package: String,
    pub channels: String,
    pub num_channels: u64,
    pub num_sub_packages: u64,
}

#[near_bindgen]
impl  Packages{
    fn new_packages(
        sub_package: String,
        channels: String,
        num_channels: u64,
        num_sub_packages: u64,
    ) -> Packages {
        Self { 
            sub_package: sub_package, 
            channels: channels, 
            num_channels: num_channels, 
            num_sub_packages: num_sub_packages,
        }
    }
}

#[near_bindgen]





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
