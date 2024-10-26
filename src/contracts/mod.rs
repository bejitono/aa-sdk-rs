use crate::types::request::UserOperationRequest;
use ethers::types::{Address, Bytes, U256};

pub mod entry_point;
pub use entry_point::{EntryPoint, EntryPointCalls, SenderAddressResult};

pub mod simple_account_factory;
pub use simple_account_factory::*;

pub mod simple_account;
pub use simple_account::*;

pub mod safe_signer_launch_pad;
pub use safe_signer_launch_pad::*;

pub mod safe_add_module_lib;
pub use safe_add_module_lib::*;

pub mod safe_proxy_factory;
pub use safe_proxy_factory::*;

pub mod safe_l2;
pub use safe_l2::*;

pub mod safe_4337_module;
pub use safe_4337_module::*;

pub mod safe_web_authn_shared_signer;
pub use safe_web_authn_shared_signer::*;

pub mod safe_multisend;
pub use safe_multisend::*;

#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct UserOperation {
    pub sender: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
    pub init_code: ::ethers::core::types::Bytes,
    pub call_data: ::ethers::core::types::Bytes,
    pub call_gas_limit: ::ethers::core::types::U256,
    pub verification_gas_limit: ::ethers::core::types::U256,
    pub pre_verification_gas: ::ethers::core::types::U256,
    pub max_fee_per_gas: ::ethers::core::types::U256,
    pub max_priority_fee_per_gas: ::ethers::core::types::U256,
    pub paymaster_and_data: ::ethers::core::types::Bytes,
    pub signature: ::ethers::core::types::Bytes,
}

impl From<UserOperation> for entry_point::UserOperation {
    fn from(request: UserOperation) -> Self {
        Self {
            sender: request.sender,
            nonce: request.nonce,
            init_code: request.init_code,
            call_data: request.call_data,
            call_gas_limit: request.call_gas_limit,
            verification_gas_limit: request.verification_gas_limit,
            pre_verification_gas: request.pre_verification_gas,
            max_fee_per_gas: request.max_fee_per_gas,
            max_priority_fee_per_gas: request.max_priority_fee_per_gas,
            paymaster_and_data: request.paymaster_and_data,
            signature: request.signature,
        }
    }
}

impl From<UserOperation> for simple_account::UserOperation {
    fn from(request: UserOperation) -> Self {
        Self {
            sender: request.sender,
            nonce: request.nonce,
            init_code: request.init_code,
            call_data: request.call_data,
            call_gas_limit: request.call_gas_limit,
            verification_gas_limit: request.verification_gas_limit,
            pre_verification_gas: request.pre_verification_gas,
            max_fee_per_gas: request.max_fee_per_gas,
            max_priority_fee_per_gas: request.max_priority_fee_per_gas,
            paymaster_and_data: request.paymaster_and_data,
            signature: request.signature,
        }
    }
}

impl From<UserOperationRequest> for UserOperation {
    fn from(request: UserOperationRequest) -> Self {
        Self {
            sender: request.sender.unwrap_or_else(|| Address::zero()),
            nonce: request.nonce.unwrap_or_else(|| U256::zero()),
            init_code: request.init_code.unwrap_or_else(|| Bytes::new()),
            call_data: request.call_data.unwrap_or_else(|| Bytes::new()),
            call_gas_limit: request.call_gas_limit.unwrap_or_else(|| U256::zero()),
            verification_gas_limit: request
                .verification_gas_limit
                .unwrap_or_else(|| U256::from(21000)),
            pre_verification_gas: request.pre_verification_gas.unwrap_or_else(|| U256::zero()),
            max_fee_per_gas: request.max_fee_per_gas.unwrap_or_else(|| U256::zero()),
            max_priority_fee_per_gas: request
                .max_priority_fee_per_gas
                .unwrap_or_else(|| U256::zero()),
            paymaster_and_data: request.paymaster_and_data.unwrap_or_else(|| Bytes::new()),
            signature: request.signature.unwrap_or_else(|| Bytes::new()),
        }
    }
}
