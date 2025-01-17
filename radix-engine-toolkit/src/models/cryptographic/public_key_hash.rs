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

use schemars::JsonSchema;
use scrypto::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::prelude::*;

#[serde_as]
#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializablePublicKeyHash {
    Secp256k1(#[typeshare(serialized_as = "String")] AsHex<[u8; 29]>),
    Ed25519(#[typeshare(serialized_as = "String")] AsHex<[u8; 29]>),
}

impl From<PublicKeyHash> for SerializablePublicKeyHash {
    fn from(value: PublicKeyHash) -> Self {
        match value {
            PublicKeyHash::Secp256k1(public_key) => Self::Secp256k1(public_key.0.into()),
            PublicKeyHash::Ed25519(public_key) => Self::Ed25519(public_key.0.into()),
        }
    }
}

impl From<SerializablePublicKeyHash> for PublicKeyHash {
    fn from(value: SerializablePublicKeyHash) -> Self {
        match value {
            SerializablePublicKeyHash::Secp256k1(value) => {
                Self::Secp256k1(Secp256k1PublicKeyHash(*value))
            }
            SerializablePublicKeyHash::Ed25519(value) => {
                Self::Ed25519(Ed25519PublicKeyHash(*value))
            }
        }
    }
}

impl From<PublicKey> for SerializablePublicKeyHash {
    fn from(value: PublicKey) -> Self {
        match value {
            PublicKey::Secp256k1(public_key) => Self::Secp256k1(public_key.to_hash().0.into()),
            PublicKey::Ed25519(public_key) => Self::Ed25519(
                Ed25519PublicKeyHash::new_from_public_key(&public_key)
                    .0
                    .into(),
            ),
        }
    }
}
