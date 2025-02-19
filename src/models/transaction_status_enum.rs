/*
 * NabooApi V1
 *
 * Here you have the first version of the naboo api to create checkout automatically
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionStatusEnum {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "part_paid")]
    PartPaid,

}

impl std::fmt::Display for TransactionStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Paid => write!(f, "paid"),
            Self::Done => write!(f, "done"),
            Self::PartPaid => write!(f, "part_paid"),
        }
    }
}

impl Default for TransactionStatusEnum {
    fn default() -> TransactionStatusEnum {
        Self::Pending
    }
}

