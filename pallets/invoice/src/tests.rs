use std::ptr::null;
use crate::{mock::*, Error, Config, Event, mock, pallet};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_invoice_same_address_error() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 2;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_noop!(Invoice::create_invoice(
                origin,
                to,
                amount,
                msg,
            ),
                Error::<Test>::SameAddressError);
    })
}

#[test]
fn create_invoice_ok() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_ok!(Invoice::create_invoice(
                origin.clone(),
                to,
                amount.clone(),
                msg.clone()));
    })
}


#[test]
fn exist_invoice_1() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_ok!(Invoice::create_invoice(
                origin.clone(),
                to,
                amount.clone(),
                msg.clone()));

        let origin = Origin::signed(ALICE);
        assert_ok!(Invoice::exist_invoice(
                origin.clone()
        ));
    })
}

#[test]
fn exist_invoice_2() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 1;
        let origin = Origin::signed(ALICE);

        assert_noop!(
            Invoice::exist_invoice(origin),
                Error::<Test>::NoInvoiceFound);
    })
}


#[test]
fn pay_invoices_same_address_error() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 2;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_noop!(Invoice::create_invoice(
                origin,
                to,
                amount,
                msg,
            ),Error::<Test>::SameAddressError);
    })
}

#[test]
fn pay_invoice_error_any_1() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_ok!(Invoice::create_invoice(
                origin.clone(),
                to,
                amount.clone(),
                msg.clone()));

        assert_noop!(Invoice::pay_invoice(
                origin.clone(),
                to,
                0),  Error::<Test>::NoInvoiceOrPaid);
    })
}


#[test]
fn pay_invoices_success() {
    new_test_ext().execute_with(|| {
        type AccountId = u64;
        type BalanceOf = u64;
        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(BOB);
        let to = ALICE;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_ok!(Invoice::create_invoice(
                origin.clone(),
                to,
                amount.clone(),
                msg.clone()));

        assert_ok!(Invoice::pay_invoice(
               Origin::signed(ALICE),
                 BOB,
            0));
    })
}

