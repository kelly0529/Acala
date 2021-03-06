//! Mocks for the honzon module.

#![cfg(test)]

use frame_support::{impl_outer_dispatch, impl_outer_event, impl_outer_origin, ord_parameter_types, parameter_types};
use primitives::H256;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::IdentityLookup,
	Perbill,
};
use support::{AuctionManager, ExchangeRate, Price, PriceProvider, Rate, Ratio};
use system::EnsureSignedBy;

use super::*;

mod honzon {
	pub use super::super::*;
}

impl_outer_dispatch! {
	pub enum Call for Runtime where origin: Origin {
		cdp_engine::CdpEngineModule,
	}
}

impl_outer_event! {
	pub enum TestEvent for Runtime {
		system<T>,
		honzon<T>,
		cdp_engine<T>,
		orml_tokens<T>,
		loans<T>,
		pallet_balances<T>,
		orml_currencies<T>,
		cdp_treasury<T>,
	}
}

impl_outer_origin! {
	pub enum Origin for Runtime {

	}
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
	pub const ExistentialDeposit: u64 = 1;
	pub const CreationFee: u64 = 2;
	pub const CollateralCurrencyIds: Vec<CurrencyId> = vec![BTC, DOT];
	pub const GlobalStabilityFee: Rate = Rate::from_parts(0);
	pub const DefaultLiquidationRatio: Ratio = Ratio::from_rational(3, 2);
	pub const DefaultDebitExchangeRate: ExchangeRate = ExchangeRate::from_natural(1);
	pub const DefaultLiquidationPenalty: Rate = Rate::from_rational(10, 100);
	pub const MinimumDebitValue: Balance = 2;
	pub const GetNativeCurrencyId: CurrencyId = ACA;
	pub const GetStableCurrencyId: CurrencyId = AUSD;
}

pub type AccountId = u64;
pub type BlockNumber = u64;
pub type Balance = u64;
pub type Amount = i64;
pub type DebitBalance = u64;
pub type DebitAmount = i64;
pub type CurrencyId = u32;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;

pub const ACA: CurrencyId = 0;
pub const AUSD: CurrencyId = 1;
pub const BTC: CurrencyId = 2;
pub const DOT: CurrencyId = 3;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;

impl system::Trait for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = ();
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
}
pub type System = system::Module<Runtime>;

impl orml_tokens::Trait for Runtime {
	type Event = TestEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type ExistentialDeposit = ExistentialDeposit;
	type DustRemoval = ();
}
pub type Tokens = orml_tokens::Module<Runtime>;

impl pallet_balances::Trait for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = TestEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = system::Module<Runtime>;
}
pub type PalletBalances = pallet_balances::Module<Runtime>;

pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Runtime, PalletBalances, Balance>;

impl orml_currencies::Trait for Runtime {
	type Event = TestEvent;
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
}
pub type Currencies = orml_currencies::Module<Runtime>;

impl loans::Trait for Runtime {
	type Event = TestEvent;
	type Convert = cdp_engine::DebitExchangeRateConvertor<Runtime>;
	type Currency = Tokens;
	type RiskManager = CdpEngineModule;
	type DebitBalance = DebitBalance;
	type DebitAmount = DebitAmount;
	type Treasury = CdpTreasury;
}
pub type LoansModule = loans::Module<Runtime>;

pub struct MockPriceSource;
impl PriceProvider<CurrencyId, Price> for MockPriceSource {
	#[allow(unused_variables)]
	fn get_price(base: CurrencyId, quote: CurrencyId) -> Option<Price> {
		Some(Price::from_natural(1))
	}

	#[allow(unused_variables)]
	fn lock_price(currency_id: CurrencyId) {}

	#[allow(unused_variables)]
	fn unlock_price(currency_id: CurrencyId) {}
}

pub struct MockAuctionManager;
impl AuctionManager<AccountId> for MockAuctionManager {
	type CurrencyId = CurrencyId;
	type Balance = Balance;

	#[allow(unused_variables)]
	fn new_collateral_auction(
		who: &AccountId,
		currency_id: Self::CurrencyId,
		amount: Self::Balance,
		target: Self::Balance,
	) {
	}

	#[allow(unused_variables)]
	fn new_debit_auction(amount: Self::Balance, fix: Self::Balance) {}

	#[allow(unused_variables)]
	fn new_surplus_auction(amount: Self::Balance) {}

	fn get_total_debit_in_auction() -> Self::Balance {
		Default::default()
	}

	fn get_total_target_in_auction() -> Self::Balance {
		Default::default()
	}
}

ord_parameter_types! {
	pub const One: AccountId = 1;
}

impl cdp_treasury::Trait for Runtime {
	type Event = TestEvent;
	type Currency = Currencies;
	type GetStableCurrencyId = GetStableCurrencyId;
	type AuctionManagerHandler = MockAuctionManager;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type Dex = ();
}
pub type CdpTreasury = cdp_treasury::Module<Runtime>;

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<Call, ()>;
type SubmitTransaction = system::offchain::TransactionSubmitter<(), Call, Extrinsic>;

parameter_types! {
	pub const MaxSlippageSwapWithDex: Ratio = Ratio::from_rational(50, 100);
}

impl cdp_engine::Trait for Runtime {
	type Event = TestEvent;
	type PriceSource = MockPriceSource;
	type CollateralCurrencyIds = CollateralCurrencyIds;
	type GlobalStabilityFee = GlobalStabilityFee;
	type DefaultLiquidationRatio = DefaultLiquidationRatio;
	type DefaultDebitExchangeRate = DefaultDebitExchangeRate;
	type DefaultLiquidationPenalty = DefaultLiquidationPenalty;
	type MinimumDebitValue = MinimumDebitValue;
	type GetStableCurrencyId = GetStableCurrencyId;
	type Treasury = CdpTreasury;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type MaxSlippageSwapWithDex = MaxSlippageSwapWithDex;
	type Currency = Currencies;
	type Dex = ();
	type Call = Call;
	type SubmitTransaction = SubmitTransaction;
}
pub type CdpEngineModule = cdp_engine::Module<Runtime>;

impl Trait for Runtime {
	type Event = TestEvent;
}
pub type HonzonModule = Module<Runtime>;

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![
				(ALICE, BTC, 1000),
				(BOB, BTC, 1000),
				(ALICE, DOT, 1000),
				(BOB, DOT, 1000),
			],
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> runtime_io::TestExternalities {
		let mut t = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
