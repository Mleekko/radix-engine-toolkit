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

pub fn hash(signed_intent: &SignedIntentV1) -> Result<TransactionHash, PrepareError> {
    signed_intent
        .prepare()
        .map(|prepared| prepared.signed_intent_hash())
        .map(|hash| TransactionHash::new(hash, signed_intent.intent.header.network_id))
}

pub fn compile(signed_intent: &SignedIntentV1) -> Result<Vec<u8>, EncodeError> {
    signed_intent.to_payload_bytes()
}

pub fn decompile<T>(payload_bytes: T) -> Result<SignedIntentV1, DecodeError>
where
    T: AsRef<[u8]>,
{
    SignedIntentV1::from_payload_bytes(payload_bytes.as_ref())
}

pub fn statically_validate(
    signed_intent: &SignedIntentV1,
    validation_config: ValidationConfig,
) -> Result<(), TransactionValidationError> {
    let validator = NotarizedTransactionValidator::new(validation_config);
    signed_intent
        .prepare()
        .map_err(TransactionValidationError::PrepareError)
        .and_then(|prepared| validator.validate_intent_v1(&prepared.intent))
}
