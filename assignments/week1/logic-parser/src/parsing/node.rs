#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "serde")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum ASTNode {
    Identifier { name: String },
    Literal { value: bool },
    #[serde(rename = "operator.not")]
    Not { operand: Box<ASTNode> },
    #[serde(rename = "operator.and")]
    And { left: Box<ASTNode>, right: Box<ASTNode> },
    #[serde(rename = "operator.or")]
    Or { left: Box<ASTNode>, right: Box<ASTNode> },
    #[serde(rename = "operator.implies")]
    Implies { left: Box<ASTNode>, right: Box<ASTNode> },
    #[serde(rename = "operator.iff")]
    IfAndOnlyIf { left: Box<ASTNode>, right: Box<ASTNode> },
}

// Serde Serialize and Deserialize traits are available when the
// optional 'serde' feature is enabled

#[cfg(not(feature = "serde"))]
#[derive(Debug)]
pub enum ASTNode {
    Identifier { name: String },
    Literal { value: bool },
    Not { operand: Box<ASTNode> },
    And { left: Box<ASTNode>, right: Box<ASTNode> },
    Or { left: Box<ASTNode>, right: Box<ASTNode> },
    Implies { left: Box<ASTNode>, right: Box<ASTNode> },
    IfAndOnlyIf { left: Box<ASTNode>, right: Box<ASTNode> },
}

impl ASTNode {
    pub fn as_string(&self) -> String {
        format!("{:#?}", self)
    }

    pub fn repr(&self) -> &str {
        match self {
            ASTNode::Identifier { name } => name,
            ASTNode::Literal { value } => if *value { "1" } else { "0" },
            ASTNode::Not { .. } => "¬",
            ASTNode::And { .. } => "∧",
            ASTNode::Or { .. } => "∨",
            ASTNode::Implies { .. } => "⇒",
            ASTNode::IfAndOnlyIf { .. } => "⟷",
        }
    }

    #[cfg(not(feature = "serde"))]
    pub fn as_json(&self) -> String {
        match self {
            ASTNode::Identifier { name } => {
                format!(r###"{{
                    "type": "identifier",
                    "name": "{name}"
                }}"###)
            },
            ASTNode::Literal { value } => {
                format!(r###"{{
                    "type": "literal",
                    "value": {value}
                }}"###)
            },
            ASTNode::Not { operand } => {
                format!(r###"{{
                    "type": "operator.not",
                    "operand": {operand}
                }}"###, operand=operand.as_json())
            },
            ASTNode::And { left, right } => {
                format!(r###"{{
                    "type": "operator.and",
                    "left": {left},
                    "right": {right}
                }}"###, left=left.as_json(), right=right.as_json())
            },
            ASTNode::Or { left, right } => {
                format!(r###"{{
                    "type": "operator.or",
                    "left": {left},
                    "right": {right}
                }}"###, left=left.as_json(), right=right.as_json())
            },
            ASTNode::Implies { left, right } => {
                format!(r###"{{
                    "type": "operator.implies",
                    "left": {left},
                    "right": {right}
                }}"###, left=left.as_json(), right=right.as_json())
            },
            ASTNode::IfAndOnlyIf { left, right } => {
                format!(r###"{{
                    "type": "operator.iff",
                    "left": {left},
                    "right": {right}
                }}"###, left=left.as_json(), right=right.as_json())
            }
        }
    }

    #[cfg(feature = "serde")]
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
