use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn can_create_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_claim(Origin::signed(1), vec![1,2,3,4]));
	});
}

#[test]
fn should_error_if_claim_rewrite_attempted() {
  new_test_ext().execute_with(|| {
    let proof = vec![1,2,3,4];
		assert_ok!(TemplateModule::create_claim(Origin::signed(1), proof.clone()));

    assert_noop!(TemplateModule::create_claim(Origin::signed(1), proof.clone()), Error::<Test>::ProofAlreadyClaimed);
  })
}

