use crate::contracts::{EntryPoint as EthersEntryPoint, SenderAddressResult};
use async_trait::async_trait;
use ethers::{
    abi::AbiDecode,
    providers::Middleware,
    types::{Address, Bytes, U256},
};
use std::fmt::Debug;
use thiserror::Error;

#[async_trait]
pub trait EntryPoint: Sync + Send + Debug {
    fn get_address(&self) -> Address;
    async fn get_sender_address(&self, init_code: Bytes) -> Result<Address, EntryPointError>;
    async fn get_nonce(&self, address: Address) -> Result<U256, EntryPointError>;
}

#[async_trait]
impl<M> EntryPoint for EthersEntryPoint<M>
where
    M: Middleware + 'static,
{
    fn get_address(&self) -> Address {
        self.address()
    }

    async fn get_sender_address(&self, init_code: Bytes) -> Result<Address, EntryPointError> {
        let result = self.get_sender_address(init_code).call().await;
        
        match result {
            Ok(_) => Err(EntryPointError::RevertError(format!(
                "Get sender address must revert."
            ))),
            Err(e) => {
                if let Some(revert_err) = e.as_revert() {
                    let Ok(sender_address) = SenderAddressResult::decode(revert_err) else {
                        return Err(EntryPointError::RevertError(format!(
                            "Decode sender address result error."
                        )));
                    };

                    Ok(sender_address.sender)
                } else {
                    Err(EntryPointError::RevertError(format!(
                        "Get sender address must revert."
                    )))
                }
            }
        }
    }

    async fn get_nonce(&self, address: Address) -> Result<U256, EntryPointError> {
        self.get_nonce(address, U256::from(0))
            .call()
            .await
            .map_err(|_| EntryPointError::ContractError(format!("Get sender address call failed.")))
    }
}

#[derive(Debug, Error)]
pub enum EntryPointError {
    #[error("contract error: {0}")]
    ContractError(String),

    #[error("revert error: {0}")]
    RevertError(String),
}
