// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::{Error, Result};
use crate::utils::pretty_print;
use clap::Parser;
use radix_engine_toolkit::request::{
    DeriveVirtualAccountAddressHandler, DeriveVirtualAccountAddressRequest, Handler,
};
use radix_engine_toolkit::utils::checked_copy_u8_slice;
use scrypto::prelude::{EcdsaSecp256k1PublicKey, EddsaEd25519PublicKey};

#[derive(Parser, Debug)]
/// Derives virtual account address of the given public key on the given network
pub struct VirtualAccountAddress {
    /// The public key to derive the virtual component address for.
    #[clap(short, long)]
    public_key: String,

    /// The network id to derive the known addresses for.
    #[clap(short, long)]
    network_id: u8,
}

impl VirtualAccountAddress {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        let public_key_bytes = hex::decode(&self.public_key)?;
        let public_key = match public_key_bytes.len() {
            EcdsaSecp256k1PublicKey::LENGTH => {
                Ok(EcdsaSecp256k1PublicKey(checked_copy_u8_slice(&public_key_bytes)?).into())
            }
            EddsaEd25519PublicKey::LENGTH => {
                Ok(EddsaEd25519PublicKey(checked_copy_u8_slice(&public_key_bytes)?).into())
            }
            _ => Err(Error::InvalidPublicKey),
        }?;

        let request = DeriveVirtualAccountAddressRequest {
            public_key,
            network_id: self.network_id,
        };
        let response = DeriveVirtualAccountAddressHandler::fulfill(request)?;
        pretty_print(&response, out)
    }
}