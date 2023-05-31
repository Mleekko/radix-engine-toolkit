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

type IntentPayload = FixedEnumVariant<{ TransactionDiscriminator::V1Intent as u8 }, IntentV1>;

pub fn hash(intent: &IntentV1) -> Result<Hash, ConvertToPreparedError> {
    intent
        .prepare()
        .map(|prepared| Hash(prepared.intent_hash().0))
}

pub fn compile(intent: &IntentV1) -> Result<Vec<u8>, EncodeError> {
    intent.to_payload_bytes()
}

pub fn decompile<T>(payload_bytes: T) -> Result<IntentV1, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode::<IntentPayload>(payload_bytes.as_ref()).map(|decompiled| decompiled.fields)
}

pub fn statically_validate(
    intent: &IntentV1,
    validation_config: ValidationConfig,
) -> Result<(), TransactionValidationError> {
    let validator = NotarizedTransactionValidator::new(validation_config);
    let prepared = intent.prepare().map_err(|error| match error {
        ConvertToPreparedError::EncodeError(error) => {
            TransactionValidationError::EncodeError(error)
        }
        ConvertToPreparedError::PrepareError(error) => {
            TransactionValidationError::PrepareError(error)
        }
    })?;
    validator.validate_intent(&prepared)
}
