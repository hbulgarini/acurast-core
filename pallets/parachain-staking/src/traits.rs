// Copyright 2019-2022 PureStake Inc.
// Copyright 2023 Papers AG

//! traits for parachain-staking

use frame_support::pallet_prelude::Weight;

pub trait OnCollatorPayout<AccountId, Balance> {
    fn on_collator_payout(
        for_round: crate::RoundIndex,
        collator_id: AccountId,
        amount: Balance,
    ) -> Weight;
}
impl<AccountId, Balance> OnCollatorPayout<AccountId, Balance> for () {
    fn on_collator_payout(
        _for_round: crate::RoundIndex,
        _collator_id: AccountId,
        _amount: Balance,
    ) -> Weight {
        Weight::zero()
    }
}

pub trait OnNewRound {
    fn on_new_round(round_index: crate::RoundIndex) -> Weight;
}
impl OnNewRound for () {
    fn on_new_round(_round_index: crate::RoundIndex) -> Weight {
        Weight::zero()
    }
}

/// Defines the behavior to payout the collator's reward.
pub trait PayoutCollatorReward<Runtime: crate::Config> {
    fn payout_collator_reward(
        round_index: crate::RoundIndex,
        collator_id: Runtime::AccountId,
        amount: crate::BalanceOf<Runtime>,
    ) -> Weight;
}

/// Defines the default behavior for paying out the collator's reward. The amount is directly
/// deposited into the collator's account.
impl<Runtime: crate::Config> PayoutCollatorReward<Runtime> for () {
    fn payout_collator_reward(
        for_round: crate::RoundIndex,
        collator_id: Runtime::AccountId,
        amount: crate::BalanceOf<Runtime>,
    ) -> Weight {
        crate::Pallet::<Runtime>::mint_collator_reward(for_round, collator_id, amount)
    }
}
