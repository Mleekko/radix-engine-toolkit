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

use sbor::*;
use scrypto::prelude::*;
use transaction::errors::*;
use transaction::model::*;
use transaction::validation::*;

use crate::models::transaction_hash::TransactionHash;

pub fn hash(
    notarized_transaction: &NotarizedTransactionV1,
) -> Result<TransactionHash, PrepareError> {
    notarized_transaction
        .prepare()
        .map(|prepared| prepared.notarized_transaction_hash())
        .map(|hash| {
            TransactionHash::new(
                hash,
                notarized_transaction.signed_intent.intent.header.network_id,
            )
        })
}

pub fn compile(notarized_transaction: &NotarizedTransactionV1) -> Result<Vec<u8>, EncodeError> {
    notarized_transaction.to_payload_bytes()
}

pub fn decompile<T>(payload_bytes: T) -> Result<NotarizedTransactionV1, DecodeError>
where
    T: AsRef<[u8]>,
{
    NotarizedTransactionV1::from_payload_bytes(payload_bytes.as_ref())
}

pub fn statically_validate(
    notarized_transaction: &NotarizedTransactionV1,
    validation_config: ValidationConfig,
) -> Result<(), TransactionValidationError> {
    let validator = NotarizedTransactionValidator::new(validation_config);
    notarized_transaction
        .prepare()
        .map_err(TransactionValidationError::PrepareError)
        .and_then(|prepared| validator.validate(prepared))
        .map(|_| ())
}
