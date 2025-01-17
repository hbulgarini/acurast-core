use frame_support::instances::{Instance1, Instance2};
use frame_support::pallet_prelude::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum HyperdriveInstance {
    Tezos,
    Ethereum,
}

pub type TezosInstance = Instance1;
pub type EthereumInstance = Instance2;

pub trait HyperdriveInstanceName {
    const NAME: HyperdriveInstance;
}

impl HyperdriveInstanceName for TezosInstance {
    const NAME: HyperdriveInstance = HyperdriveInstance::Tezos;
}

impl HyperdriveInstanceName for EthereumInstance {
    const NAME: HyperdriveInstance = HyperdriveInstance::Ethereum;
}
