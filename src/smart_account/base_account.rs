use crate::contracts::UserOperation;
use crate::types::ExecuteCall;
use async_trait::async_trait;
use ethers::providers::{Middleware, ProviderError};
use ethers::types::Chain;
use ethers::{
    providers::{JsonRpcClient, Provider},
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, TransactionRequest, U256},
};
use std::fmt::Debug;
use thiserror::Error;

use super::{utils, EntryPoint, EntryPointError, SmartAccountSigner};

#[async_trait]
pub trait BaseAccount: Sync + Send + Debug {
    type EntryPoint: EntryPoint;
    type Provider: JsonRpcClient;
    type Inner: Middleware<Provider = Self::Provider>;

    fn inner(&self) -> &Self::Inner;

    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    fn entry_point(&self) -> &Self::EntryPoint;

    fn get_chain(&self) -> Chain;

    async fn get_account_address(&self) -> Result<Address, AccountError>;

    fn get_verification_gas_limit(&self) -> U256 {
        U256::from(110000)
    }

    fn get_pre_verification_gas<U: Into<UserOperation> + Send + Sync>(&self, user_op: U) -> U256 {
        utils::calc_pre_verification_gas(user_op.into(), None)
    }

    async fn get_nonce(&self) -> Result<U256, AccountError> {
        // TODO: Use cache trait to cache nonce, address, etc. Can also initialize

        let account_address: Address = self.get_account_address().await?;

        self.entry_point()
            .get_nonce(account_address)
            .await
            .map_err(AccountError::EntryPointError)
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError>;

    async fn get_init_code(&self) -> Result<Bytes, AccountError> {
        if !self.check_is_deployed().await? {
            return self.get_account_init_code().await;
        }

        Ok(Bytes::new())
    }

    async fn is_deployed(&self) -> bool;
    async fn set_is_deployed(&self, is_deployed: bool);

    async fn check_is_deployed(&self) -> Result<bool, AccountError> {
        if self.is_deployed().await {
            return Ok(self.is_deployed().await);
        }

        let sender_address_code: Bytes = self
            .provider()
            .get_code(self.get_account_address().await?, None)
            .await
            .map_err(AccountError::ProviderError)?;

        if sender_address_code.len() > 2 {
            self.set_is_deployed(true).await;
        }

        Ok(self.is_deployed().await)
    }

    // TODO Doesn't need to be async
    async fn encode_execute(&self, call: ExecuteCall) -> Result<Vec<u8>, AccountError>;

    async fn encode_execute_batch(&self, calls: Vec<ExecuteCall>) -> Result<Vec<u8>, AccountError>;

    async fn estimate_creation_gas(&self) -> Result<U256, AccountError> {
        let init_code: Bytes = self.get_init_code().await?;

        if init_code.is_empty() {
            Ok(U256::zero())
        } else {
            let deployer_address = &init_code[0..20];
            let deployer_address = ethers::types::Address::from_slice(deployer_address);
            let deployer_call_data = &init_code[20..];

            let typed_tx: TypedTransaction = TransactionRequest::new()
                .to(deployer_address)
                .data(deployer_call_data.to_vec())
                .into();

            let gas_estimate: U256 = self
                .provider()
                .estimate_gas(&typed_tx, None)
                .await
                .map_err(AccountError::ProviderError)?;

            Ok(gas_estimate)
        }
    }

    async fn get_user_op_hash<U: Into<UserOperation> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<[u8; 32], AccountError> {
        let chain_id: U256 = self.get_chain().into();
        let entry_point_address: Address = self.entry_point().get_address();

        Ok(utils::get_user_op_hash(
            user_op.into(),
            entry_point_address,
            chain_id,
        ))
    }

    async fn get_counterfactual_address(&self) -> Result<Address, AccountError> {
        let init_code: Bytes = self.get_account_init_code().await?;

        self.entry_point()
            .get_sender_address(init_code)
            .await
            .map_err(AccountError::EntryPointError)

        // TODO: Check null address
    }

    // TODO: `Signer` produces an ECDSA signature. Will need to add our own Signer type // should be fine now
    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: [u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError>;

    // TODO: `Signer` produces an ECDSA signature. Will need to add our own Signer type
    async fn sign_user_op<U: Into<UserOperation> + Send + Sync, S: SmartAccountSigner>(
        &self,
        user_op: U,
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        let user_op_hash = self.get_user_op_hash(user_op).await?;
        let signature = self.sign_user_op_hash(user_op_hash, signer).await;

        signature
    }
}

pub struct TransactionDetailsForUserOp {
    pub target: Address,
    pub data: Vec<u8>,
    pub value: Option<U256>,
    pub gas_limit: Option<U256>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
}

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("decode error: {0}")]
    DecodeError(String),

    #[error("encode error: {0}")]
    EncodeError(String),

    #[error("revert error: {0}")]
    RevertError(String),

    #[error("contract error: {0}")]
    EntryPointError(EntryPointError),

    #[error("provider error: {0}")]
    ProviderError(ProviderError),

    #[error("nonce error")]
    NonceError,

    #[error("signer error")]
    SignerError,
}

#[cfg(test)]
mod tests {
    use crate::contracts::simple_account_factory::CreateAccountCall;

