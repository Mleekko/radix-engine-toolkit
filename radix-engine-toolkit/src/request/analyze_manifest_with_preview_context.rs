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

use std::collections::BTreeSet;

use crate::error::{Error, Result};
use crate::model::address::{EntityAddress, NetworkAwarePackageAddress};
use crate::model::address::{NetworkAwareComponentAddress, NetworkAwareResourceAddress};
use crate::model::instruction::Instruction;
use crate::model::transaction::{InstructionKind, InstructionList, TransactionManifest};
use crate::visitor::{
    traverse_instruction, AccountDeposit, AccountDepositsInstructionVisitor,
    AccountInteractionsInstructionVisitor, AccountProofsInstructionVisitor, AccountWithdraw,
    AccountWithdrawsInstructionVisitor, AddressAggregatorVisitor, ValueNetworkAggregatorVisitor,
};
use radix_engine::transaction::{TransactionReceipt, TransactionResult};
use radix_engine::types::{scrypto_decode, ComponentAddress};
use toolkit_derive::serializable;

use super::traits::Handler;
use super::{ConvertManifestHandler, ConvertManifestRequest};

// =================
// Model Definition
// =================

/// Analyzes the passed manifest to determine the entities that this manifest interacts with.
#[serializable]
pub struct AnalyzeManifestWithPreviewContextRequest {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the manifest will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// The manifest to analyze.
    pub manifest: TransactionManifest,

    /// The SBOR encoded transaction receipt obtained from the performing a transaction preview
    /// with the given manifest. This byte array is serialized as a hex-encoded byte array.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::hex::Hex")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    pub transaction_receipt: Vec<u8>,
}

/// The response of the [`AnalyzeManifestWithPreviewContextRequest`]
#[serializable]
pub struct AnalyzeManifestWithPreviewContextResponse {
    // TODO: Should we remove all native packages and components from this list?
    /// The set of addresses encountered in the manifest.
    ///
    /// This field is populated through static analysis of the manifest and captures the set of all
    /// addresses encountered in the manifest. This captures addresses if they're used in calls,
    /// used as arguments, or contained as parts of some list or array.
    pub encountered_addresses: EncounteredAddresses,

    /// A set of account component addresses which were involved in actions that require auth.
    ///
    /// This field is obtained through static analysis of the manifest by the Radix Engine Toolkit.
    /// When the toolkit encounters an instruction being performed on an account that requires auth
    /// (e.g., withdrawing funds, locking fee, creating proofs), it is added to this address set.
    ///
    /// It is then the job of the wallet to determine whether the account has been securified and
    /// uses an access controller or is still operating in signature mode and produce the correct
    /// auth based on that.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub accounts_requiring_auth: BTreeSet<NetworkAwareComponentAddress>,

    /// A set of the resource addresses of which proofs were created from accounts in this
    /// manifest.
    ///
    /// This field is populated through static analysis of the manifest instruction. This field
    /// captures the resource addresses of all of the proofs created from accounts throughout the
    /// manifest. This field does not capture the amount of the proof created nor which account the
    /// proof was created from.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub account_proof_resources: BTreeSet<NetworkAwareResourceAddress>,

    /// A list of the account withdraws seen in the manifest.
    ///
    /// This field is populated through static analysis of the manifest and it captures information
    /// relating to the resources withdrawn from accounts such as the component address of the
    /// account, the resource address of the withdrawn, and either an amount or set of non-fungible
    /// local ids of the withdrawn resources.
    pub account_withdraws: Vec<AccountWithdraw>,

    /// A list of the account deposits which occur in the transaction.
    ///
    /// This field is populated through both static analysis of the manifest and through the
    /// context provided by the transaction preview. All deposits referred to as "exact" are
    /// deposits which are guaranteed by the static analysis while the ones referred to as
    /// "estimate" are deposits which are primarily obtained from the context of the previews
    pub account_deposits: Vec<AccountDeposit>,

    /// The set of entities which were newly created in this transaction.
    pub created_entities: CreatedEntities,
}

