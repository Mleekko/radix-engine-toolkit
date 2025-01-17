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

use crate::instruction_visitor::core::error::InstructionVisitorError;
use crate::instruction_visitor::core::traits::InstructionVisitor;
use crate::utils::is_identity;
use scrypto::prelude::*;
use transaction::prelude::DynamicGlobalAddress;

#[derive(Clone, Default)]
pub struct IdentityInteractionsVisitor(HashSet<ComponentAddress>);

impl IdentityInteractionsVisitor {
    pub fn output(self) -> HashSet<ComponentAddress> {
        self.0
    }
}

impl InstructionVisitor for IdentityInteractionsVisitor {
    fn visit_call_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_identity(address) {
            let component_address = match address {
                DynamicGlobalAddress::Static(address) => {
                    // This never panics. We have already checked that this is an identity when we
                    // called `is_identity`.
                    ComponentAddress::new_or_panic(address.as_node_id().0)
                }
                DynamicGlobalAddress::Named(_) => {
                    return Ok(());
                }
            };

            if crate::statics::IDENTITY_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .any(|MethodKey { ident }| ident.as_str() == method_name)
            {
                self.0.insert(component_address);
            }
        };
        Ok(())
    }

    fn visit_call_role_assignment_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_identity(address) {
            let component_address = match address {
                DynamicGlobalAddress::Static(address) => {
                    // This never panics. We have already checked that this is an identity when we
                    // called `is_identity`.
                    ComponentAddress::new_or_panic(address.as_node_id().0)
                }
                DynamicGlobalAddress::Named(_) => {
                    return Ok(());
                }
            };

            if crate::statics::ROLE_ASSIGNMENT_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .any(|MethodKey { ident }| ident.as_str() == method_name)
            {
                self.0.insert(component_address);
            }
        }
        Ok(())
    }

    fn visit_call_metadata_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_identity(address) {
            let component_address = match address {
                DynamicGlobalAddress::Static(address) => {
                    // This never panics. We have already checked that this is an identity when we
                    // called `is_identity`.
                    ComponentAddress::new_or_panic(address.as_node_id().0)
                }
                DynamicGlobalAddress::Named(_) => {
                    return Ok(());
                }
            };

            if crate::statics::METADATA_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .any(|MethodKey { ident }| ident.as_str() == method_name)
            {
                self.0.insert(component_address);
            }
        }
        Ok(())
    }

    fn visit_call_royalty_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_identity(address) {
            let component_address = match address {
                DynamicGlobalAddress::Static(address) => {
                    // This never panics. We have already checked that this is an identity when we
                    // called `is_identity`.
                    ComponentAddress::new_or_panic(address.as_node_id().0)
                }
                DynamicGlobalAddress::Named(_) => {
                    return Ok(());
                }
            };

            if crate::statics::ROYALTY_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .any(|MethodKey { ident }| ident.as_str() == method_name)
            {
                self.0.insert(component_address);
            }
        }
        Ok(())
    }
}
