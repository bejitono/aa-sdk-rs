use ethers::types::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

// pub struct UserOperation {
//     pub sender: ::ethers::core::types::Address,
//     pub nonce: ::ethers::core::types::U256,
//     pub init_code: ::ethers::core::types::Bytes,
//     pub call_data: ::ethers::core::types::Bytes,
//     pub call_gas_limit: ::ethers::core::types::U256,
//     pub verification_gas_limit: ::ethers::core::types::U256,
//     pub pre_verification_gas: ::ethers::core::types::U256,
//     pub max_fee_per_gas: ::ethers::core::types::U256,
//     pub max_priority_fee_per_gas: ::ethers::core::types::U256,
//     pub paymaster_and_data: ::ethers::core::types::Bytes,
//     pub signature: ::ethers::core::types::Bytes,
// }

/// Parameters for sending a user operation
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UserOperationRequest {
    /// Sender address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Address>,

    /// Nonce
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,

    /// Init code
    #[serde(rename = "initCode", default, skip_serializing_if = "Option::is_none")]
    pub init_code: Option<Bytes>,

    /// Call data
    #[serde(rename = "callData", default, skip_serializing_if = "Option::is_none")]
    pub call_data: Option<Bytes>,

    /// Call gas limit
    #[serde(
        rename = "callGasLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub call_gas_limit: Option<U256>,

    /// Verification gas limit
    #[serde(
        rename = "verificationGasLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_gas_limit: Option<U256>,

    /// Pre-verification gas limit
    #[serde(
        rename = "preVerificationGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_verification_gas: Option<U256>,

    /// Represents the maximum amount that a user is willing
    /// to pay for their tx (inclusive of baseFeePerGas and maxPriorityFeePerGas).
    #[serde(
        rename = "maxFeePerGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_fee_per_gas: Option<U256>,

    /// Represents the maximum tx fee that will go to the miner as part of the user's
    #[serde(
        rename = "maxPriorityFeePerGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_priority_fee_per_gas: Option<U256>,

    /// Sender address
    #[serde(
        rename = "paymasterAndData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub paymaster_and_data: Option<Bytes>,

    /// Sender address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Bytes>,
}

impl UserOperationRequest {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn sender<T: Into<Address>>(mut self, sender: T) -> Self {
        self.sender = Some(sender.into());
        self
    }

    #[must_use]
    pub fn nonce<T: Into<U256>>(mut self, nonce: T) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    #[must_use]
    pub fn init_code<T: Into<Bytes>>(mut self, init_code: T) -> Self {
        self.init_code = Some(init_code.into());
        self
    }

    #[must_use]
    pub fn call_data<T: Into<Bytes>>(mut self, call_data: T) -> Self {
        self.call_data = Some(call_data.into());
        self
    }

    #[must_use]
    pub fn call_gas_limit<T: Into<U256>>(mut self, call_gas_limit: T) -> Self {
        self.call_gas_limit = Some(call_gas_limit.into());
        self
    }

    #[must_use]
    pub fn verification_gas_limit<T: Into<U256>>(mut self, verification_gas_limit: T) -> Self {
        self.verification_gas_limit = Some(verification_gas_limit.into());
        self
    }

    #[must_use]
    pub fn pre_verification_gas<T: Into<U256>>(mut self, pre_verification_gas: T) -> Self {
        self.pre_verification_gas = Some(pre_verification_gas.into());
        self
    }

    #[must_use]
    pub fn max_fee_per_gas<T: Into<U256>>(mut self, max_fee_per_gas: T) -> Self {
        self.max_fee_per_gas = Some(max_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn max_priority_fee_per_gas<T: Into<U256>>(mut self, max_priority_fee_per_gas: T) -> Self {
        self.max_priority_fee_per_gas = Some(max_priority_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn paymaster_and_data<T: Into<Bytes>>(mut self, paymaster_and_data: T) -> Self {
        self.paymaster_and_data = Some(paymaster_and_data.into());
        self
    }

    #[must_use]
    pub fn signature<T: Into<Bytes>>(mut self, signature: T) -> Self {
        self.signature = Some(signature.into());
        self
    }
}