/// The set of addresses encountered in the manifest
#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct EncounteredAddresses {
    /// The set of component addresses encountered in the manifest
    pub component_addresses: EncounteredComponents,

    /// The set of resource addresses encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub resource_addresses: BTreeSet<NetworkAwareResourceAddress>,

    /// The set of package addresses encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub package_addresses: BTreeSet<NetworkAwarePackageAddress>,
}

/// The set of newly created entities
#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct CreatedEntities {
    /// The set of addresses of newly created components.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub component_addresses: BTreeSet<NetworkAwareComponentAddress>,

    /// The set of addresses of newly created resources.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub resource_addresses: BTreeSet<NetworkAwareResourceAddress>,

    /// The set of addresses of newly created packages.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub package_addresses: BTreeSet<NetworkAwarePackageAddress>,
}

/// The set of addresses encountered in the manifest
#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct EncounteredComponents {
    /// The set of user application components encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub user_applications: BTreeSet<NetworkAwareComponentAddress>,

    /// The set of account components encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub accounts: BTreeSet<NetworkAwareComponentAddress>,

    /// The set of identity components encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub identities: BTreeSet<NetworkAwareComponentAddress>,

    /// The set of clock components encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub clocks: BTreeSet<NetworkAwareComponentAddress>,

    /// The set of epoch_manager components encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub epoch_managers: BTreeSet<NetworkAwareComponentAddress>,

    /// The set of validator components encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub validators: BTreeSet<NetworkAwareComponentAddress>,

    /// The set of validator components encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub access_controller: BTreeSet<NetworkAwareComponentAddress>,
}

impl From<BTreeSet<NetworkAwareComponentAddress>> for EncounteredComponents {
    fn from(value: BTreeSet<NetworkAwareComponentAddress>) -> Self {
        let mut user_applications = BTreeSet::new();
        let mut accounts = BTreeSet::new();
        let mut identities = BTreeSet::new();
        let mut clocks = BTreeSet::new();
        let mut epoch_managers = BTreeSet::new();
        let mut validators = BTreeSet::new();
        let mut access_controller = BTreeSet::new();

        for address in value {
            let underlying_address = address.address;
            match underlying_address {
                ComponentAddress::Normal(..) => user_applications.insert(address),
                ComponentAddress::Account(..)
                | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                | ComponentAddress::EddsaEd25519VirtualAccount(..) => accounts.insert(address),
                ComponentAddress::Identity(..)
                | ComponentAddress::EcdsaSecp256k1VirtualIdentity(..)
                | ComponentAddress::EddsaEd25519VirtualIdentity(..) => identities.insert(address),
                ComponentAddress::Clock(..) => clocks.insert(address),
                ComponentAddress::EpochManager(..) => epoch_managers.insert(address),
                ComponentAddress::Validator(..) => validators.insert(address),
                ComponentAddress::AccessController(..) => access_controller.insert(address),
            };
        }

        Self {
            user_applications,
            accounts,
            identities,
            clocks,
            epoch_managers,
            validators,
            access_controller,
        }
    }
}

// ===============
// Implementation
// ===============

pub struct AnalyzeManifestWithPreviewContextHandler;

