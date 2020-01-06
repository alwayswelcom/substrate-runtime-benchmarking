#![cfg_attr(not(feature = "std"), no_std)]

/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event};
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Benchmarking {
		// Membership stored as a single storage vector
		MemberVec: Vec<T::AccountId>;
		// Membership stored as a storage map to an option
		MemberMap: map T::AccountId => Option<()>;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		pub fn add_member_vec(origin, new: T::AccountId) {
			let _ = ensure_signed(origin)?;
			MemberVec::<T>::mutate(|members| members.push(new));
		}

		pub fn add_member_map(origin, new: T::AccountId) {
			let _ = ensure_signed(origin)?;
			MemberMap::<T>::insert(new, ());
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		Dummy(u32, AccountId),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use primitives::H256;
	use support::{impl_outer_origin, assert_ok, parameter_types};
	use sr_primitives::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, weights::Weight, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type Benchmarking = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn basic_setup_works() {
		new_test_ext().execute_with(|| {
			// Initial vec is empty
			assert_eq!(MemberVec::<Test>::exists(), false);
			// Initial map is empty
			assert_eq!(MemberMap::<Test>::exists(1), false);
			// Check add to map works fine
			assert_ok!(Benchmarking::add_member_map(Origin::signed(1), 1));
			// Check add to vec works fine
			assert_ok!(Benchmarking::add_member_vec(Origin::signed(1), 1));
			// Stuff is populated
			assert_eq!(MemberVec::<Test>::exists(), true);
			assert_eq!(MemberMap::<Test>::exists(1), true);
			assert_eq!(MemberVec::<Test>::get(), vec![1]);
			assert_eq!(MemberMap::<Test>::get(1), Some(()));
		});
	}
}
