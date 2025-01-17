#![allow(unused_imports)]

use derive_more::{Display, From, Into};
use frame_support::pallet_prelude::*;
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU64},
    Deserialize, Serialize,
};
use frame_system as system;
use hex_literal::hex;
use pallet_acurast::CU32;
use sp_core::H256;
use sp_core::*;
use sp_runtime::traits::Keccak256;
use sp_runtime::MultiSignature;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    AccountId32,
};
use sp_std::prelude::*;

use pallet_acurast_marketplace::RegistrationExtra;

use crate::chain::tezos::TezosParser;
use crate::instances::{EthereumInstance, TezosInstance};
use crate::stub::AcurastAccountId;
use crate::types::RawAction;
use crate::{weights, ActionExecutor, ParsedAction, StateOwner, StateProof, StateProofNode};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

parameter_types! {
    pub TargetChainStateOwner: StateOwner = StateOwner::try_from(hex!("050a0000001600009f7f36d0241d3e6a82254216d7de5780aa67d8f9").to_vec()).unwrap();
    pub const TransmissionRate: u64 = 5;
    pub const TransmissionQuorum: u8 = 2;
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        TezosHyperdrive: crate::<Instance1>,
        EthereumHyperdrive: crate::<Instance2>
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub type MaxAllowedSources = CU32<4>;

impl crate::Config<TezosInstance> for Test {
    type RuntimeEvent = RuntimeEvent;
    type ParsableAccountId = AcurastAccountId;
    type TargetChainOwner = TargetChainStateOwner;
    type TargetChainHash = H256;
    type TargetChainBlockNumber = u64;
    type Balance = Balance;
    type RegistrationExtra =
        RegistrationExtra<Self::Balance, <Self as frame_system::Config>::AccountId, Self::MaxSlots>;
    type MaxAllowedSources = MaxAllowedSources;
    type MaxSlots = CU32<64>;
    type MaxTransmittersPerSnapshot = CU32<64>;
    type TargetChainHashing = Keccak256;
    type TransmissionRate = TransmissionRate;
    type TransmissionQuorum = TransmissionQuorum;
    type ActionExecutor = ();
    type Proof = crate::chain::tezos::TezosProof<
        Self::ParsableAccountId,
        <Self as frame_system::Config>::AccountId,
    >;
    type WeightInfo = weights::WeightInfo<Test>;
}

impl crate::Config<EthereumInstance> for Test {
    type RuntimeEvent = RuntimeEvent;
    type ParsableAccountId = AcurastAccountId;
    type TargetChainOwner = TargetChainStateOwner;
    type TargetChainHash = H256;
    type TargetChainBlockNumber = u64;
    type Balance = Balance;
    type RegistrationExtra =
        RegistrationExtra<Self::Balance, <Self as frame_system::Config>::AccountId, Self::MaxSlots>;
    type MaxAllowedSources = MaxAllowedSources;
    type MaxSlots = CU32<64>;
    type MaxTransmittersPerSnapshot = CU32<64>;
    type TargetChainHashing = Keccak256;
    type TransmissionRate = TransmissionRate;
    type TransmissionQuorum = TransmissionQuorum;
    type ActionExecutor = ();
    type Proof = crate::chain::ethereum::EthereumProof<
        AcurastAccountId,
        <Self as frame_system::Config>::AccountId,
    >;
    type WeightInfo = weights::WeightInfo<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub fn events() -> Vec<RuntimeEvent> {
    log::debug!("{:#?}", System::events());
    let evt = System::events()
        .into_iter()
        .map(|evt| evt.event)
        .collect::<Vec<_>>();

    System::reset_events();

    evt
}

pub type Balance = u128;

impl<AccountId, Extra> ActionExecutor<AccountId, MaxAllowedSources, Extra> for () {
    fn execute(_: ParsedAction<AccountId, MaxAllowedSources, Extra>) -> DispatchResultWithPostInfo {
        Ok(().into())
    }
}
