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
pub struct CashOutOrangeRequest {
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "phone_number")]
    pub phone_number: String,
}

impl CashOutOrangeRequest {
    pub fn new(full_name: String, amount: i32, phone_number: String) -> CashOutOrangeRequest {
        CashOutOrangeRequest {
            full_name,
            amount,
            phone_number,
        }
    }
}

