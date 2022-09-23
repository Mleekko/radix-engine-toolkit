use crate::error::Error;
use crate::export_request;
use crate::models::manifest::ManifestInstructionsKind;
use crate::requests::*;
use crate::traits::{Request, Validate};
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileUnknownTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,
    #[serde(with = "hex::serde")]
    pub compiled_unknown_intent: Vec<u8>,
}

impl From<DecompileUnknownTransactionIntentRequest> for DecompileTransactionIntentRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileTransactionIntentRequest {
            compiled_intent: request.compiled_unknown_intent,
            manifest_instructions_output_format: request.manifest_instructions_output_format,
        }
    }
}

impl From<DecompileUnknownTransactionIntentRequest> for DecompileSignedTransactionIntentRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileSignedTransactionIntentRequest {
            compiled_signed_intent: request.compiled_unknown_intent,
            manifest_instructions_output_format: request.manifest_instructions_output_format,
        }
    }
}

impl From<DecompileUnknownTransactionIntentRequest> for DecompileNotarizedTransactionIntentRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileNotarizedTransactionIntentRequest {
            compiled_notarized_intent: request.compiled_unknown_intent,
            manifest_instructions_output_format: request.manifest_instructions_output_format,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum DecompileUnknownTransactionIntentResponse {
    TransactionIntent(DecompileTransactionIntentResponse),
    SignedTransactionIntent(DecompileSignedTransactionIntentResponse),
    NotarizedTransactionIntent(DecompileNotarizedTransactionIntentResponse),
}

impl From<DecompileTransactionIntentResponse> for DecompileUnknownTransactionIntentResponse {
    fn from(response: DecompileTransactionIntentResponse) -> Self {
        Self::TransactionIntent(response)
    }
}

impl From<DecompileSignedTransactionIntentResponse> for DecompileUnknownTransactionIntentResponse {
    fn from(response: DecompileSignedTransactionIntentResponse) -> Self {
        Self::SignedTransactionIntent(response)
    }
}

impl From<DecompileNotarizedTransactionIntentResponse>
    for DecompileUnknownTransactionIntentResponse
{
    fn from(response: DecompileNotarizedTransactionIntentResponse) -> Self {
        Self::NotarizedTransactionIntent(response)
    }
}

// ===========
// Validation
// ===========

impl Validate for DecompileUnknownTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DecompileUnknownTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        // Validation is not done here. The other request which fulfills this request will do the
        // validation on its own.
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DecompileUnknownTransactionIntentResponse>
    for DecompileUnknownTransactionIntentRequest
{
    fn handle_request(self) -> Result<DecompileUnknownTransactionIntentResponse, Error> {
        let response: DecompileUnknownTransactionIntentResponse = if let Ok(response) =
            Into::<DecompileTransactionIntentRequest>::into(self.clone()).fulfill_request()
        {
            Ok(response.into())
        } else if let Ok(response) =
            Into::<DecompileSignedTransactionIntentRequest>::into(self.clone()).fulfill_request()
        {
            Ok(response.into())
        } else if let Ok(response) =
            Into::<DecompileNotarizedTransactionIntentRequest>::into(self).fulfill_request()
        {
            Ok(response.into())
        } else {
            Err(Error::UnrecognizedCompiledIntentFormat)
        }?;

        Ok(response)
    }
}

export_request!(DecompileUnknownTransactionIntentRequest as decompile_unknown_transaction_intent);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
