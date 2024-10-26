use async_trait::async_trait;
use ethers::{
    providers::{JsonRpcClient, Middleware, ProviderError},
    types::{Bytes, U256},
    utils,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Add, str::FromStr};

use crate::types::{
    AccountCall, FromErr, UserOpHash, UserOperation, UserOperationGasEstimate,
    UserOperationReceipt, UserOperationRequest,
};

use super::{AccountError, BaseAccount, EntryPoint, SmartAccountMiddleware, SmartAccountSigner};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct SmartAccountProvider<P, A> {
    inner: P,
    account: A,
    // entry_point_address: Address,
}

impl<P, A> SmartAccountProvider<P, A>
where
    P: JsonRpcClient,
    A: BaseAccount,
{
    // TODO: Add try_from that only accepts url plus account
    pub fn new(inner: P, account: A) -> Self {
        Self { inner, account }
    }

    pub fn provider(&self) -> &P {
        &self.inner
    }
}

#[async_trait]
impl<P: JsonRpcClient, A: BaseAccount> SmartAccountMiddleware for SmartAccountProvider<P, A> {
    type Error = SmartAccountProviderError;
    type Provider = P;
    type Account = A;
    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        self
    }

    async fn send_user_operation<
        U: Into<UserOperationRequest> + Send + Sync,
        S: SmartAccountSigner,
    >(
        &self,
        user_op: U,
        // TODO: Passing in signer through method param for now. Consider separate signer middleware.
        signer: &S,
    ) -> Result<UserOpHash, SmartAccountProviderError> {
        let mut user_op: UserOperationRequest = user_op.into();
        self.fill_user_operation(&mut user_op).await?;

        if user_op.signature.is_none() {
            let signature: Bytes = self.sign_user_operation(user_op.clone(), signer).await?;
            user_op.signature = Some(signature)
        }

        let serialized_user_op = utils::serialize(&user_op);
        let serialized_entry_point_address =
            utils::serialize(&self.account.entry_point().get_address());

        self.inner()
            .provider()
            .request(
                "eth_sendUserOperation",
                [serialized_user_op, serialized_entry_point_address],
            )
            .await
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
    }

    async fn fill_user_operation(
        &self,
        user_op: &mut UserOperationRequest,
    ) -> Result<(), SmartAccountProviderError> {
        if let Some(ref call) = user_op.call {
            let call_data: Bytes = match call {
                AccountCall::Execute(execute_call) => {
                    let call_data: Vec<u8> = self
                        .account
                        .encode_execute(execute_call.clone())
                        .await
                        .map_err(SmartAccountProviderError::AccountError)?;

                    call_data.into()
                }
                AccountCall::ExecuteBatch(execute_calls) => {
                    let call_data: Vec<u8> = self
                        .account
                        .encode_execute_batch(execute_calls.clone())
                        .await
                        .map_err(SmartAccountProviderError::AccountError)?;

                    call_data.into()
                }
            };

            user_op.call_data = Some(call_data.into());
        }

        if user_op.nonce.is_none() {
            let nonce: U256 = self.account.get_nonce().await.unwrap_or(U256::from(0));
            user_op.set_nonce(nonce);
        }

        if user_op.sender.is_none() {
            user_op.sender = Some(
                self.account
                    .get_account_address()
                    .await
                    .map_err(SmartAccountProviderError::AccountError)?,
            );
        }

        if user_op.init_code.is_none() {
            user_op.init_code = Some(
                self.account
                    .get_init_code()
                    .await
                    .map_err(SmartAccountProviderError::AccountError)?,
            );
        }

        if user_op.max_fee_per_gas.is_none() || user_op.max_priority_fee_per_gas.is_none() {
            let (max_fee_per_gas, max_priority_fee_per_gas) = self
                .account
                .inner()
                .provider()
                .estimate_eip1559_fees(None)
                .await?;

            if user_op.max_priority_fee_per_gas.is_none() {
                user_op.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
            }

            if user_op.max_fee_per_gas.is_none() {
                user_op.max_fee_per_gas = Some(max_fee_per_gas);
            }
        }

        if user_op.pre_verification_gas.is_none() {
            let init_gas: U256 = self
                .account
                .estimate_creation_gas()
                .await
                .map_err(SmartAccountProviderError::AccountError)?;
            user_op.verification_gas_limit =
                Some(self.account.get_verification_gas_limit().add(init_gas));
        }

        if user_op.call_gas_limit.is_none()
            || user_op.verification_gas_limit.is_none()
            || user_op.pre_verification_gas.is_none()
        {
            let gas_estimate = self
                .estimate_user_operation_gas(&user_op.clone().with_defaults())
                .await?;
            user_op.call_gas_limit = Some(
                user_op
                    .call_gas_limit
                    .unwrap_or(gas_estimate.call_gas_limit.into()),
            );
            user_op.pre_verification_gas = Some(
                user_op
                    .pre_verification_gas
                    .unwrap_or(gas_estimate.pre_verification_gas.into()),
            );
            user_op.verification_gas_limit = Some(
                user_op
                    .verification_gas_limit
                    .unwrap_or(gas_estimate.verification_gas_limit.into()),
            );
        }

        // TODO: hard coding for now. we'll need to call alchemy_requestGasAndPaymasterAndData as gas also depends on paymaster data
        // user_op.pre_verification_gas = Some(U256::from(67076));

        if user_op.paymaster_and_data.is_none() {
            if let Ok(paymaster_and_data) = self.get_paymaster_and_data(user_op.clone()).await {
                user_op.paymaster_and_data = Some(paymaster_and_data);

                // Update pre-verification gas with the additional paymaster data.
                let pre_verification_gas = self.account.get_pre_verification_gas(user_op.clone());

                user_op.pre_verification_gas = Some(pre_verification_gas);
            } else {
                user_op.paymaster_and_data = Some(Bytes::new());
            }
        }

        Ok(())
    }

    async fn sign_user_operation<S: SmartAccountSigner>(
        &self,
        user_op: UserOperationRequest,
        // TODO: Passing in signer through method param for now.
        signer: &S,
    ) -> Result<Bytes, SmartAccountProviderError> {
        self.account
            .sign_user_op(user_op, signer)
            .await
            .map_err(SmartAccountProviderError::AccountError)
    }

    async fn estimate_user_operation_gas(
        &self,
        user_op: &UserOperationRequest,
    ) -> Result<UserOperationGasEstimate, SmartAccountProviderError> {
        let serialized_user_op = utils::serialize(user_op);
        let serialized_entry_point_address =
            utils::serialize(&self.account.entry_point().get_address());

        self.inner()
            .provider()
            .request(
                "eth_estimateUserOperationGas",
                [serialized_user_op, serialized_entry_point_address],
            )
            .await
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
    }

    async fn get_user_operation<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperation>, SmartAccountProviderError> {
        let hash = user_op_hash.into();

        self.inner()
            .provider()
            .request("eth_getUserOperationByHash", [hash])
            .await
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
    }

    async fn get_user_operation_receipt<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperationReceipt>, SmartAccountProviderError> {
        let hash = user_op_hash.into();

        self.inner()
            .provider()
            .request("eth_getUserOperationReceipt", [hash])
            .await
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
    }

    async fn get_supported_entry_points(&self) -> Result<Vec<String>, SmartAccountProviderError> {
        self.inner()
            .provider()
            .request("eth_supportedEntryPoints", ())
            .await
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
    }

    async fn get_paymaster_and_data<U: Into<UserOperationRequest> + Send + Sync>(
        &self,
        _user_op: U,
    ) -> Result<Bytes, SmartAccountProviderError> {
        Err(SmartAccountProviderError::CustomError(
            "No paymaster data defined".to_string(),
        ))
    }
}

impl<P, A> AsRef<P> for SmartAccountProvider<P, A> {
    fn as_ref(&self) -> &P {
        &self.inner
    }
}

impl FromErr<SmartAccountProviderError> for SmartAccountProviderError {
    fn from(src: SmartAccountProviderError) -> Self {
        src
    }
}

#[derive(Debug, Error)]
/// An error thrown when making a call to the provider
pub enum SmartAccountProviderError {
    /// An internal error in the JSON RPC Client
    #[error(transparent)]
    JsonRpcClientError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    AccountError(#[from] AccountError),

    #[error("error: {0}")]
    CustomError(String),
}
