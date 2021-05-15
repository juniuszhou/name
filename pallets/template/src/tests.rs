use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use sp_io::hashing::keccak_256;

#[test]
fn test_register() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name_hash = [0_u8; 32];
		assert_ok!(TemplateModule::register_name(Origin::signed(1), name_hash.clone()));
		assert_noop!(
			TemplateModule::register_name(Origin::signed(1), name_hash.clone()),
			Error::<Test>::NameAlreadyRegistered
		);
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::register(name_hash), Some(1));
	});
}

#[test]
fn test_claim() {
	new_test_ext().execute_with(|| {
		let name = vec![1_u8];
		let name_hash: [u8; 32] = keccak_256(&name[..]);
		let invalid_name_hash = [0_u8; 32];

		assert_ok!(TemplateModule::register_name(Origin::signed(1), name_hash.clone()));

		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::claim_name(Origin::signed(2), name.clone(), name_hash.clone()),
			Error::<Test>::NameNotRegisteredByYou
		);

		assert_noop!(
			TemplateModule::claim_name(Origin::signed(1), name.clone(), invalid_name_hash),
			Error::<Test>::NameNotMatchHash
		);

		assert_ok!(TemplateModule::claim_name(Origin::signed(1), name.clone(), name_hash.clone()));

		assert_noop!(
			TemplateModule::claim_name(Origin::signed(1), name.clone(), name_hash.clone()),
			Error::<Test>::NameAlreadyClaimed
		);

		assert_eq!(TemplateModule::owner(name), Some(1));

	});
}
