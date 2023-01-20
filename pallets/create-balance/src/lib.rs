//! Minimal Pallet allows to add balances to calling user

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;

pub use pallet::*;

#[pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::SaturatedConversion;
	use frame_support::traits::*;
	use frame_system::pallet_prelude::*;
	use pallet_evm::AddressMapping;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type AddressMapping: AddressMapping<Self::AccountId>;
		type Currency: Currency<Self::AccountId>;
	}

	// Pallet callable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique collectible.
		///
		/// The actual collectible creation is done in the `mint()` function.
		#[pallet::weight(0)]
		pub fn create_balances(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;
			T::Currency::deposit_creating(&sender, 100_100_100_u128.saturated_into());
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn destroy_balances(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;
			T::Currency::slash(&sender, 100_100_100_u128.saturated_into());
			Ok(())
		}
	}
}
