use codec::FullCodec;
use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize};
use sp_std::{fmt::Debug, vec::Vec};
pub use scale_info::TypeInfo;

/// Hooks to manage reward pool
pub trait RewardHandler<AccountId, BlockNumber> {
	/// The share type of pool
	type Share: AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize + Debug + TypeInfo;

	/// The reward balance type
	type Balance: AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize + Debug + TypeInfo;

	/// The reward pool ID type
	type PoolId: Copy + FullCodec;

	/// The currency type
	type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug + TypeInfo;

	/// Accumulate rewards
	fn accumulate_reward(
		now: BlockNumber,
		callback: impl FnMut(Self::PoolId, Self::Balance),
	) -> Vec<(Self::CurrencyId, Self::Balance)>;

	/// Payout the reward to `who`
	fn payout(who: &AccountId, pool: Self::PoolId, amount: Self::Balance);
}
