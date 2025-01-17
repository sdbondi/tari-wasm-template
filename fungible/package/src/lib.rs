//  Copyright 2022. The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use tari_template_lib::prelude::*;

// TODO: we should be able to put FungibleCoin inside the mod
#[derive(Debug, Clone, Encode, Decode)]
// #[resource(Fungible)]
pub struct FungibleCoin {}

// TODO: macro can implement this
impl ResourceDefinition for FungibleCoin {
    fn resource_type() -> ResourceType {
        ResourceType::Fungible
    }
}

#[template]
mod account_template {
    use super::*;

    pub struct FungibleAccount {
        vault: Vault<FungibleCoin>,
        // owner_badge: AccessBadge,
    }

    impl FungibleAccount {
        pub fn initial_mint(initial_supply: Amount) -> Self {
            // let owner_badge = ResourceBuilder::badge().allow_actions(ResourceAction::all()).build();

            // Because of `derive(Fungible)` we could go `let bucket = FungibleCoin::mint(initial_supply, ...)`
            let coins = ResourceBuilder::fungible()
                .with_token_symbol("🪙")
                .initial_supply(initial_supply)
                // .access_rules(rules![
                //     mint => require(owner_badge),
                //     burn => deny,
                //     update_metadata => require(owner_badge),
                //     update_access_rules => deny,
                // ])
                .build_bucket();

            Self {
                vault: Vault::from_bucket(coins),
                // owner_badge,
            }
        }

        // #[access_rules(allow_all)]
        pub fn new_account(&self) -> Self {
            // let owner_badge = ResourceBuilder::badge().allow_actions(ResourceAction::all()).build();
            let address = self.vault.resource_address();

            Self {
                vault: Vault::new_empty(address),
                // owner_badge,
            }
        }

        // #[access_rules(allow_all)]
        pub fn symbol(&self) -> String {
            // self.vault.metadata().get(known_metadata_keys::TOKEN_SYMBOL).unwrap()
            todo!()
        }

        // #[access_rules(requires(owner_badge))]
        pub fn balance(&self) -> Amount {
            self.vault.balance()
        }

        // #[access_rules(requires(owner_badge))]
        // pub fn transfer_to(&mut self, account: ComponentAddress, amount: Amount) {
        //     let bucket = self.withdraw(amount);
        //     system().component(&account).invoke("deposit", args![bucket]);
        // }

        // #[access_rules(requires(owner_badge))]
        pub fn withdraw(&mut self, amount: Amount) -> Bucket<FungibleCoin> {
            self.vault.withdraw(amount)
        }

        // #[access_rules(allow_all)]
        pub fn deposit(&mut self, bucket: Bucket<FungibleCoin>) {
            self.vault.deposit(bucket);
        }
    }
}
