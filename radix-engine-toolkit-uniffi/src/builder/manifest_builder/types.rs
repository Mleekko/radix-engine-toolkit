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

#[derive(Debug, Clone, Record)]
pub struct ManifestBuilderBucket {
    pub name: String,
}

#[derive(Debug, Clone, Record)]
pub struct ManifestBuilderProof {
    pub name: String,
}

#[derive(Debug, Clone, Record)]
pub struct ManifestBuilderAddressReservation {
    pub name: String,
}

#[derive(Debug, Clone, Record)]
pub struct ManifestBuilderNamedAddress {
    pub name: String,
}

#[derive(Clone, Debug, Enum)]
pub enum ManifestBuilderAddress {
    Named { value: ManifestBuilderNamedAddress },
    Static { value: Arc<Address> },
}

pub trait NameRecordConvertible {
    type Native;

    fn to_native(&self, name_record: &NameRecord) -> Result<Self::Native>;
}

impl NameRecordConvertible for ManifestBuilderBucket {
    type Native = NativeManifestBucket;

    fn to_native(&self, name_record: &NameRecord) -> Result<Self::Native> {
        name_record.get_bucket(&self.name).map(|value| *value)
    }
}

impl NameRecordConvertible for ManifestBuilderProof {
    type Native = NativeManifestProof;

    fn to_native(&self, name_record: &NameRecord) -> Result<Self::Native> {
        name_record.get_proof(&self.name).map(|value| *value)
    }
}

impl NameRecordConvertible for ManifestBuilderAddressReservation {
    type Native = NativeManifestAddressReservation;

    fn to_native(&self, name_record: &NameRecord) -> Result<Self::Native> {
        name_record
            .get_address_reservation(&self.name)
            .map(|value| *value)
    }
}

impl NameRecordConvertible for ManifestBuilderNamedAddress {
    type Native = u32;

    fn to_native(&self, name_record: &NameRecord) -> Result<Self::Native> {
        name_record
            .get_named_address(&self.name)
            .map(|value| *value)
    }
}

impl NameRecordConvertible for ManifestBuilderAddress {
    type Native = NativeManifestAddress;

    fn to_native(&self, name_record: &NameRecord) -> Result<Self::Native> {
        match self {
            Self::Named { value } => value.to_native(name_record).map(Self::Native::Named),
            Self::Static { value } => Ok(Self::Native::Static((**value).into())),
        }
    }
}
