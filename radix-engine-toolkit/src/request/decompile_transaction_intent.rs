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

use crate::error::Result;
use crate::model::transaction::TransactionIntent;
use crate::request::Handler;
use crate::traits::CompilableIntent;
use crate::{
    traverse_instruction, Instruction, InstructionKind, InstructionList, ValueAliasingVisitor,
};
use serializable::serializable;

// =================
// Model Definition
// =================

/// This function does the opposite of the compile_transaction_intent function. It takes in a
/// compiled transaction intent and decompiles it into its human-readable / machine-readable format.
#[serializable]
pub struct DecompileTransactionIntentRequest {
    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub instructions_output_kind: InstructionKind,

    /// A byte array serialized as a hex string which represents the compiled transaction intent to
    /// decompile.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

/// The response from [`DecompileTransactionIntentRequest`].
#[serializable]
pub struct DecompileTransactionIntentResponse {
    /// The decompiled transaction intent where the instructions are in the format specified in the
    /// request.
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}

// ===============
// Implementation
// ===============

pub struct DecompileTransactionIntentHandler;

impl Handler<DecompileTransactionIntentRequest, DecompileTransactionIntentResponse>
    for DecompileTransactionIntentHandler
{
    fn pre_process(
        request: DecompileTransactionIntentRequest,
    ) -> Result<DecompileTransactionIntentRequest> {
        Ok(request)
    }

    fn handle(
        request: &DecompileTransactionIntentRequest,
    ) -> Result<DecompileTransactionIntentResponse> {
        TransactionIntent::decompile(&request.compiled_intent, request.instructions_output_kind)
            .map(|transaction_intent| DecompileTransactionIntentResponse { transaction_intent })
    }

    fn post_process(
        _: &DecompileTransactionIntentRequest,
        mut response: DecompileTransactionIntentResponse,
    ) -> Result<DecompileTransactionIntentResponse> {
        // Visitors
        let mut aliasing_visitor = ValueAliasingVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] =
            match response.transaction_intent.manifest.instructions {
                InstructionList::Parsed(ref mut instructions) => instructions,
                InstructionList::String(..) => &mut [],
            };

        // Traverse instructions with visitors
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(instruction, &mut [&mut aliasing_visitor], &mut [])
            })
            .collect::<Result<Vec<_>>>()?;

        // The aliasing visitor performs all of the modifications in place as it meets them. Nothing
        // else needs to be done here.

        Ok(response)
    }
}