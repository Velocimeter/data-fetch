pub use o_token::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod o_token {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_paymentToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_underlyingToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_pair"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPair"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gaugeFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_treasury"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_voter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_votingEscrow"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_router"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FULL_LOCK"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FULL_LOCK"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_FEES"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_FEES"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_TWAP_POINTS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_TWAP_POINTS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MINTER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MINTER_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MIN_DISCOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MIN_DISCOUNT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PAUSER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PAUSER_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("discount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("discount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exercise"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exercise"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxPaymentAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exercise"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxPaymentAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exerciseLp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exerciseLp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxPaymentAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_discount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exerciseVe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exerciseVe"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxPaymentAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gauge"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDiscountedPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDiscountedPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLockDurationForLpDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLockDurationForLpDiscount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_discount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("duration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLpDiscountedPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLpDiscountedPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_discount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getPaymentTokenAmountForExerciseLp",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPaymentTokenAmountForExerciseLp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_discount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "paymentAmountToAddLiquidity",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSlopeInterceptForLpDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSlopeInterceptForLpDiscount",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slope"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intercept"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTimeWeightedAveragePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTimeWeightedAveragePrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getVeDiscountedPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVeDiscountedPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPaused"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockDurationForMaxLpDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lockDurationForMaxLpDiscount",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockDurationForMinLpDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lockDurationForMinLpDiscount",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxLPDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxLPDiscount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minLPDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minLPDiscount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pair"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pair"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPair"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paymentToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("router"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("router"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_discount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vmFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setLockDurationForMaxLpDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setLockDurationForMaxLpDiscount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_duration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setLockDurationForMinLpDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setLockDurationForMinLpDiscount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_duration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMaxLPDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMaxLPDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_lpMaxDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMinLPDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMinLPDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_lpMinDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPairAndPaymentToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPairAndPaymentToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPair"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setRouter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRouter"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_router"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTreasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTreasury"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_treasury"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vmTreasury"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTwapPoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTwapPoints"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_twapPoints"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setVeDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setVeDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_veDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("teamFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("teamFee"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("treasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("treasury"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("twapPoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("twapPoints"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unPause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unPause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("underlyingToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("underlyingToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateGauge"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("veDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("veDiscount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vmFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vmFee"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vmTreasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vmTreasury"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("voter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("votingEscrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("votingEscrow"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Exercise"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Exercise"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paymentAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExerciseLp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExerciseLp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paymentAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExerciseVe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExerciseVe"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paymentAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nftId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PauseStateChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PauseStateChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isPaused"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("discount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newTeamFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newVMFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newGauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetLockDurationForMaxLpDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SetLockDurationForMaxLpDiscount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "lockDurationForMaxLpDiscount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetLockDurationForMinLpDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SetLockDurationForMinLpDiscount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "lockDurationForMinLpDiscount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetMaxLPDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetMaxLPDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpMaxDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetMinLPDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetMinLPDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpMinDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetPairAndPaymentToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SetPairAndPaymentToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPaymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetRouter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetRouter"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newRouter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetTreasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetTreasury"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newTreasury"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newVMTreasury"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetTwapPoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetTwapPoints"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("twapPoints"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetVeDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetVeDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("veDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_IncorrectPairToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_IncorrectPairToken",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_InvalidDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_InvalidDiscount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_InvalidFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_InvalidFee",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_InvalidLockDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_InvalidLockDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_InvalidTwapPoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_InvalidTwapPoints",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_NoAdminRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_NoAdminRole",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_NoMinterRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_NoMinterRole",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_NoPauserRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_NoPauserRole",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_PastDeadline"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_PastDeadline",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OptionToken_Paused"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptionToken_SlippageTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptionToken_SlippageTooHigh",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OTOKEN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct oTOKEN<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for oTOKEN<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for oTOKEN<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for oTOKEN<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for oTOKEN<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(oTOKEN)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> oTOKEN<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OTOKEN_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `ADMIN_ROLE` (0x75b238fc) function
        pub fn admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([117, 178, 56, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FULL_LOCK` (0x54cb0384) function
        pub fn full_lock(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 203, 3, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_DISCOUNT` (0xe3495569) function
        pub fn max_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 73, 85, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_FEES` (0xc2300bef) function
        pub fn max_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([194, 48, 11, 239], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_TWAP_POINTS` (0x881326b3) function
        pub fn max_twap_points(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([136, 19, 38, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINTER_ROLE` (0xd5391393) function
        pub fn minter_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 57, 19, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MIN_DISCOUNT` (0x2ac8a92c) function
        pub fn min_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([42, 200, 169, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PAUSER_ROLE` (0xe63ab1e9) function
        pub fn pauser_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 58, 177, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `discount` (0x6b6f4a9d) function
        pub fn discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 111, 74, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exercise` (0xa1d50c3a) function
        pub fn exercise_with_amount_and_max_payment_amount_and_deadline(
            &self,
            amount: ::ethers::core::types::U256,
            max_payment_amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [161, 213, 12, 58],
                    (amount, max_payment_amount, recipient, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exercise` (0xd6379b72) function
        pub fn exercise(
            &self,
            amount: ::ethers::core::types::U256,
            max_payment_amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [214, 55, 155, 114],
                    (amount, max_payment_amount, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exerciseLp` (0xf897bd1d) function
        pub fn exercise_lp(
            &self,
            amount: ::ethers::core::types::U256,
            max_payment_amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            discount: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [248, 151, 189, 29],
                    (amount, max_payment_amount, recipient, discount, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exerciseVe` (0x62994c05) function
        pub fn exercise_ve(
            &self,
            amount: ::ethers::core::types::U256,
            max_payment_amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [98, 153, 76, 5],
                    (amount, max_payment_amount, recipient, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauge` (0xa6f19c84) function
        pub fn gauge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([166, 241, 156, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDiscountedPrice` (0x339ccade) function
        pub fn get_discounted_price(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([51, 156, 202, 222], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLockDurationForLpDiscount` (0x787dd9e4) function
        pub fn get_lock_duration_for_lp_discount(
            &self,
            discount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 125, 217, 228], discount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLpDiscountedPrice` (0xc7d1e395) function
        pub fn get_lp_discounted_price(
            &self,
            amount: ::ethers::core::types::U256,
            discount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([199, 209, 227, 149], (amount, discount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPaymentTokenAmountForExerciseLp` (0x9e66645b) function
        pub fn get_payment_token_amount_for_exercise_lp(
            &self,
            amount: ::ethers::core::types::U256,
            discount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([158, 102, 100, 91], (amount, discount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlopeInterceptForLpDiscount` (0x3cdfed56) function
        pub fn get_slope_intercept_for_lp_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash([60, 223, 237, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTimeWeightedAveragePrice` (0xe8772bb2) function
        pub fn get_time_weighted_average_price(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 119, 43, 178], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVeDiscountedPrice` (0x6e180f6a) function
        pub fn get_ve_discounted_price(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 24, 15, 106], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPaused` (0xb187bd26) function
        pub fn is_paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 135, 189, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockDurationForMaxLpDiscount` (0x1adb040a) function
        pub fn lock_duration_for_max_lp_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([26, 219, 4, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockDurationForMinLpDiscount` (0xb3427576) function
        pub fn lock_duration_for_min_lp_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 66, 117, 118], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxLPDiscount` (0x062e5bbe) function
        pub fn max_lp_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([6, 46, 91, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minLPDiscount` (0x860ca9bb) function
        pub fn min_lp_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([134, 12, 169, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pair` (0xa8aa1b31) function
        pub fn pair(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 170, 27, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paymentToken` (0x3013ce29) function
        pub fn payment_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([48, 19, 206, 41], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `router` (0xf887ea40) function
        pub fn router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 135, 234, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDiscount` (0xdabd2719) function
        pub fn set_discount(
            &self,
            discount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 189, 39, 25], discount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFees` (0x0b78f9c0) function
        pub fn set_fees(
            &self,
            fee: ::ethers::core::types::U256,
            vm_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 120, 249, 192], (fee, vm_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGauge` (0x55a68ed3) function
        pub fn set_gauge(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 166, 142, 211], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLockDurationForMaxLpDiscount` (0x0694d1f1) function
        pub fn set_lock_duration_for_max_lp_discount(
            &self,
            duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 148, 209, 241], duration)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLockDurationForMinLpDiscount` (0x2fc1b057) function
        pub fn set_lock_duration_for_min_lp_discount(
            &self,
            duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 193, 176, 87], duration)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxLPDiscount` (0xf595c5ad) function
        pub fn set_max_lp_discount(
            &self,
            lp_max_discount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 149, 197, 173], lp_max_discount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinLPDiscount` (0x50a566c5) function
        pub fn set_min_lp_discount(
            &self,
            lp_min_discount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 165, 102, 197], lp_min_discount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPairAndPaymentToken` (0xfb1d6bd9) function
        pub fn set_pair_and_payment_token(
            &self,
            pair: ::ethers::core::types::Address,
            payment_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 29, 107, 217], (pair, payment_token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRouter` (0xc0d78655) function
        pub fn set_router(
            &self,
            router: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 215, 134, 85], router)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTreasury` (0xadfe0870) function
        pub fn set_treasury(
            &self,
            treasury: ::ethers::core::types::Address,
            vm_treasury: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 254, 8, 112], (treasury, vm_treasury))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTwapPoints` (0xfa4473b3) function
        pub fn set_twap_points(
            &self,
            twap_points: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 68, 115, 179], twap_points)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVeDiscount` (0xde87db2f) function
        pub fn set_ve_discount(
            &self,
            ve_discount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 135, 219, 47], ve_discount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `teamFee` (0xd7c94efd) function
        pub fn team_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 201, 78, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `treasury` (0x61d027b3) function
        pub fn treasury(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([97, 208, 39, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `twapPoints` (0x5d283021) function
        pub fn twap_points(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([93, 40, 48, 33], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unPause` (0xf7b188a5) function
        pub fn un_pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 177, 136, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingToken` (0x2495a599) function
        pub fn underlying_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([36, 149, 165, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateGauge` (0x79c79707) function
        pub fn update_gauge(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 199, 151, 7], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `veDiscount` (0x51217cbe) function
        pub fn ve_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([81, 33, 124, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vmFee` (0xff26b1c8) function
        pub fn vm_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([255, 38, 177, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vmTreasury` (0x20d1b665) function
        pub fn vm_treasury(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([32, 209, 182, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voter` (0x46c96aac) function
        pub fn voter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 201, 106, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingEscrow` (0x4f2bfe5b) function
        pub fn voting_escrow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([79, 43, 254, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Exercise` event
        pub fn exercise_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExerciseFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExerciseLp` event
        pub fn exercise_lp_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExerciseLpFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExerciseVe` event
        pub fn exercise_ve_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExerciseVeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PauseStateChanged` event
        pub fn pause_state_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauseStateChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetDiscount` event
        pub fn set_discount_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetDiscountFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetFees` event
        pub fn set_fees_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetFeesFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetGauge` event
        pub fn set_gauge_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetGaugeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetLockDurationForMaxLpDiscount` event
        pub fn set_lock_duration_for_max_lp_discount_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetLockDurationForMaxLpDiscountFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetLockDurationForMinLpDiscount` event
        pub fn set_lock_duration_for_min_lp_discount_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetLockDurationForMinLpDiscountFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetMaxLPDiscount` event
        pub fn set_max_lp_discount_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetMaxLPDiscountFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetMinLPDiscount` event
        pub fn set_min_lp_discount_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetMinLPDiscountFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetPairAndPaymentToken` event
        pub fn set_pair_and_payment_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetPairAndPaymentTokenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetRouter` event
        pub fn set_router_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetRouterFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetTreasury` event
        pub fn set_treasury_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetTreasuryFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetTwapPoints` event
        pub fn set_twap_points_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetTwapPointsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetVeDiscount` event
        pub fn set_ve_discount_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetVeDiscountFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, oTOKENEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for oTOKEN<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `OptionToken_IncorrectPairToken` with signature `OptionToken_IncorrectPairToken()` and selector `0xa818b0ad`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OptionToken_IncorrectPairToken",
        abi = "OptionToken_IncorrectPairToken()"
    )]
    pub struct OptionToken_IncorrectPairToken;
    ///Custom Error type `OptionToken_InvalidDiscount` with signature `OptionToken_InvalidDiscount()` and selector `0x4a5f22d0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OptionToken_InvalidDiscount",
        abi = "OptionToken_InvalidDiscount()"
    )]
    pub struct OptionToken_InvalidDiscount;
    ///Custom Error type `OptionToken_InvalidFee` with signature `OptionToken_InvalidFee()` and selector `0x05da7a82`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OptionToken_InvalidFee", abi = "OptionToken_InvalidFee()")]
    pub struct OptionToken_InvalidFee;
    ///Custom Error type `OptionToken_InvalidLockDuration` with signature `OptionToken_InvalidLockDuration()` and selector `0x4dd99fe7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OptionToken_InvalidLockDuration",
        abi = "OptionToken_InvalidLockDuration()"
    )]
    pub struct OptionToken_InvalidLockDuration;
    ///Custom Error type `OptionToken_InvalidTwapPoints` with signature `OptionToken_InvalidTwapPoints()` and selector `0x0ac04085`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OptionToken_InvalidTwapPoints",
        abi = "OptionToken_InvalidTwapPoints()"
    )]
    pub struct OptionToken_InvalidTwapPoints;
    ///Custom Error type `OptionToken_NoAdminRole` with signature `OptionToken_NoAdminRole()` and selector `0xf982dd0f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OptionToken_NoAdminRole", abi = "OptionToken_NoAdminRole()")]
    pub struct OptionToken_NoAdminRole;
    ///Custom Error type `OptionToken_NoMinterRole` with signature `OptionToken_NoMinterRole()` and selector `0x4fcb6d01`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OptionToken_NoMinterRole", abi = "OptionToken_NoMinterRole()")]
    pub struct OptionToken_NoMinterRole;
    ///Custom Error type `OptionToken_NoPauserRole` with signature `OptionToken_NoPauserRole()` and selector `0xb1c851f8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OptionToken_NoPauserRole", abi = "OptionToken_NoPauserRole()")]
    pub struct OptionToken_NoPauserRole;
    ///Custom Error type `OptionToken_PastDeadline` with signature `OptionToken_PastDeadline()` and selector `0x5aac6262`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OptionToken_PastDeadline", abi = "OptionToken_PastDeadline()")]
    pub struct OptionToken_PastDeadline;
    ///Custom Error type `OptionToken_Paused` with signature `OptionToken_Paused()` and selector `0x00b4aa37`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OptionToken_Paused", abi = "OptionToken_Paused()")]
    pub struct OptionToken_Paused;
    ///Custom Error type `OptionToken_SlippageTooHigh` with signature `OptionToken_SlippageTooHigh()` and selector `0x8e921434`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OptionToken_SlippageTooHigh",
        abi = "OptionToken_SlippageTooHigh()"
    )]
    pub struct OptionToken_SlippageTooHigh;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum oTOKENErrors {
        OptionToken_IncorrectPairToken(OptionToken_IncorrectPairToken),
        OptionToken_InvalidDiscount(OptionToken_InvalidDiscount),
        OptionToken_InvalidFee(OptionToken_InvalidFee),
        OptionToken_InvalidLockDuration(OptionToken_InvalidLockDuration),
        OptionToken_InvalidTwapPoints(OptionToken_InvalidTwapPoints),
        OptionToken_NoAdminRole(OptionToken_NoAdminRole),
        OptionToken_NoMinterRole(OptionToken_NoMinterRole),
        OptionToken_NoPauserRole(OptionToken_NoPauserRole),
        OptionToken_PastDeadline(OptionToken_PastDeadline),
        OptionToken_Paused(OptionToken_Paused),
        OptionToken_SlippageTooHigh(OptionToken_SlippageTooHigh),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for oTOKENErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_IncorrectPairToken as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_IncorrectPairToken(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_InvalidDiscount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_InvalidDiscount(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_InvalidFee as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_InvalidFee(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_InvalidLockDuration as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_InvalidLockDuration(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_InvalidTwapPoints as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_InvalidTwapPoints(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_NoAdminRole as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_NoAdminRole(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_NoMinterRole as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_NoMinterRole(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_NoPauserRole as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_NoPauserRole(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_PastDeadline as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_PastDeadline(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_Paused as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OptionToken_Paused(decoded));
            }
            if let Ok(decoded)
                = <OptionToken_SlippageTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptionToken_SlippageTooHigh(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for oTOKENErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::OptionToken_IncorrectPairToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_InvalidDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_InvalidFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_InvalidLockDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_InvalidTwapPoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_NoAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_NoMinterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_NoPauserRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_PastDeadline(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_Paused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptionToken_SlippageTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for oTOKENErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <OptionToken_IncorrectPairToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_InvalidDiscount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_InvalidFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_InvalidLockDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_InvalidTwapPoints as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_NoAdminRole as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_NoMinterRole as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_NoPauserRole as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_PastDeadline as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_Paused as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OptionToken_SlippageTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for oTOKENErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OptionToken_IncorrectPairToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_InvalidDiscount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_InvalidFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_InvalidLockDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_InvalidTwapPoints(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_NoAdminRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_NoMinterRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_NoPauserRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_PastDeadline(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_Paused(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptionToken_SlippageTooHigh(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for oTOKENErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<OptionToken_IncorrectPairToken> for oTOKENErrors {
        fn from(value: OptionToken_IncorrectPairToken) -> Self {
            Self::OptionToken_IncorrectPairToken(value)
        }
    }
    impl ::core::convert::From<OptionToken_InvalidDiscount> for oTOKENErrors {
        fn from(value: OptionToken_InvalidDiscount) -> Self {
            Self::OptionToken_InvalidDiscount(value)
        }
    }
    impl ::core::convert::From<OptionToken_InvalidFee> for oTOKENErrors {
        fn from(value: OptionToken_InvalidFee) -> Self {
            Self::OptionToken_InvalidFee(value)
        }
    }
    impl ::core::convert::From<OptionToken_InvalidLockDuration> for oTOKENErrors {
        fn from(value: OptionToken_InvalidLockDuration) -> Self {
            Self::OptionToken_InvalidLockDuration(value)
        }
    }
    impl ::core::convert::From<OptionToken_InvalidTwapPoints> for oTOKENErrors {
        fn from(value: OptionToken_InvalidTwapPoints) -> Self {
            Self::OptionToken_InvalidTwapPoints(value)
        }
    }
    impl ::core::convert::From<OptionToken_NoAdminRole> for oTOKENErrors {
        fn from(value: OptionToken_NoAdminRole) -> Self {
            Self::OptionToken_NoAdminRole(value)
        }
    }
    impl ::core::convert::From<OptionToken_NoMinterRole> for oTOKENErrors {
        fn from(value: OptionToken_NoMinterRole) -> Self {
            Self::OptionToken_NoMinterRole(value)
        }
    }
    impl ::core::convert::From<OptionToken_NoPauserRole> for oTOKENErrors {
        fn from(value: OptionToken_NoPauserRole) -> Self {
            Self::OptionToken_NoPauserRole(value)
        }
    }
    impl ::core::convert::From<OptionToken_PastDeadline> for oTOKENErrors {
        fn from(value: OptionToken_PastDeadline) -> Self {
            Self::OptionToken_PastDeadline(value)
        }
    }
    impl ::core::convert::From<OptionToken_Paused> for oTOKENErrors {
        fn from(value: OptionToken_Paused) -> Self {
            Self::OptionToken_Paused(value)
        }
    }
    impl ::core::convert::From<OptionToken_SlippageTooHigh> for oTOKENErrors {
        fn from(value: OptionToken_SlippageTooHigh) -> Self {
            Self::OptionToken_SlippageTooHigh(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Exercise", abi = "Exercise(address,address,uint256,uint256)")]
    pub struct ExerciseFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub payment_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ExerciseLp",
        abi = "ExerciseLp(address,address,uint256,uint256,uint256)"
    )]
    pub struct ExerciseLpFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub payment_amount: ::ethers::core::types::U256,
        pub lp_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ExerciseVe",
        abi = "ExerciseVe(address,address,uint256,uint256,uint256)"
    )]
    pub struct ExerciseVeFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub payment_amount: ::ethers::core::types::U256,
        pub nft_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PauseStateChanged", abi = "PauseStateChanged(bool)")]
    pub struct PauseStateChangedFilter {
        pub is_paused: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetDiscount", abi = "SetDiscount(uint256)")]
    pub struct SetDiscountFilter {
        pub discount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetFees", abi = "SetFees(uint256,uint256)")]
    pub struct SetFeesFilter {
        pub new_team_fee: ::ethers::core::types::U256,
        pub new_vm_fee: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetGauge", abi = "SetGauge(address)")]
    pub struct SetGaugeFilter {
        #[ethevent(indexed)]
        pub new_gauge: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "SetLockDurationForMaxLpDiscount",
        abi = "SetLockDurationForMaxLpDiscount(uint256)"
    )]
    pub struct SetLockDurationForMaxLpDiscountFilter {
        pub lock_duration_for_max_lp_discount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "SetLockDurationForMinLpDiscount",
        abi = "SetLockDurationForMinLpDiscount(uint256)"
    )]
    pub struct SetLockDurationForMinLpDiscountFilter {
        pub lock_duration_for_min_lp_discount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetMaxLPDiscount", abi = "SetMaxLPDiscount(uint256)")]
    pub struct SetMaxLPDiscountFilter {
        pub lp_max_discount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetMinLPDiscount", abi = "SetMinLPDiscount(uint256)")]
    pub struct SetMinLPDiscountFilter {
        pub lp_min_discount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "SetPairAndPaymentToken",
        abi = "SetPairAndPaymentToken(address,address)"
    )]
    pub struct SetPairAndPaymentTokenFilter {
        #[ethevent(indexed)]
        pub new_pair: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_payment_token: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetRouter", abi = "SetRouter(address)")]
    pub struct SetRouterFilter {
        #[ethevent(indexed)]
        pub new_router: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetTreasury", abi = "SetTreasury(address,address)")]
    pub struct SetTreasuryFilter {
        #[ethevent(indexed)]
        pub new_treasury: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_vm_treasury: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetTwapPoints", abi = "SetTwapPoints(uint256)")]
    pub struct SetTwapPointsFilter {
        pub twap_points: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetVeDiscount", abi = "SetVeDiscount(uint256)")]
    pub struct SetVeDiscountFilter {
        pub ve_discount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum oTOKENEvents {
        ApprovalFilter(ApprovalFilter),
        ExerciseFilter(ExerciseFilter),
        ExerciseLpFilter(ExerciseLpFilter),
        ExerciseVeFilter(ExerciseVeFilter),
        PauseStateChangedFilter(PauseStateChangedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        SetDiscountFilter(SetDiscountFilter),
        SetFeesFilter(SetFeesFilter),
        SetGaugeFilter(SetGaugeFilter),
        SetLockDurationForMaxLpDiscountFilter(SetLockDurationForMaxLpDiscountFilter),
        SetLockDurationForMinLpDiscountFilter(SetLockDurationForMinLpDiscountFilter),
        SetMaxLPDiscountFilter(SetMaxLPDiscountFilter),
        SetMinLPDiscountFilter(SetMinLPDiscountFilter),
        SetPairAndPaymentTokenFilter(SetPairAndPaymentTokenFilter),
        SetRouterFilter(SetRouterFilter),
        SetTreasuryFilter(SetTreasuryFilter),
        SetTwapPointsFilter(SetTwapPointsFilter),
        SetVeDiscountFilter(SetVeDiscountFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for oTOKENEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(oTOKENEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ExerciseFilter::decode_log(log) {
                return Ok(oTOKENEvents::ExerciseFilter(decoded));
            }
            if let Ok(decoded) = ExerciseLpFilter::decode_log(log) {
                return Ok(oTOKENEvents::ExerciseLpFilter(decoded));
            }
            if let Ok(decoded) = ExerciseVeFilter::decode_log(log) {
                return Ok(oTOKENEvents::ExerciseVeFilter(decoded));
            }
            if let Ok(decoded) = PauseStateChangedFilter::decode_log(log) {
                return Ok(oTOKENEvents::PauseStateChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(oTOKENEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(oTOKENEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(oTOKENEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = SetDiscountFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetDiscountFilter(decoded));
            }
            if let Ok(decoded) = SetFeesFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetFeesFilter(decoded));
            }
            if let Ok(decoded) = SetGaugeFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetGaugeFilter(decoded));
            }
            if let Ok(decoded) = SetLockDurationForMaxLpDiscountFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetLockDurationForMaxLpDiscountFilter(decoded));
            }
            if let Ok(decoded) = SetLockDurationForMinLpDiscountFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetLockDurationForMinLpDiscountFilter(decoded));
            }
            if let Ok(decoded) = SetMaxLPDiscountFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetMaxLPDiscountFilter(decoded));
            }
            if let Ok(decoded) = SetMinLPDiscountFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetMinLPDiscountFilter(decoded));
            }
            if let Ok(decoded) = SetPairAndPaymentTokenFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetPairAndPaymentTokenFilter(decoded));
            }
            if let Ok(decoded) = SetRouterFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetRouterFilter(decoded));
            }
            if let Ok(decoded) = SetTreasuryFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetTreasuryFilter(decoded));
            }
            if let Ok(decoded) = SetTwapPointsFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetTwapPointsFilter(decoded));
            }
            if let Ok(decoded) = SetVeDiscountFilter::decode_log(log) {
                return Ok(oTOKENEvents::SetVeDiscountFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(oTOKENEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for oTOKENEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExerciseFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExerciseLpFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExerciseVeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseStateChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDiscountFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGaugeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLockDurationForMaxLpDiscountFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLockDurationForMinLpDiscountFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxLPDiscountFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMinLPDiscountFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPairAndPaymentTokenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRouterFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTreasuryFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTwapPointsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetVeDiscountFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for oTOKENEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ExerciseFilter> for oTOKENEvents {
        fn from(value: ExerciseFilter) -> Self {
            Self::ExerciseFilter(value)
        }
    }
    impl ::core::convert::From<ExerciseLpFilter> for oTOKENEvents {
        fn from(value: ExerciseLpFilter) -> Self {
            Self::ExerciseLpFilter(value)
        }
    }
    impl ::core::convert::From<ExerciseVeFilter> for oTOKENEvents {
        fn from(value: ExerciseVeFilter) -> Self {
            Self::ExerciseVeFilter(value)
        }
    }
    impl ::core::convert::From<PauseStateChangedFilter> for oTOKENEvents {
        fn from(value: PauseStateChangedFilter) -> Self {
            Self::PauseStateChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for oTOKENEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for oTOKENEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for oTOKENEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<SetDiscountFilter> for oTOKENEvents {
        fn from(value: SetDiscountFilter) -> Self {
            Self::SetDiscountFilter(value)
        }
    }
    impl ::core::convert::From<SetFeesFilter> for oTOKENEvents {
        fn from(value: SetFeesFilter) -> Self {
            Self::SetFeesFilter(value)
        }
    }
    impl ::core::convert::From<SetGaugeFilter> for oTOKENEvents {
        fn from(value: SetGaugeFilter) -> Self {
            Self::SetGaugeFilter(value)
        }
    }
    impl ::core::convert::From<SetLockDurationForMaxLpDiscountFilter> for oTOKENEvents {
        fn from(value: SetLockDurationForMaxLpDiscountFilter) -> Self {
            Self::SetLockDurationForMaxLpDiscountFilter(value)
        }
    }
    impl ::core::convert::From<SetLockDurationForMinLpDiscountFilter> for oTOKENEvents {
        fn from(value: SetLockDurationForMinLpDiscountFilter) -> Self {
            Self::SetLockDurationForMinLpDiscountFilter(value)
        }
    }
    impl ::core::convert::From<SetMaxLPDiscountFilter> for oTOKENEvents {
        fn from(value: SetMaxLPDiscountFilter) -> Self {
            Self::SetMaxLPDiscountFilter(value)
        }
    }
    impl ::core::convert::From<SetMinLPDiscountFilter> for oTOKENEvents {
        fn from(value: SetMinLPDiscountFilter) -> Self {
            Self::SetMinLPDiscountFilter(value)
        }
    }
    impl ::core::convert::From<SetPairAndPaymentTokenFilter> for oTOKENEvents {
        fn from(value: SetPairAndPaymentTokenFilter) -> Self {
            Self::SetPairAndPaymentTokenFilter(value)
        }
    }
    impl ::core::convert::From<SetRouterFilter> for oTOKENEvents {
        fn from(value: SetRouterFilter) -> Self {
            Self::SetRouterFilter(value)
        }
    }
    impl ::core::convert::From<SetTreasuryFilter> for oTOKENEvents {
        fn from(value: SetTreasuryFilter) -> Self {
            Self::SetTreasuryFilter(value)
        }
    }
    impl ::core::convert::From<SetTwapPointsFilter> for oTOKENEvents {
        fn from(value: SetTwapPointsFilter) -> Self {
            Self::SetTwapPointsFilter(value)
        }
    }
    impl ::core::convert::From<SetVeDiscountFilter> for oTOKENEvents {
        fn from(value: SetVeDiscountFilter) -> Self {
            Self::SetVeDiscountFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for oTOKENEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `ADMIN_ROLE` function with signature `ADMIN_ROLE()` and selector `0x75b238fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ADMIN_ROLE", abi = "ADMIN_ROLE()")]
    pub struct AdminRoleCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `FULL_LOCK` function with signature `FULL_LOCK()` and selector `0x54cb0384`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "FULL_LOCK", abi = "FULL_LOCK()")]
    pub struct FullLockCall;
    ///Container type for all input parameters for the `MAX_DISCOUNT` function with signature `MAX_DISCOUNT()` and selector `0xe3495569`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "MAX_DISCOUNT", abi = "MAX_DISCOUNT()")]
    pub struct MaxDiscountCall;
    ///Container type for all input parameters for the `MAX_FEES` function with signature `MAX_FEES()` and selector `0xc2300bef`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "MAX_FEES", abi = "MAX_FEES()")]
    pub struct MaxFeesCall;
    ///Container type for all input parameters for the `MAX_TWAP_POINTS` function with signature `MAX_TWAP_POINTS()` and selector `0x881326b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "MAX_TWAP_POINTS", abi = "MAX_TWAP_POINTS()")]
    pub struct MaxTwapPointsCall;
    ///Container type for all input parameters for the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `0xd5391393`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "MINTER_ROLE", abi = "MINTER_ROLE()")]
    pub struct MinterRoleCall;
    ///Container type for all input parameters for the `MIN_DISCOUNT` function with signature `MIN_DISCOUNT()` and selector `0x2ac8a92c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "MIN_DISCOUNT", abi = "MIN_DISCOUNT()")]
    pub struct MinDiscountCall;
    ///Container type for all input parameters for the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "PAUSER_ROLE", abi = "PAUSER_ROLE()")]
    pub struct PauserRoleCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `discount` function with signature `discount()` and selector `0x6b6f4a9d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "discount", abi = "discount()")]
    pub struct DiscountCall;
    ///Container type for all input parameters for the `exercise` function with signature `exercise(uint256,uint256,address,uint256)` and selector `0xa1d50c3a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "exercise", abi = "exercise(uint256,uint256,address,uint256)")]
    pub struct ExerciseWithAmountAndMaxPaymentAmountAndDeadlineCall {
        pub amount: ::ethers::core::types::U256,
        pub max_payment_amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `exercise` function with signature `exercise(uint256,uint256,address)` and selector `0xd6379b72`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "exercise", abi = "exercise(uint256,uint256,address)")]
    pub struct ExerciseCall {
        pub amount: ::ethers::core::types::U256,
        pub max_payment_amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `exerciseLp` function with signature `exerciseLp(uint256,uint256,address,uint256,uint256)` and selector `0xf897bd1d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "exerciseLp",
        abi = "exerciseLp(uint256,uint256,address,uint256,uint256)"
    )]
    pub struct ExerciseLpCall {
        pub amount: ::ethers::core::types::U256,
        pub max_payment_amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub discount: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `exerciseVe` function with signature `exerciseVe(uint256,uint256,address,uint256)` and selector `0x62994c05`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "exerciseVe", abi = "exerciseVe(uint256,uint256,address,uint256)")]
    pub struct ExerciseVeCall {
        pub amount: ::ethers::core::types::U256,
        pub max_payment_amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `gauge` function with signature `gauge()` and selector `0xa6f19c84`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gauge", abi = "gauge()")]
    pub struct GaugeCall;
    ///Container type for all input parameters for the `getDiscountedPrice` function with signature `getDiscountedPrice(uint256)` and selector `0x339ccade`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getDiscountedPrice", abi = "getDiscountedPrice(uint256)")]
    pub struct GetDiscountedPriceCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLockDurationForLpDiscount` function with signature `getLockDurationForLpDiscount(uint256)` and selector `0x787dd9e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getLockDurationForLpDiscount",
        abi = "getLockDurationForLpDiscount(uint256)"
    )]
    pub struct GetLockDurationForLpDiscountCall {
        pub discount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLpDiscountedPrice` function with signature `getLpDiscountedPrice(uint256,uint256)` and selector `0xc7d1e395`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getLpDiscountedPrice",
        abi = "getLpDiscountedPrice(uint256,uint256)"
    )]
    pub struct GetLpDiscountedPriceCall {
        pub amount: ::ethers::core::types::U256,
        pub discount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPaymentTokenAmountForExerciseLp` function with signature `getPaymentTokenAmountForExerciseLp(uint256,uint256)` and selector `0x9e66645b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getPaymentTokenAmountForExerciseLp",
        abi = "getPaymentTokenAmountForExerciseLp(uint256,uint256)"
    )]
    pub struct GetPaymentTokenAmountForExerciseLpCall {
        pub amount: ::ethers::core::types::U256,
        pub discount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getSlopeInterceptForLpDiscount` function with signature `getSlopeInterceptForLpDiscount()` and selector `0x3cdfed56`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getSlopeInterceptForLpDiscount",
        abi = "getSlopeInterceptForLpDiscount()"
    )]
    pub struct GetSlopeInterceptForLpDiscountCall;
    ///Container type for all input parameters for the `getTimeWeightedAveragePrice` function with signature `getTimeWeightedAveragePrice(uint256)` and selector `0xe8772bb2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getTimeWeightedAveragePrice",
        abi = "getTimeWeightedAveragePrice(uint256)"
    )]
    pub struct GetTimeWeightedAveragePriceCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getVeDiscountedPrice` function with signature `getVeDiscountedPrice(uint256)` and selector `0x6e180f6a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getVeDiscountedPrice", abi = "getVeDiscountedPrice(uint256)")]
    pub struct GetVeDiscountedPriceCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPaused` function with signature `isPaused()` and selector `0xb187bd26`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isPaused", abi = "isPaused()")]
    pub struct IsPausedCall;
    ///Container type for all input parameters for the `lockDurationForMaxLpDiscount` function with signature `lockDurationForMaxLpDiscount()` and selector `0x1adb040a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "lockDurationForMaxLpDiscount",
        abi = "lockDurationForMaxLpDiscount()"
    )]
    pub struct LockDurationForMaxLpDiscountCall;
    ///Container type for all input parameters for the `lockDurationForMinLpDiscount` function with signature `lockDurationForMinLpDiscount()` and selector `0xb3427576`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "lockDurationForMinLpDiscount",
        abi = "lockDurationForMinLpDiscount()"
    )]
    pub struct LockDurationForMinLpDiscountCall;
    ///Container type for all input parameters for the `maxLPDiscount` function with signature `maxLPDiscount()` and selector `0x062e5bbe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maxLPDiscount", abi = "maxLPDiscount()")]
    pub struct MaxLPDiscountCall;
    ///Container type for all input parameters for the `minLPDiscount` function with signature `minLPDiscount()` and selector `0x860ca9bb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "minLPDiscount", abi = "minLPDiscount()")]
    pub struct MinLPDiscountCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `pair` function with signature `pair()` and selector `0xa8aa1b31`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pair", abi = "pair()")]
    pub struct PairCall;
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paymentToken` function with signature `paymentToken()` and selector `0x3013ce29`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "paymentToken", abi = "paymentToken()")]
    pub struct PaymentTokenCall;
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `router` function with signature `router()` and selector `0xf887ea40`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "router", abi = "router()")]
    pub struct RouterCall;
    ///Container type for all input parameters for the `setDiscount` function with signature `setDiscount(uint256)` and selector `0xdabd2719`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setDiscount", abi = "setDiscount(uint256)")]
    pub struct SetDiscountCall {
        pub discount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFees` function with signature `setFees(uint256,uint256)` and selector `0x0b78f9c0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFees", abi = "setFees(uint256,uint256)")]
    pub struct SetFeesCall {
        pub fee: ::ethers::core::types::U256,
        pub vm_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setGauge` function with signature `setGauge(address)` and selector `0x55a68ed3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setGauge", abi = "setGauge(address)")]
    pub struct SetGaugeCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setLockDurationForMaxLpDiscount` function with signature `setLockDurationForMaxLpDiscount(uint256)` and selector `0x0694d1f1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setLockDurationForMaxLpDiscount",
        abi = "setLockDurationForMaxLpDiscount(uint256)"
    )]
    pub struct SetLockDurationForMaxLpDiscountCall {
        pub duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setLockDurationForMinLpDiscount` function with signature `setLockDurationForMinLpDiscount(uint256)` and selector `0x2fc1b057`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setLockDurationForMinLpDiscount",
        abi = "setLockDurationForMinLpDiscount(uint256)"
    )]
    pub struct SetLockDurationForMinLpDiscountCall {
        pub duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMaxLPDiscount` function with signature `setMaxLPDiscount(uint256)` and selector `0xf595c5ad`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setMaxLPDiscount", abi = "setMaxLPDiscount(uint256)")]
    pub struct SetMaxLPDiscountCall {
        pub lp_max_discount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMinLPDiscount` function with signature `setMinLPDiscount(uint256)` and selector `0x50a566c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setMinLPDiscount", abi = "setMinLPDiscount(uint256)")]
    pub struct SetMinLPDiscountCall {
        pub lp_min_discount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPairAndPaymentToken` function with signature `setPairAndPaymentToken(address,address)` and selector `0xfb1d6bd9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setPairAndPaymentToken",
        abi = "setPairAndPaymentToken(address,address)"
    )]
    pub struct SetPairAndPaymentTokenCall {
        pub pair: ::ethers::core::types::Address,
        pub payment_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRouter` function with signature `setRouter(address)` and selector `0xc0d78655`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setRouter", abi = "setRouter(address)")]
    pub struct SetRouterCall {
        pub router: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTreasury` function with signature `setTreasury(address,address)` and selector `0xadfe0870`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setTreasury", abi = "setTreasury(address,address)")]
    pub struct SetTreasuryCall {
        pub treasury: ::ethers::core::types::Address,
        pub vm_treasury: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTwapPoints` function with signature `setTwapPoints(uint256)` and selector `0xfa4473b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setTwapPoints", abi = "setTwapPoints(uint256)")]
    pub struct SetTwapPointsCall {
        pub twap_points: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setVeDiscount` function with signature `setVeDiscount(uint256)` and selector `0xde87db2f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setVeDiscount", abi = "setVeDiscount(uint256)")]
    pub struct SetVeDiscountCall {
        pub ve_discount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `teamFee` function with signature `teamFee()` and selector `0xd7c94efd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "teamFee", abi = "teamFee()")]
    pub struct TeamFeeCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `treasury` function with signature `treasury()` and selector `0x61d027b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "treasury", abi = "treasury()")]
    pub struct TreasuryCall;
    ///Container type for all input parameters for the `twapPoints` function with signature `twapPoints()` and selector `0x5d283021`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "twapPoints", abi = "twapPoints()")]
    pub struct TwapPointsCall;
    ///Container type for all input parameters for the `unPause` function with signature `unPause()` and selector `0xf7b188a5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unPause", abi = "unPause()")]
    pub struct UnPauseCall;
    ///Container type for all input parameters for the `underlyingToken` function with signature `underlyingToken()` and selector `0x2495a599`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "underlyingToken", abi = "underlyingToken()")]
    pub struct UnderlyingTokenCall;
    ///Container type for all input parameters for the `updateGauge` function with signature `updateGauge()` and selector `0x79c79707`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateGauge", abi = "updateGauge()")]
    pub struct UpdateGaugeCall;
    ///Container type for all input parameters for the `veDiscount` function with signature `veDiscount()` and selector `0x51217cbe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "veDiscount", abi = "veDiscount()")]
    pub struct VeDiscountCall;
    ///Container type for all input parameters for the `vmFee` function with signature `vmFee()` and selector `0xff26b1c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "vmFee", abi = "vmFee()")]
    pub struct VmFeeCall;
    ///Container type for all input parameters for the `vmTreasury` function with signature `vmTreasury()` and selector `0x20d1b665`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "vmTreasury", abi = "vmTreasury()")]
    pub struct VmTreasuryCall;
    ///Container type for all input parameters for the `voter` function with signature `voter()` and selector `0x46c96aac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "voter", abi = "voter()")]
    pub struct VoterCall;
    ///Container type for all input parameters for the `votingEscrow` function with signature `votingEscrow()` and selector `0x4f2bfe5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "votingEscrow", abi = "votingEscrow()")]
    pub struct VotingEscrowCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum oTOKENCalls {
        AdminRole(AdminRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        DomainSeparator(DomainSeparatorCall),
        FullLock(FullLockCall),
        MaxDiscount(MaxDiscountCall),
        MaxFees(MaxFeesCall),
        MaxTwapPoints(MaxTwapPointsCall),
        MinterRole(MinterRoleCall),
        MinDiscount(MinDiscountCall),
        PauserRole(PauserRoleCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        Discount(DiscountCall),
        ExerciseWithAmountAndMaxPaymentAmountAndDeadline(
            ExerciseWithAmountAndMaxPaymentAmountAndDeadlineCall,
        ),
        Exercise(ExerciseCall),
        ExerciseLp(ExerciseLpCall),
        ExerciseVe(ExerciseVeCall),
        Gauge(GaugeCall),
        GetDiscountedPrice(GetDiscountedPriceCall),
        GetLockDurationForLpDiscount(GetLockDurationForLpDiscountCall),
        GetLpDiscountedPrice(GetLpDiscountedPriceCall),
        GetPaymentTokenAmountForExerciseLp(GetPaymentTokenAmountForExerciseLpCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetSlopeInterceptForLpDiscount(GetSlopeInterceptForLpDiscountCall),
        GetTimeWeightedAveragePrice(GetTimeWeightedAveragePriceCall),
        GetVeDiscountedPrice(GetVeDiscountedPriceCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IsPaused(IsPausedCall),
        LockDurationForMaxLpDiscount(LockDurationForMaxLpDiscountCall),
        LockDurationForMinLpDiscount(LockDurationForMinLpDiscountCall),
        MaxLPDiscount(MaxLPDiscountCall),
        MinLPDiscount(MinLPDiscountCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Pair(PairCall),
        Pause(PauseCall),
        PaymentToken(PaymentTokenCall),
        Permit(PermitCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        Router(RouterCall),
        SetDiscount(SetDiscountCall),
        SetFees(SetFeesCall),
        SetGauge(SetGaugeCall),
        SetLockDurationForMaxLpDiscount(SetLockDurationForMaxLpDiscountCall),
        SetLockDurationForMinLpDiscount(SetLockDurationForMinLpDiscountCall),
        SetMaxLPDiscount(SetMaxLPDiscountCall),
        SetMinLPDiscount(SetMinLPDiscountCall),
        SetPairAndPaymentToken(SetPairAndPaymentTokenCall),
        SetRouter(SetRouterCall),
        SetTreasury(SetTreasuryCall),
        SetTwapPoints(SetTwapPointsCall),
        SetVeDiscount(SetVeDiscountCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TeamFee(TeamFeeCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Treasury(TreasuryCall),
        TwapPoints(TwapPointsCall),
        UnPause(UnPauseCall),
        UnderlyingToken(UnderlyingTokenCall),
        UpdateGauge(UpdateGaugeCall),
        VeDiscount(VeDiscountCall),
        VmFee(VmFeeCall),
        VmTreasury(VmTreasuryCall),
        Voter(VoterCall),
        VotingEscrow(VotingEscrowCall),
    }
    impl ::ethers::core::abi::AbiDecode for oTOKENCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminRole(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <FullLockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FullLock(decoded));
            }
            if let Ok(decoded)
                = <MaxDiscountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxDiscount(decoded));
            }
            if let Ok(decoded)
                = <MaxFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxFees(decoded));
            }
            if let Ok(decoded)
                = <MaxTwapPointsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxTwapPoints(decoded));
            }
            if let Ok(decoded)
                = <MinterRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinterRole(decoded));
            }
            if let Ok(decoded)
                = <MinDiscountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinDiscount(decoded));
            }
            if let Ok(decoded)
                = <PauserRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PauserRole(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DiscountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Discount(decoded));
            }
            if let Ok(decoded)
                = <ExerciseWithAmountAndMaxPaymentAmountAndDeadlineCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::ExerciseWithAmountAndMaxPaymentAmountAndDeadline(decoded),
                );
            }
            if let Ok(decoded)
                = <ExerciseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Exercise(decoded));
            }
            if let Ok(decoded)
                = <ExerciseLpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExerciseLp(decoded));
            }
            if let Ok(decoded)
                = <ExerciseVeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExerciseVe(decoded));
            }
            if let Ok(decoded)
                = <GaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Gauge(decoded));
            }
            if let Ok(decoded)
                = <GetDiscountedPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetDiscountedPrice(decoded));
            }
            if let Ok(decoded)
                = <GetLockDurationForLpDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLockDurationForLpDiscount(decoded));
            }
            if let Ok(decoded)
                = <GetLpDiscountedPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLpDiscountedPrice(decoded));
            }
            if let Ok(decoded)
                = <GetPaymentTokenAmountForExerciseLpCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPaymentTokenAmountForExerciseLp(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GetSlopeInterceptForLpDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSlopeInterceptForLpDiscount(decoded));
            }
            if let Ok(decoded)
                = <GetTimeWeightedAveragePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTimeWeightedAveragePrice(decoded));
            }
            if let Ok(decoded)
                = <GetVeDiscountedPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVeDiscountedPrice(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <IsPausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPaused(decoded));
            }
            if let Ok(decoded)
                = <LockDurationForMaxLpDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LockDurationForMaxLpDiscount(decoded));
            }
            if let Ok(decoded)
                = <LockDurationForMinLpDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LockDurationForMinLpDiscount(decoded));
            }
            if let Ok(decoded)
                = <MaxLPDiscountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxLPDiscount(decoded));
            }
            if let Ok(decoded)
                = <MinLPDiscountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinLPDiscount(decoded));
            }
            if let Ok(decoded)
                = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <PairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pair(decoded));
            }
            if let Ok(decoded)
                = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded)
                = <PaymentTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PaymentToken(decoded));
            }
            if let Ok(decoded)
                = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <RouterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Router(decoded));
            }
            if let Ok(decoded)
                = <SetDiscountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDiscount(decoded));
            }
            if let Ok(decoded)
                = <SetFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFees(decoded));
            }
            if let Ok(decoded)
                = <SetGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetGauge(decoded));
            }
            if let Ok(decoded)
                = <SetLockDurationForMaxLpDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetLockDurationForMaxLpDiscount(decoded));
            }
            if let Ok(decoded)
                = <SetLockDurationForMinLpDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetLockDurationForMinLpDiscount(decoded));
            }
            if let Ok(decoded)
                = <SetMaxLPDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetMaxLPDiscount(decoded));
            }
            if let Ok(decoded)
                = <SetMinLPDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetMinLPDiscount(decoded));
            }
            if let Ok(decoded)
                = <SetPairAndPaymentTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetPairAndPaymentToken(decoded));
            }
            if let Ok(decoded)
                = <SetRouterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRouter(decoded));
            }
            if let Ok(decoded)
                = <SetTreasuryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTreasury(decoded));
            }
            if let Ok(decoded)
                = <SetTwapPointsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTwapPoints(decoded));
            }
            if let Ok(decoded)
                = <SetVeDiscountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetVeDiscount(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TeamFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TeamFee(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <TreasuryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Treasury(decoded));
            }
            if let Ok(decoded)
                = <TwapPointsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TwapPoints(decoded));
            }
            if let Ok(decoded)
                = <UnPauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnPause(decoded));
            }
            if let Ok(decoded)
                = <UnderlyingTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnderlyingToken(decoded));
            }
            if let Ok(decoded)
                = <UpdateGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateGauge(decoded));
            }
            if let Ok(decoded)
                = <VeDiscountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VeDiscount(decoded));
            }
            if let Ok(decoded)
                = <VmFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VmFee(decoded));
            }
            if let Ok(decoded)
                = <VmTreasuryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VmTreasury(decoded));
            }
            if let Ok(decoded)
                = <VoterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter(decoded));
            }
            if let Ok(decoded)
                = <VotingEscrowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VotingEscrow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for oTOKENCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FullLock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxTwapPoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PauserRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Discount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExerciseWithAmountAndMaxPaymentAmountAndDeadline(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Exercise(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExerciseLp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExerciseVe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gauge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDiscountedPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLockDurationForLpDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLpDiscountedPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPaymentTokenAmountForExerciseLp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSlopeInterceptForLpDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTimeWeightedAveragePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVeDiscountedPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockDurationForMaxLpDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockDurationForMinLpDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxLPDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinLPDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pair(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PaymentToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Router(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLockDurationForMaxLpDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLockDurationForMinLpDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxLPDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMinLPDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPairAndPaymentToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRouter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTreasury(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTwapPoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVeDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TeamFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Treasury(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TwapPoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnPause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnderlyingToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VeDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VmFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VmTreasury(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VotingEscrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for oTOKENCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::FullLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxTwapPoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinterRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Discount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExerciseWithAmountAndMaxPaymentAmountAndDeadline(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Exercise(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExerciseLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExerciseVe(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDiscountedPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLockDurationForLpDiscount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLpDiscountedPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPaymentTokenAmountForExerciseLp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlopeInterceptForLpDiscount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTimeWeightedAveragePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVeDiscountedPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockDurationForMaxLpDiscount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LockDurationForMinLpDiscount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxLPDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinLPDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pair(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PaymentToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Router(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLockDurationForMaxLpDiscount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLockDurationForMinLpDiscount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxLPDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinLPDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPairAndPaymentToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRouter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTreasury(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTwapPoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetVeDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TeamFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Treasury(element) => ::core::fmt::Display::fmt(element, f),
                Self::TwapPoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::VeDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::VmFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::VmTreasury(element) => ::core::fmt::Display::fmt(element, f),
                Self::Voter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingEscrow(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminRoleCall> for oTOKENCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for oTOKENCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for oTOKENCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<FullLockCall> for oTOKENCalls {
        fn from(value: FullLockCall) -> Self {
            Self::FullLock(value)
        }
    }
    impl ::core::convert::From<MaxDiscountCall> for oTOKENCalls {
        fn from(value: MaxDiscountCall) -> Self {
            Self::MaxDiscount(value)
        }
    }
    impl ::core::convert::From<MaxFeesCall> for oTOKENCalls {
        fn from(value: MaxFeesCall) -> Self {
            Self::MaxFees(value)
        }
    }
    impl ::core::convert::From<MaxTwapPointsCall> for oTOKENCalls {
        fn from(value: MaxTwapPointsCall) -> Self {
            Self::MaxTwapPoints(value)
        }
    }
    impl ::core::convert::From<MinterRoleCall> for oTOKENCalls {
        fn from(value: MinterRoleCall) -> Self {
            Self::MinterRole(value)
        }
    }
    impl ::core::convert::From<MinDiscountCall> for oTOKENCalls {
        fn from(value: MinDiscountCall) -> Self {
            Self::MinDiscount(value)
        }
    }
    impl ::core::convert::From<PauserRoleCall> for oTOKENCalls {
        fn from(value: PauserRoleCall) -> Self {
            Self::PauserRole(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for oTOKENCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for oTOKENCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for oTOKENCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for oTOKENCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for oTOKENCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DiscountCall> for oTOKENCalls {
        fn from(value: DiscountCall) -> Self {
            Self::Discount(value)
        }
    }
    impl ::core::convert::From<ExerciseWithAmountAndMaxPaymentAmountAndDeadlineCall>
    for oTOKENCalls {
        fn from(value: ExerciseWithAmountAndMaxPaymentAmountAndDeadlineCall) -> Self {
            Self::ExerciseWithAmountAndMaxPaymentAmountAndDeadline(value)
        }
    }
    impl ::core::convert::From<ExerciseCall> for oTOKENCalls {
        fn from(value: ExerciseCall) -> Self {
            Self::Exercise(value)
        }
    }
    impl ::core::convert::From<ExerciseLpCall> for oTOKENCalls {
        fn from(value: ExerciseLpCall) -> Self {
            Self::ExerciseLp(value)
        }
    }
    impl ::core::convert::From<ExerciseVeCall> for oTOKENCalls {
        fn from(value: ExerciseVeCall) -> Self {
            Self::ExerciseVe(value)
        }
    }
    impl ::core::convert::From<GaugeCall> for oTOKENCalls {
        fn from(value: GaugeCall) -> Self {
            Self::Gauge(value)
        }
    }
    impl ::core::convert::From<GetDiscountedPriceCall> for oTOKENCalls {
        fn from(value: GetDiscountedPriceCall) -> Self {
            Self::GetDiscountedPrice(value)
        }
    }
    impl ::core::convert::From<GetLockDurationForLpDiscountCall> for oTOKENCalls {
        fn from(value: GetLockDurationForLpDiscountCall) -> Self {
            Self::GetLockDurationForLpDiscount(value)
        }
    }
    impl ::core::convert::From<GetLpDiscountedPriceCall> for oTOKENCalls {
        fn from(value: GetLpDiscountedPriceCall) -> Self {
            Self::GetLpDiscountedPrice(value)
        }
    }
    impl ::core::convert::From<GetPaymentTokenAmountForExerciseLpCall> for oTOKENCalls {
        fn from(value: GetPaymentTokenAmountForExerciseLpCall) -> Self {
            Self::GetPaymentTokenAmountForExerciseLp(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for oTOKENCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetSlopeInterceptForLpDiscountCall> for oTOKENCalls {
        fn from(value: GetSlopeInterceptForLpDiscountCall) -> Self {
            Self::GetSlopeInterceptForLpDiscount(value)
        }
    }
    impl ::core::convert::From<GetTimeWeightedAveragePriceCall> for oTOKENCalls {
        fn from(value: GetTimeWeightedAveragePriceCall) -> Self {
            Self::GetTimeWeightedAveragePrice(value)
        }
    }
    impl ::core::convert::From<GetVeDiscountedPriceCall> for oTOKENCalls {
        fn from(value: GetVeDiscountedPriceCall) -> Self {
            Self::GetVeDiscountedPrice(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for oTOKENCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for oTOKENCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IsPausedCall> for oTOKENCalls {
        fn from(value: IsPausedCall) -> Self {
            Self::IsPaused(value)
        }
    }
    impl ::core::convert::From<LockDurationForMaxLpDiscountCall> for oTOKENCalls {
        fn from(value: LockDurationForMaxLpDiscountCall) -> Self {
            Self::LockDurationForMaxLpDiscount(value)
        }
    }
    impl ::core::convert::From<LockDurationForMinLpDiscountCall> for oTOKENCalls {
        fn from(value: LockDurationForMinLpDiscountCall) -> Self {
            Self::LockDurationForMinLpDiscount(value)
        }
    }
    impl ::core::convert::From<MaxLPDiscountCall> for oTOKENCalls {
        fn from(value: MaxLPDiscountCall) -> Self {
            Self::MaxLPDiscount(value)
        }
    }
    impl ::core::convert::From<MinLPDiscountCall> for oTOKENCalls {
        fn from(value: MinLPDiscountCall) -> Self {
            Self::MinLPDiscount(value)
        }
    }
    impl ::core::convert::From<MintCall> for oTOKENCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for oTOKENCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for oTOKENCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PairCall> for oTOKENCalls {
        fn from(value: PairCall) -> Self {
            Self::Pair(value)
        }
    }
    impl ::core::convert::From<PauseCall> for oTOKENCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PaymentTokenCall> for oTOKENCalls {
        fn from(value: PaymentTokenCall) -> Self {
            Self::PaymentToken(value)
        }
    }
    impl ::core::convert::From<PermitCall> for oTOKENCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for oTOKENCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for oTOKENCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RouterCall> for oTOKENCalls {
        fn from(value: RouterCall) -> Self {
            Self::Router(value)
        }
    }
    impl ::core::convert::From<SetDiscountCall> for oTOKENCalls {
        fn from(value: SetDiscountCall) -> Self {
            Self::SetDiscount(value)
        }
    }
    impl ::core::convert::From<SetFeesCall> for oTOKENCalls {
        fn from(value: SetFeesCall) -> Self {
            Self::SetFees(value)
        }
    }
    impl ::core::convert::From<SetGaugeCall> for oTOKENCalls {
        fn from(value: SetGaugeCall) -> Self {
            Self::SetGauge(value)
        }
    }
    impl ::core::convert::From<SetLockDurationForMaxLpDiscountCall> for oTOKENCalls {
        fn from(value: SetLockDurationForMaxLpDiscountCall) -> Self {
            Self::SetLockDurationForMaxLpDiscount(value)
        }
    }
    impl ::core::convert::From<SetLockDurationForMinLpDiscountCall> for oTOKENCalls {
        fn from(value: SetLockDurationForMinLpDiscountCall) -> Self {
            Self::SetLockDurationForMinLpDiscount(value)
        }
    }
    impl ::core::convert::From<SetMaxLPDiscountCall> for oTOKENCalls {
        fn from(value: SetMaxLPDiscountCall) -> Self {
            Self::SetMaxLPDiscount(value)
        }
    }
    impl ::core::convert::From<SetMinLPDiscountCall> for oTOKENCalls {
        fn from(value: SetMinLPDiscountCall) -> Self {
            Self::SetMinLPDiscount(value)
        }
    }
    impl ::core::convert::From<SetPairAndPaymentTokenCall> for oTOKENCalls {
        fn from(value: SetPairAndPaymentTokenCall) -> Self {
            Self::SetPairAndPaymentToken(value)
        }
    }
    impl ::core::convert::From<SetRouterCall> for oTOKENCalls {
        fn from(value: SetRouterCall) -> Self {
            Self::SetRouter(value)
        }
    }
    impl ::core::convert::From<SetTreasuryCall> for oTOKENCalls {
        fn from(value: SetTreasuryCall) -> Self {
            Self::SetTreasury(value)
        }
    }
    impl ::core::convert::From<SetTwapPointsCall> for oTOKENCalls {
        fn from(value: SetTwapPointsCall) -> Self {
            Self::SetTwapPoints(value)
        }
    }
    impl ::core::convert::From<SetVeDiscountCall> for oTOKENCalls {
        fn from(value: SetVeDiscountCall) -> Self {
            Self::SetVeDiscount(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for oTOKENCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for oTOKENCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TeamFeeCall> for oTOKENCalls {
        fn from(value: TeamFeeCall) -> Self {
            Self::TeamFee(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for oTOKENCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for oTOKENCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for oTOKENCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TreasuryCall> for oTOKENCalls {
        fn from(value: TreasuryCall) -> Self {
            Self::Treasury(value)
        }
    }
    impl ::core::convert::From<TwapPointsCall> for oTOKENCalls {
        fn from(value: TwapPointsCall) -> Self {
            Self::TwapPoints(value)
        }
    }
    impl ::core::convert::From<UnPauseCall> for oTOKENCalls {
        fn from(value: UnPauseCall) -> Self {
            Self::UnPause(value)
        }
    }
    impl ::core::convert::From<UnderlyingTokenCall> for oTOKENCalls {
        fn from(value: UnderlyingTokenCall) -> Self {
            Self::UnderlyingToken(value)
        }
    }
    impl ::core::convert::From<UpdateGaugeCall> for oTOKENCalls {
        fn from(value: UpdateGaugeCall) -> Self {
            Self::UpdateGauge(value)
        }
    }
    impl ::core::convert::From<VeDiscountCall> for oTOKENCalls {
        fn from(value: VeDiscountCall) -> Self {
            Self::VeDiscount(value)
        }
    }
    impl ::core::convert::From<VmFeeCall> for oTOKENCalls {
        fn from(value: VmFeeCall) -> Self {
            Self::VmFee(value)
        }
    }
    impl ::core::convert::From<VmTreasuryCall> for oTOKENCalls {
        fn from(value: VmTreasuryCall) -> Self {
            Self::VmTreasury(value)
        }
    }
    impl ::core::convert::From<VoterCall> for oTOKENCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
        }
    }
    impl ::core::convert::From<VotingEscrowCall> for oTOKENCalls {
        fn from(value: VotingEscrowCall) -> Self {
            Self::VotingEscrow(value)
        }
    }
    ///Container type for all return fields from the `ADMIN_ROLE` function with signature `ADMIN_ROLE()` and selector `0x75b238fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `FULL_LOCK` function with signature `FULL_LOCK()` and selector `0x54cb0384`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FullLockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_DISCOUNT` function with signature `MAX_DISCOUNT()` and selector `0xe3495569`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaxDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_FEES` function with signature `MAX_FEES()` and selector `0xc2300bef`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaxFeesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_TWAP_POINTS` function with signature `MAX_TWAP_POINTS()` and selector `0x881326b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaxTwapPointsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `0xd5391393`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MinterRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MIN_DISCOUNT` function with signature `MIN_DISCOUNT()` and selector `0x2ac8a92c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MinDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PauserRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `discount` function with signature `discount()` and selector `0x6b6f4a9d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `exercise` function with signature `exercise(uint256,uint256,address,uint256)` and selector `0xa1d50c3a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExerciseWithAmountAndMaxPaymentAmountAndDeadlineReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `exercise` function with signature `exercise(uint256,uint256,address)` and selector `0xd6379b72`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExerciseReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `exerciseLp` function with signature `exerciseLp(uint256,uint256,address,uint256,uint256)` and selector `0xf897bd1d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExerciseLpReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `exerciseVe` function with signature `exerciseVe(uint256,uint256,address,uint256)` and selector `0x62994c05`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExerciseVeReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `gauge` function with signature `gauge()` and selector `0xa6f19c84`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GaugeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDiscountedPrice` function with signature `getDiscountedPrice(uint256)` and selector `0x339ccade`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetDiscountedPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLockDurationForLpDiscount` function with signature `getLockDurationForLpDiscount(uint256)` and selector `0x787dd9e4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetLockDurationForLpDiscountReturn {
        pub duration: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getLpDiscountedPrice` function with signature `getLpDiscountedPrice(uint256,uint256)` and selector `0xc7d1e395`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetLpDiscountedPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPaymentTokenAmountForExerciseLp` function with signature `getPaymentTokenAmountForExerciseLp(uint256,uint256)` and selector `0x9e66645b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetPaymentTokenAmountForExerciseLpReturn {
        pub payment_amount: ::ethers::core::types::U256,
        pub payment_amount_to_add_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getSlopeInterceptForLpDiscount` function with signature `getSlopeInterceptForLpDiscount()` and selector `0x3cdfed56`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSlopeInterceptForLpDiscountReturn {
        pub slope: ::ethers::core::types::I256,
        pub intercept: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `getTimeWeightedAveragePrice` function with signature `getTimeWeightedAveragePrice(uint256)` and selector `0xe8772bb2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetTimeWeightedAveragePriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVeDiscountedPrice` function with signature `getVeDiscountedPrice(uint256)` and selector `0x6e180f6a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetVeDiscountedPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `isPaused` function with signature `isPaused()` and selector `0xb187bd26`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsPausedReturn(pub bool);
    ///Container type for all return fields from the `lockDurationForMaxLpDiscount` function with signature `lockDurationForMaxLpDiscount()` and selector `0x1adb040a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LockDurationForMaxLpDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lockDurationForMinLpDiscount` function with signature `lockDurationForMinLpDiscount()` and selector `0xb3427576`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LockDurationForMinLpDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxLPDiscount` function with signature `maxLPDiscount()` and selector `0x062e5bbe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaxLPDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minLPDiscount` function with signature `minLPDiscount()` and selector `0x860ca9bb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MinLPDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pair` function with signature `pair()` and selector `0xa8aa1b31`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PairReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paymentToken` function with signature `paymentToken()` and selector `0x3013ce29`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PaymentTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `router` function with signature `router()` and selector `0xf887ea40`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RouterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `teamFee` function with signature `teamFee()` and selector `0xd7c94efd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TeamFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferFromReturn(pub bool);
    ///Container type for all return fields from the `treasury` function with signature `treasury()` and selector `0x61d027b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TreasuryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `twapPoints` function with signature `twapPoints()` and selector `0x5d283021`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TwapPointsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `underlyingToken` function with signature `underlyingToken()` and selector `0x2495a599`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UnderlyingTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `veDiscount` function with signature `veDiscount()` and selector `0x51217cbe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VeDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vmFee` function with signature `vmFee()` and selector `0xff26b1c8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VmFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vmTreasury` function with signature `vmTreasury()` and selector `0x20d1b665`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VmTreasuryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `voter` function with signature `voter()` and selector `0x46c96aac`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VoterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `votingEscrow` function with signature `votingEscrow()` and selector `0x4f2bfe5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VotingEscrowReturn(pub ::ethers::core::types::Address);
}