    use super::*;
    use crate::contracts::{EntryPoint as EthersEntryPoint, SimpleAccountFactoryCalls};
    use async_trait::async_trait;
    use ethers::abi::AbiEncode;
    use ethers::prelude::{Http, Provider};
    use ethers::types::{Address, Bytes, H256, U256};
    use std::sync::Arc;
    use std::{assert_eq, println};

    #[derive(Debug)]
    struct MockBaseAccount {
        inner: Arc<Provider<Http>>,
        entry_point: Arc<EthersEntryPoint<Provider<Http>>>,
    }

    #[async_trait]
    impl BaseAccount for MockBaseAccount {
        type EntryPoint = EthersEntryPoint<Provider<Http>>;
        type Provider = Http;
        type Inner = Provider<Http>;

        fn inner(&self) -> &Self::Inner {
            &self.inner
        }

        fn get_chain(&self) -> Chain {
            Chain::Mainnet
        }

        async fn get_account_address(&self) -> Result<Address, AccountError> {
            Ok("0x12fd82c9b1a44979838a19dfa5153bd093b0e75e"
                .parse()
                .unwrap())
        }

        fn entry_point(&self) -> &Self::EntryPoint {
            &self.entry_point
        }

        async fn get_account_init_code(&self) -> Result<Bytes, AccountError> {
            let factory_address: Address = "0x9406Cc6185a346906296840746125a0E44976454"
                .parse()
                .unwrap();

            let owner: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
                .parse()
                .unwrap();

            let index = U256::from(1);

            let call =
                SimpleAccountFactoryCalls::CreateAccount(CreateAccountCall { owner, salt: index });

            println!("encoded call: {:?}", call.clone().encode_hex());

            let mut result: Vec<u8> = Vec::new();

            result.extend_from_slice(factory_address.as_bytes());
            result.extend_from_slice(&call.encode());

            let result_bytes = Bytes::from(result);

            println!("encoded calldata: {:?}", result_bytes.clone().encode_hex());

            Ok(result_bytes)
        }

        async fn get_nonce(&self) -> Result<U256, AccountError> {
            unimplemented!() // You will need to provide an actual implementation.
        }

        async fn is_deployed(&self) -> bool {
            false
        }

        async fn set_is_deployed(&self, _is_deployed: bool) {}

        async fn encode_execute(&self, _call: ExecuteCall) -> Result<Vec<u8>, AccountError> {
            unimplemented!() // You will need to provide an actual implementation.
        }

        async fn encode_execute_batch(
            &self,
            _calls: Vec<ExecuteCall>,
        ) -> Result<Vec<u8>, AccountError> {
            unimplemented!() // You will need to provide an actual implementation.
        }

        async fn sign_user_op_hash<S: SmartAccountSigner>(
            &self,
            _user_op_hash: [u8; 32],
            _signer: &S,
        ) -> Result<Bytes, AccountError> {
            unimplemented!() // You will need to provide an actual implementation.
        }
    }

    impl MockBaseAccount {
        async fn get_onchain_user_op_hash(&self, user_op: UserOperation) -> [u8; 32] {
            self.entry_point()
                .get_user_op_hash(user_op.into())
                .call()
                .await
                .unwrap()
        }
    }

    const ENTRY_POINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
    const RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/lRcdJTfR_zjZSef3yutTGE6OIY9YFx1E";

    #[tokio::test]
    async fn test_get_counterfactual_address() {
        let account = make_mock_account();
        let counterfactual_address = account.get_counterfactual_address().await.unwrap();
        let expected_address: Address = "0x2beb3c8a50ac9c67833e0748d69fd1d38d1193e2"
            .parse()
            .unwrap();

        assert!(counterfactual_address == expected_address)
    }

    #[tokio::test]
    async fn test_user_op_hash() {
        let owner: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let account = make_mock_account();
        println!(
            "init code {:?}",
            account.get_account_init_code().await.unwrap()
        );
        let user_op = UserOperation {
            sender: owner,
            nonce: U256::from(1),
            init_code: account.get_account_init_code().await.unwrap(),
            call_data: Bytes::from(vec![]),
            call_gas_limit: U256::from(0),
            verification_gas_limit: U256::from(0),
            pre_verification_gas: U256::from(0),
            max_fee_per_gas: U256::from(0),
            max_priority_fee_per_gas: U256::from(0),
            paymaster_and_data: Bytes::from(vec![]),
            signature: Bytes::from(vec![]),
        };

        let onchain_hash = account.get_onchain_user_op_hash(user_op.clone()).await;
        let offchain_hash = account.get_user_op_hash(user_op.clone()).await.unwrap();

        println!("onchain {:?}", H256::from(onchain_hash));
        println!("offchain {:?}", H256::from(offchain_hash));

        assert!(onchain_hash == offchain_hash)
    }

    #[tokio::test]
    async fn test_estimate_creation_gas() {
        let account = make_mock_account();
        let creation_gas = account.estimate_creation_gas().await.unwrap();

        assert_eq!(creation_gas, U256::from(291723))
    }

    fn make_mock_account() -> MockBaseAccount {
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();
        let entry_point_address: Address = ENTRY_POINT_ADDRESS.parse().unwrap();
        let entry_point = Arc::new(EthersEntryPoint::new(
            entry_point_address,
            Arc::new(provider.clone()),
        ));

        let account = MockBaseAccount {
            inner: Arc::new(provider),
            entry_point,
        };

        account
    }
}
