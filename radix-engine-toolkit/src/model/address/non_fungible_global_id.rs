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

use crate::model::address::EntityAddress;
use scrypto::prelude::{
    FromPublicKey, NonFungibleGlobalId as NativeNonFungibleGlobalId, NonFungibleLocalId, PublicKey,
};
use toolkit_derive::serializable;

use crate::model::address::NetworkAwareResourceAddress;

/// Represents a non-fungible address which may be considered as the "global" address of a
/// non-fungible unit as it contains both the resource address and the non-fungible id for that
/// unit.
#[serializable]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[schemars(
    example = "crate::example::address::non_fungible::non_fungible_global_integer",
    example = "crate::example::address::non_fungible::non_fungible_global_string",
    example = "crate::example::address::non_fungible::non_fungible_global_bytes",
    example = "crate::example::address::non_fungible::non_fungible_global_uuid"
)]
pub struct NonFungibleGlobalId {
    #[schemars(with = "EntityAddress")]
    #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
    pub resource_address: NetworkAwareResourceAddress,

    #[serde_as(as = "serde_with::TryFromInto<crate::model::address::NonFungibleLocalId>")]
    #[schemars(with = "crate::model::address::NonFungibleLocalId")]
    pub non_fungible_local_id: NonFungibleLocalId,
}

impl NonFungibleGlobalId {
    pub fn new(
        resource_address: NetworkAwareResourceAddress,
        non_fungible_local_id: NonFungibleLocalId,
    ) -> Self {
        Self {
            resource_address,
            non_fungible_local_id,
        }
    }

    pub fn from_public_key<P: Into<PublicKey> + Clone>(public_key: &P, network_id: u8) -> Self {
        let native_non_fungible_global_id = NativeNonFungibleGlobalId::from_public_key(public_key);
        Self {
            resource_address: NetworkAwareResourceAddress {
                network_id,
                address: native_non_fungible_global_id.resource_address(),
            },
            non_fungible_local_id: native_non_fungible_global_id.local_id().clone(),
        }
    }
}

impl From<NonFungibleGlobalId> for scrypto::prelude::NonFungibleGlobalId {
    fn from(value: NonFungibleGlobalId) -> Self {
        scrypto::prelude::NonFungibleGlobalId::new(
            value.resource_address.address,
            value.non_fungible_local_id,
        )
    }
}
