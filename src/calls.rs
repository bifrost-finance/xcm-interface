use codec::{Decode, Encode};
use frame_support::{sp_runtime::MultiSignature, RuntimeDebug};
use sp_std::vec::Vec;

use crate::ChainId;

#[derive(Encode, Decode, RuntimeDebug)]
pub enum UtilityCall<RelayChainCall> {
	#[codec(index = 1)]
	AsDerivative(u16, RelayChainCall),
	#[codec(index = 2)]
	BatchAll(Vec<RelayChainCall>),
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum StakingCall {
	#[codec(index = 3)]
	WithdrawUnbonded(u32),
}

pub mod rococo {

	pub use crate::calls::*;

	#[derive(Encode, Decode, RuntimeDebug)]
	pub enum RelaychainCall<BalanceOf, AccountIdOf, BlockNumberOf> {
		#[codec(index = 28)]
		Crowdloan(ContributeCall<BalanceOf, AccountIdOf>),
		#[codec(index = 91)]
		Proxy(ProxyCall<AccountIdOf, BlockNumberOf>),
	}
}

pub mod kusama {

	pub use crate::calls::*;

	#[derive(Encode, Decode, RuntimeDebug)]
	pub enum RelaychainCall<BalanceOf, AccountIdOf, BlockNumberOf> {
		#[codec(index = 73)]
		Crowdloan(ContributeCall<BalanceOf, AccountIdOf>),
		#[codec(index = 30)]
		Proxy(ProxyCall<AccountIdOf, BlockNumberOf>),
	}
}

pub mod polkadot {

	pub use crate::calls::*;

	#[derive(Encode, Decode, RuntimeDebug)]
	pub enum RelaychainCall<BalanceOf, AccountIdOf, BlockNumberOf> {
		#[codec(index = 73)]
		Crowdloan(ContributeCall<BalanceOf, AccountIdOf>),
		#[codec(index = 29)]
		Proxy(ProxyCall<AccountIdOf, BlockNumberOf>),
	}
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum ContributeCall<BalanceOf, AccountIdOf> {
	#[codec(index = 1)]
	Contribute(Contribution<BalanceOf>),
	#[codec(index = 2)]
	Withdraw(Withdraw<AccountIdOf>),
	#[codec(index = 6)]
	AddMemo(AddMemo),
}

#[derive(PartialEq, Encode, Decode, RuntimeDebug)]
pub struct Contribution<BalanceOf> {
	#[codec(compact)]
	pub index: ChainId,
	#[codec(compact)]
	pub value: BalanceOf,
	pub signature: Option<MultiSignature>,
}

#[derive(PartialEq, Encode, Decode, RuntimeDebug)]
pub struct Withdraw<AccountIdOf> {
	pub who: AccountIdOf,
	#[codec(compact)]
	pub index: ChainId,
}

#[derive(PartialEq, Encode, Decode, RuntimeDebug)]
pub struct AddMemo {
	pub index: ChainId,
	pub memo: Vec<u8>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	Staking,
	IdentityJudgement,
	CancelProxy,
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum ProxyCall<AccountIdOf, BlockNumberOf> {
	#[codec(index = 1)]
	Add(AddProxy<AccountIdOf, BlockNumberOf>),
	#[codec(index = 2)]
	Remove(RemoveProxy<AccountIdOf, BlockNumberOf>),
}

#[derive(PartialEq, Encode, Decode, RuntimeDebug)]
pub struct AddProxy<AccountIdOf, BlockNumberOf> {
	pub delegate: AccountIdOf,
	pub proxy_type: ProxyType,
	pub delay: BlockNumberOf,
}

#[derive(PartialEq, Encode, Decode, RuntimeDebug)]
pub struct RemoveProxy<AccountIdOf, BlockNumberOf> {
	pub delegate: AccountIdOf,
	pub proxy_type: ProxyType,
	pub delay: BlockNumberOf,
}
