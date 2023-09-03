#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod auction_contract {
    use ink_prelude::vec::Vec;
    use ink_storage::collections::HashMap as StorageHashMap;
    use ink_storage::traits::{PackedLayout, SpreadLayout};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    pub struct Item {
        owner: AccountId,
        description: String,
        current_bid: Balance,
        highest_bidder: Option<AccountId>,
    }

    #[ink(storage)]
    pub struct AuctionContract {
        owner: AccountId,
        items: StorageHashMap<u32, Item>,
        next_item_id: u32,
    }

    impl AuctionContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                items: Default::default(),
                next_item_id: 0,
            }
        }

        pub fn list_item(&mut self, description: String) {
            let item = Item {
                owner: self.env().caller(),
                description,
                current_bid: 0,
                highest_bidder: None,
            };
            self.items.insert(self.next_item_id, item);
            self.next_item_id += 1;
        }

        pub fn bid(&mut self, item_id: u32, bid_amount: Balance) {
            let caller = self.env().caller();
            let mut item = self.items.get_mut(&item_id).expect("Item not found");
            if bid_amount > item.current_bid {
                item.current_bid = bid_amount;
                item.highest_bidder = Some(caller);
            }
        }

        pub fn get_item(&self, item_id: u32) -> Option<&Item> {
            self.items.get(&item_id)
        }

        // Add functions for editing and stopping listings here

        // Add other query functions here
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::test::DefaultAccounts;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut contract = AuctionContract::new(Default::default());
            let accounts = DefaultAccounts::new().expect("Cannot get accounts");

            // Test listing an item
            contract.list_item(String::from("Item 1"));
            let item = contract.get_item(0).expect("Item not found");
            assert_eq!(item.owner, accounts.alice);
            assert_eq!(item.description, "Item 1");

            // Test bidding on an item
            contract.bid(0, 100);
            let item = contract.get_item(0).expect("Item not found");
            assert_eq!(item.current_bid, 100);
        }
    }
}
