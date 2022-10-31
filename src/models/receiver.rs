use radix_transaction::manifest::ast::{
    RENode as AstRENode, Receiver as AstReceiver, Value as AstValue,
};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::error::Error;
use crate::models::{Identifier, NodeId};

use super::ValueKind;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "node")]
pub enum Receiver {
    Owned(RENode),
    Ref(RENode),
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "identifier")]
pub enum RENode {
    Bucket(Identifier),
    Proof(Identifier),

    AuthZoneStack(#[serde_as(as = "DisplayFromStr")] u32),
    Worktop,

    Global(String),
    KeyValueStore(NodeId),
    NonFungibleStore(NodeId),
    Component(NodeId),
    System(NodeId),
    Vault(NodeId),
    ResourceManager(NodeId),
    Package(NodeId),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RENodeKind {
    Bucket,
    Proof,

    AuthZoneStack,
    Worktop,

    Global,
    KeyValueStore,
    NonFungibleStore,
    Component,
    System,
    Vault,
    ResourceManager,
    Package,
}

// ============
// Conversions
// ============

impl From<Receiver> for AstReceiver {
    fn from(receiver: Receiver) -> Self {
        ast_receiver_from_receiver(&receiver)
    }
}

impl TryFrom<AstReceiver> for Receiver {
    type Error = Error;

    fn try_from(value: AstReceiver) -> Result<Self, Self::Error> {
        receiver_from_ast_receiver(&value)
    }
}

pub fn ast_receiver_from_receiver(receiver: &Receiver) -> AstReceiver {
    match receiver {
        Receiver::Owned(re_node) => AstReceiver::Owned(ast_re_node_from_re_node(re_node)),
        Receiver::Ref(re_node) => AstReceiver::Ref(ast_re_node_from_re_node(re_node)),
    }
}

pub fn receiver_from_ast_receiver(receiver: &AstReceiver) -> Result<Receiver, Error> {
    let receiver: Receiver = match receiver {
        AstReceiver::Owned(re_node) => Receiver::Owned(re_node_from_ast_re_node(re_node)?),
        AstReceiver::Ref(re_node) => Receiver::Ref(re_node_from_ast_re_node(re_node)?),
    };
    Ok(receiver)
}

pub fn ast_re_node_from_re_node(re_node: &RENode) -> AstRENode {
    match re_node {
        RENode::Bucket(identifier) => {
            let ast_value: AstValue = match identifier {
                Identifier::String(string) => AstValue::String(string.clone()),
                Identifier::U32(number) => AstValue::U32(*number),
            };
            AstRENode::Bucket(ast_value)
        }
        RENode::Proof(identifier) => {
            let ast_value: AstValue = match identifier {
                Identifier::String(string) => AstValue::String(string.clone()),
                Identifier::U32(number) => AstValue::U32(*number),
            };
            AstRENode::Proof(ast_value)
        }

        RENode::AuthZoneStack(auth_zone_id) => {
            let ast_value: AstValue = AstValue::U32(*auth_zone_id);
            AstRENode::AuthZoneStack(ast_value)
        }
        RENode::Worktop => AstRENode::Worktop,

        RENode::Global(identifier) => {
            let ast_value: AstValue = AstValue::String(identifier.to_owned());
            AstRENode::Global(ast_value)
        }
        RENode::KeyValueStore(identifier) => {
            let ast_value: AstValue = AstValue::String(identifier.to_string());
            AstRENode::KeyValueStore(ast_value)
        }
        RENode::NonFungibleStore(identifier) => {
            let ast_value: AstValue = AstValue::String(identifier.to_string());
            AstRENode::NonFungibleStore(ast_value)
        }
        RENode::Component(identifier) => {
            let ast_value: AstValue = AstValue::String(identifier.to_string());
            AstRENode::Component(ast_value)
        }
        RENode::System(identifier) => {
            let ast_value: AstValue = AstValue::String(identifier.to_string());
            AstRENode::System(ast_value)
        }
        RENode::Vault(identifier) => {
            let ast_value: AstValue = AstValue::String(identifier.to_string());
            AstRENode::Vault(ast_value)
        }
        RENode::ResourceManager(identifier) => {
            let ast_value: AstValue = AstValue::String(identifier.to_string());
            AstRENode::ResourceManager(ast_value)
        }
        RENode::Package(identifier) => {
            let ast_value: AstValue = AstValue::String(identifier.to_string());
            AstRENode::Package(ast_value)
        }
    }
}

pub fn re_node_from_ast_re_node(ast_re_node: &AstRENode) -> Result<RENode, Error> {
    let re_node: RENode = match ast_re_node {
        AstRENode::Bucket(identifier) => {
            if let AstValue::U32(identifier) = identifier {
                RENode::Bucket(Identifier::U32(*identifier))
            } else if let AstValue::String(identifier) = identifier {
                RENode::Bucket(Identifier::String(identifier.clone()))
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Bucket,
                    allowed_children_kinds: vec![ValueKind::U32, ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Proof(identifier) => {
            if let AstValue::U32(identifier) = identifier {
                RENode::Proof(Identifier::U32(*identifier))
            } else if let AstValue::String(identifier) = identifier {
                RENode::Proof(Identifier::String(identifier.clone()))
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Proof,
                    allowed_children_kinds: vec![ValueKind::U32, ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }

        AstRENode::AuthZoneStack(identifier) => {
            if let AstValue::U32(identifier) = identifier {
                RENode::AuthZoneStack(*identifier)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::AuthZoneStack,
                    allowed_children_kinds: vec![ValueKind::U32],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }

        AstRENode::Worktop => RENode::Worktop,

        AstRENode::Global(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Global(identifier.clone())
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Global,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::KeyValueStore(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::KeyValueStore(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::KeyValueStore,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::NonFungibleStore(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::NonFungibleStore(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::NonFungibleStore,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Component(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Component(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Component,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::System(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::System(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::System,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Vault(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Vault(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Vault,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::ResourceManager(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::ResourceManager(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::ResourceManager,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Package(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Package(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Package,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
    };
    Ok(re_node)
}