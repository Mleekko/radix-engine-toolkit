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

use radix_engine_toolkit::utils::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use transaction::manifest::*;
use transaction::prelude::*;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableInstructions {
    String(String),
    Parsed(Vec<SerializableInstruction>),
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub enum SerializableInstructionsKind {
    String,
    Parsed,
}

impl SerializableInstructions {
    pub fn to_instructions(
        &self,
        network_id: u8,
    ) -> Result<Vec<InstructionV1>, SerializableInstructionsError> {
        match self {
            Self::String(string) => transaction::manifest::compile(
                string,
                &network_definition_from_network_id(network_id),
                MockBlobProvider::new(),
            )
            .map_err(SerializableInstructionsError::from)
            .map(|manifest| manifest.instructions),
            Self::Parsed(parsed) => {
                to_native_instructions(parsed).map_err(SerializableInstructionsError::from)
            }
        }
    }

    pub fn convert_serializable_instructions_kind(
        &mut self,
        to_type: SerializableInstructionsKind,
        network_id: u8,
    ) -> Result<(), SerializableInstructionsError> {
        match (&self, to_type) {
            (Self::String(..), SerializableInstructionsKind::String)
            | (Self::Parsed(..), SerializableInstructionsKind::Parsed) => Ok(()),
            (Self::Parsed(parsed), SerializableInstructionsKind::String) => {
                let instructions = to_native_instructions(parsed)?;
                let string = decompile(
                    &instructions,
                    &network_definition_from_network_id(network_id),
                )?;
                *self = Self::String(string);
                Ok(())
            }
            (Self::String(string), SerializableInstructionsKind::Parsed) => {
                let instructions = transaction::manifest::compile(
                    string,
                    &network_definition_from_network_id(network_id),
                    MockBlobProvider::new(),
                )
                .map(|manifest| manifest.instructions)?;
                let instructions = to_serializable_instructions(&instructions)?;
                *self = Self::Parsed(instructions);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SerializableInstructionsError {
    CompilationError(String),
    DecompilationError(String),
    LocatedInstructionConversionError(LocatedInstructionConversionError),
}

impl From<CompileError> for SerializableInstructionsError {
    fn from(value: CompileError) -> Self {
        Self::CompilationError(format!("{value:?}"))
    }
}

impl From<DecompileError> for SerializableInstructionsError {
    fn from(value: DecompileError) -> Self {
        Self::CompilationError(format!("{value:?}"))
    }
}

impl From<LocatedInstructionConversionError> for SerializableInstructionsError {
    fn from(value: LocatedInstructionConversionError) -> Self {
        Self::LocatedInstructionConversionError(value)
    }
}
