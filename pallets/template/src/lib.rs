#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;
	use sp_io::hashing::keccak_256;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn owner)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn register)]
	pub type Register<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 32], T::AccountId>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Registered(T::AccountId, [u8; 32]),
		Claimed(T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		NameAlreadyRegistered,
		NameNotRegistered,
		NameNotRegisteredByYou,
		NameAlreadyClaimed,
		NameNotMatchHash,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn register_name(origin: OriginFor<T>, name_hash: [u8; 32]) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Register<T>>::get(name_hash) {
				Some(old) => Err(Error::<T>::NameAlreadyRegistered)?,
				None => {
					<Register<T>>::insert(name_hash, who.clone());
					Self::deposit_event(Event::Registered(who, name_hash));
					Ok(().into())
				},
			}		
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn claim_name(origin: OriginFor<T>, name: Vec<u8>, name_hash: [u8; 32]) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Self::validate_name(&name, &name_hash)?;

			match <Register<T>>::get(&name_hash) {
				None => Err(Error::<T>::NameNotRegistered)?,
				Some(old) => {
					if old != who.clone() {
						Err(Error::<T>::NameNotRegisteredByYou)?
					}
				},
			}

			match <Owner<T>>::get(&name) {
				Some(_) => Err(Error::<T>::NameAlreadyClaimed)?,
				None => {
					<Owner<T>>::insert(name.clone(), who.clone());
					Self::deposit_event(Event::Claimed(who, name));
					Ok(().into())

				}
			}
		}
	}

	impl<T: Config> Module<T> {
		fn validate_name(name: &Vec<u8>, name_hash: &[u8; 32]) -> DispatchResultWithPostInfo {
			if keccak_256(&name[..]) == *name_hash {
				Ok(().into())
			} else {
				Err(Error::<T>::NameNotMatchHash)?
			}
		}
	}
}
