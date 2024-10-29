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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CashOutResponse {
    #[serde(rename = "phone_number")]
    pub phone_number: String,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "status")]
    pub status: models::TransactionStatusEnum,
}

impl CashOutResponse {
    pub fn new(phone_number: String, amount: i32, full_name: String, status: models::TransactionStatusEnum) -> CashOutResponse {
        CashOutResponse {
            phone_number,
            amount,
            full_name,
            status,
        }
    }
}

