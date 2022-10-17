use crate::error::Error;
use crate::export_request;
use crate::models::serde::NetworkAwareResourceAddress;
use crate::traits::{Request, Validate};
use scrypto::prelude::{NonFungibleAddress, NonFungibleId};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressRequest {
    pub resource_address: NetworkAwareResourceAddress,
    #[serde_as(as = "DisplayFromStr")]
    pub non_fungible_id: NonFungibleId,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub non_fungible_address: NonFungibleAddress,
}

// ===========
// Validation
// ===========

impl Validate for DeriveNonFungibleAddressRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DeriveNonFungibleAddressResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DeriveNonFungibleAddressResponse> for DeriveNonFungibleAddressRequest {
    fn handle_request(self) -> Result<DeriveNonFungibleAddressResponse, Error> {
        let non_fungible_address: NonFungibleAddress =
            NonFungibleAddress::new(self.resource_address.address, self.non_fungible_id);

        Ok(DeriveNonFungibleAddressResponse {
            non_fungible_address,
        })
    }
}

export_request!(DeriveNonFungibleAddressRequest as derive_non_fungible_address);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
