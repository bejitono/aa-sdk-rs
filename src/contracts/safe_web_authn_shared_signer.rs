pub use safe_web_authn_shared_signer::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod safe_web_authn_shared_signer {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"NotDelegateCalled\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"publicKeyHash\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"P256.Verifiers\",\"name\":\"verifiers\",\"type\":\"uint176\"}],\"name\":\"SafeWebAuthnSharedSignerConfigured\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"SIGNER_SLOT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\"},{\"internalType\":\"P256.Verifiers\",\"name\":\"verifiers\",\"type\":\"uint176\"}],\"internalType\":\"struct SafeWebAuthnSharedSigner.Signer\",\"name\":\"signer\",\"type\":\"tuple\"}],\"name\":\"configure\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"getConfiguration\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\"},{\"internalType\":\"P256.Verifiers\",\"name\":\"verifiers\",\"type\":\"uint176\"}],\"internalType\":\"struct SafeWebAuthnSharedSigner.Signer\",\"name\":\"signer\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"message\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"name\":\"isValidSignature\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"magicValue\",\"type\":\"bytes4\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"name\":\"isValidSignature\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"magicValue\",\"type\":\"bytes4\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static SAFEWEBAUTHNSHAREDSIGNER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct SafeWebAuthnSharedSigner<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SafeWebAuthnSharedSigner<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SafeWebAuthnSharedSigner<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SafeWebAuthnSharedSigner<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SafeWebAuthnSharedSigner<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SafeWebAuthnSharedSigner))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SafeWebAuthnSharedSigner<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SAFEWEBAUTHNSHAREDSIGNER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `SIGNER_SLOT` (0x44b8666e) function
        pub fn signer_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([68, 184, 102, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configure` (0x0dd9692f) function
        pub fn configure(
            &self,
            signer: Signer,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 217, 105, 47], (signer,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfiguration` (0xc44b11f7) function
        pub fn get_configuration(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Signer> {
            self.0
                .method_hash([196, 75, 17, 247], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x1626ba7e) function
        pub fn is_valid_signature(
            &self,
            message: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([22, 38, 186, 126], (message, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x20c13b0b) function
        pub fn is_valid_signature_with_data(
            &self,
            data: ::ethers::core::types::Bytes,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([32, 193, 59, 11], (data, signature))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SafeWebAuthnSharedSignerConfigured` event
        pub fn safe_web_authn_shared_signer_configured_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeWebAuthnSharedSignerConfiguredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeWebAuthnSharedSignerConfiguredFilter,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SafeWebAuthnSharedSigner<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotDelegateCalled` with signature `NotDelegateCalled()` and selector `0x02eecf08`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotDelegateCalled", abi = "NotDelegateCalled()")]
    pub struct NotDelegateCalled;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "SafeWebAuthnSharedSignerConfigured",
        abi = "SafeWebAuthnSharedSignerConfigured(bytes32,uint256,uint256,uint176)"
    )]
    pub struct SafeWebAuthnSharedSignerConfiguredFilter {
        #[ethevent(indexed)]
        pub public_key_hash: [u8; 32],
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub verifiers: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `SIGNER_SLOT` function with signature `SIGNER_SLOT()` and selector `0x44b8666e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "SIGNER_SLOT", abi = "SIGNER_SLOT()")]
    pub struct SignerSlotCall;
    ///Container type for all input parameters for the `configure` function with signature `configure((uint256,uint256,uint176))` and selector `0x0dd9692f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "configure", abi = "configure((uint256,uint256,uint176))")]
    pub struct ConfigureCall {
        pub signer: Signer,
    }
    ///Container type for all input parameters for the `getConfiguration` function with signature `getConfiguration(address)` and selector `0xc44b11f7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getConfiguration", abi = "getConfiguration(address)")]
    pub struct GetConfigurationCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes32,bytes)")]
    pub struct IsValidSignatureCall {
        pub message: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `0x20c13b0b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes,bytes)")]
    pub struct IsValidSignatureWithDataCall {
        pub data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SafeWebAuthnSharedSignerCalls {
        SignerSlot(SignerSlotCall),
        Configure(ConfigureCall),
        GetConfiguration(GetConfigurationCall),
        IsValidSignature(IsValidSignatureCall),
        IsValidSignatureWithData(IsValidSignatureWithDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for SafeWebAuthnSharedSignerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SignerSlotCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SignerSlot(decoded));
            }
            if let Ok(decoded) = <ConfigureCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Configure(decoded));
            }
            if let Ok(decoded) =
                <GetConfigurationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetConfiguration(decoded));
            }
            if let Ok(decoded) =
                <IsValidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsValidSignature(decoded));
            }
            if let Ok(decoded) =
                <IsValidSignatureWithDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsValidSignatureWithData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SafeWebAuthnSharedSignerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SignerSlot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Configure(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetConfiguration(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsValidSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsValidSignatureWithData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SafeWebAuthnSharedSignerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SignerSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::Configure(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfiguration(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignatureWithData(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SignerSlotCall> for SafeWebAuthnSharedSignerCalls {
        fn from(value: SignerSlotCall) -> Self {
            Self::SignerSlot(value)
        }
    }
    impl ::core::convert::From<ConfigureCall> for SafeWebAuthnSharedSignerCalls {
        fn from(value: ConfigureCall) -> Self {
            Self::Configure(value)
        }
    }
    impl ::core::convert::From<GetConfigurationCall> for SafeWebAuthnSharedSignerCalls {
        fn from(value: GetConfigurationCall) -> Self {
            Self::GetConfiguration(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureCall> for SafeWebAuthnSharedSignerCalls {
        fn from(value: IsValidSignatureCall) -> Self {
            Self::IsValidSignature(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureWithDataCall> for SafeWebAuthnSharedSignerCalls {
        fn from(value: IsValidSignatureWithDataCall) -> Self {
            Self::IsValidSignatureWithData(value)
        }
    }
    ///Container type for all return fields from the `SIGNER_SLOT` function with signature `SIGNER_SLOT()` and selector `0x44b8666e`
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
    pub struct SignerSlotReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getConfiguration` function with signature `getConfiguration(address)` and selector `0xc44b11f7`
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
    pub struct GetConfigurationReturn {
        pub signer: Signer,
    }
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
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
    pub struct IsValidSignatureReturn {
        pub magic_value: [u8; 4],
    }
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `0x20c13b0b`
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
    pub struct IsValidSignatureWithDataReturn {
        pub magic_value: [u8; 4],
    }
    ///`Signer(uint256,uint256,uint176)`
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
    pub struct Signer {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub verifiers: ::ethers::core::types::U256,
    }
}