impl Handler<AnalyzeManifestWithPreviewContextRequest, AnalyzeManifestWithPreviewContextResponse>
    for AnalyzeManifestWithPreviewContextHandler
{
    fn pre_process(
        mut request: AnalyzeManifestWithPreviewContextRequest,
    ) -> Result<AnalyzeManifestWithPreviewContextRequest> {
        // Visitors
        let mut network_aggregator_visitor = ValueNetworkAggregatorVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match request.manifest.instructions {
            InstructionList::Parsed(ref mut instructions) => instructions,
            InstructionList::String(..) => &mut [],
        };

        // Traverse instructions with visitors
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(instruction, &mut [&mut network_aggregator_visitor], &mut [])
            })
            .collect::<Result<Vec<_>>>()?;

        // Check for network mismatches
        if let Some(network_id) = network_aggregator_visitor
            .0
            .iter()
            .find(|network_id| **network_id != request.network_id)
        {
            return Err(crate::error::Error::NetworkMismatchError {
                found: *network_id,
                expected: request.network_id,
            });
        }
        Ok(request)
    }

    fn handle(
        request: &AnalyzeManifestWithPreviewContextRequest,
    ) -> Result<AnalyzeManifestWithPreviewContextResponse> {
        // Getting the instructions in the passed manifest as Parsed instructions.
        let mut instructions = {
            let manifest = ConvertManifestHandler::fulfill(ConvertManifestRequest {
                network_id: request.network_id,
                instructions_output_kind: InstructionKind::Parsed,
                manifest: request.manifest.clone(),
            })?
            .manifest;

            match manifest.instructions {
                InstructionList::Parsed(instructions) => Ok(instructions),
                InstructionList::String(..) => Err(crate::error::Error::Infallible {
                    message: "Impossible Case! We converted to parsed but it's still a string!"
                        .into(),
                }),
            }
        }?;

        let receipt = scrypto_decode::<TransactionReceipt>(&request.transaction_receipt)?;
        let commit = match receipt.result {
            TransactionResult::Commit(commit) => Ok(commit),
            _ => Err(Error::TransactionNotCommitted),
        }?;

        // Setting up the visitors that will be used on the instructions
        let mut account_interactions_visitor = AccountInteractionsInstructionVisitor::default();
        let mut account_withdraws_visitor = AccountWithdrawsInstructionVisitor::default();
        let mut account_proofs_visitor = AccountProofsInstructionVisitor::default();
        let mut address_aggregator_visitor = AddressAggregatorVisitor::default();
        let mut account_deposits_visitor = {
            let resource_changes = receipt
                .execution_trace
                .resource_changes
                .clone()
                .into_iter()
                .map(|(k, v)| (k as u32, v))
                .collect();
            let worktop_changes = receipt
                .execution_trace
                .worktop_changes()
                .into_iter()
                .map(|(k, v)| (k as u32, v))
                .collect();
            AccountDepositsInstructionVisitor::new(
                request.network_id,
                resource_changes,
                worktop_changes,
            )
        };
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(
                    instruction,
                    &mut [&mut address_aggregator_visitor],
                    &mut [
                        &mut account_interactions_visitor,
                        &mut account_withdraws_visitor,
                        &mut account_deposits_visitor,
                        &mut account_proofs_visitor,
                    ],
                )
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(AnalyzeManifestWithPreviewContextResponse {
            accounts_requiring_auth: account_interactions_visitor.auth_required,
            account_proof_resources: account_proofs_visitor.created_proofs,
            encountered_addresses: EncounteredAddresses {
                component_addresses: address_aggregator_visitor.component_addresses.into(),
                resource_addresses: address_aggregator_visitor.resource_addresses,
                package_addresses: address_aggregator_visitor.package_addresses,
            },
            account_withdraws: account_withdraws_visitor.0,
            account_deposits: account_deposits_visitor.deposits,
            created_entities: CreatedEntities {
                component_addresses: commit
                    .new_component_addresses()
                    .iter()
                    .map(|address| NetworkAwareComponentAddress {
                        address: *address,
                        network_id: request.network_id,
                    })
                    .collect(),
                resource_addresses: commit
                    .new_resource_addresses()
                    .iter()
                    .map(|address| NetworkAwareResourceAddress {
                        address: *address,
                        network_id: request.network_id,
                    })
                    .collect(),
                package_addresses: commit
                    .new_package_addresses()
                    .iter()
                    .map(|address| NetworkAwarePackageAddress {
                        address: *address,
                        network_id: request.network_id,
                    })
                    .collect(),
            },
        })
    }

    fn post_process(
        _: &AnalyzeManifestWithPreviewContextRequest,
        response: AnalyzeManifestWithPreviewContextResponse,
    ) -> Result<AnalyzeManifestWithPreviewContextResponse> {
        Ok(response)
    }
}