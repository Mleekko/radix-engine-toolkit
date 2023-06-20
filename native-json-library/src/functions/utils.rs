// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::prelude::*;
use schemars::JsonSchema;
use scrypto::prelude::*;
use serde::{Deserialize, Serialize};

pub type KnownAddressesInput = SerializableU8;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct KnownAddressesOutput {
    pub resource_addresses: ResourceAddresses,
    pub package_addresses: PackageAddresses,
    pub component_addresses: ComponentAddresses,
}

pub struct KnownAddress;
impl<'f> Function<'f> for KnownAddress {
    type Input = KnownAddressesInput;
    type Output = KnownAddressesOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *input;

        let resource_addresses = construct_addresses! {
            ResourceAddresses,
            network_id,
            [
                xrd,
                secp256k1_signature_virtual_badge,
                ed25519_signature_virtual_badge,
                package_of_direct_caller_virtual_badge,
                global_caller_virtual_badge,
                system_transaction_badge,
                package_owner_badge,
                validator_owner_badge,
                account_owner_badge,
                identity_owner_badge,
            ]
        };
        let package_addresses = construct_addresses! {
            PackageAddresses,
            network_id,
            [
                package_package,
                resource_package,
                account_package,
                identity_package,
                consensus_manager_package,
                access_controller_package,
                pool_package,
                transaction_processor_package,
                metadata_module_package,
                royalty_module_package,
                access_rules_module_package,
                genesis_helper_package,
                faucet_package,
            ]
        };
        let component_addresses = construct_addresses! {
            ComponentAddresses,
            network_id,
            [
                consensus_manager,
                genesis_helper,
                faucet,
            ]
        };

        Ok(Self::Output {
            component_addresses,
            package_addresses,
            resource_addresses,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourceAddresses {
    pub xrd: SerializableNodeId,
    pub secp256k1_signature_virtual_badge: SerializableNodeId,
    pub ed25519_signature_virtual_badge: SerializableNodeId,
    pub package_of_direct_caller_virtual_badge: SerializableNodeId,
    pub global_caller_virtual_badge: SerializableNodeId,
    pub system_transaction_badge: SerializableNodeId,
    pub package_owner_badge: SerializableNodeId,
    pub validator_owner_badge: SerializableNodeId,
    pub account_owner_badge: SerializableNodeId,
    pub identity_owner_badge: SerializableNodeId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PackageAddresses {
    pub package_package: SerializableNodeId,
    pub resource_package: SerializableNodeId,
    pub account_package: SerializableNodeId,
    pub identity_package: SerializableNodeId,
    pub consensus_manager_package: SerializableNodeId,
    pub access_controller_package: SerializableNodeId,
    pub pool_package: SerializableNodeId,
    pub transaction_processor_package: SerializableNodeId,
    pub metadata_module_package: SerializableNodeId,
    pub royalty_module_package: SerializableNodeId,
    pub access_rules_module_package: SerializableNodeId,
    pub genesis_helper_package: SerializableNodeId,
    pub faucet_package: SerializableNodeId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ComponentAddresses {
    pub consensus_manager: SerializableNodeId,
    pub genesis_helper: SerializableNodeId,
    pub faucet: SerializableNodeId,
}

macro_rules! construct_addresses {
    ($struct_ident: expr, $network_id: expr, [$($field: ident),* $(,)?]) => {
        paste::paste! {
            $struct_ident {
                $(
                    $field: SerializableNodeId::from_global_address([< $field:upper >], $network_id),
                )*
            }
        }
    };
}
use construct_addresses;
