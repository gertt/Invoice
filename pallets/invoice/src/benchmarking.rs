//! Benchmarking setup for invoice
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Invoice;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};
use crate::Event;
use frame_support::{assert_ok, sp_runtime::traits::Bounded};
use frame_support::sp_runtime::traits::Saturating;
use sp_std::vec::Vec;


fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}


benchmarks! {
	create_invoice {
		let to: T::AccountId = account("receiver", 0, 0);
		let amount = BalanceOf::<T>::max_value();
		let caller: T::AccountId = whitelisted_caller();
		let msg = Vec::from([2, 23, 34, 45]);

	}: _(RawOrigin::Signed(caller.clone()), to.clone(), amount.clone(), msg.clone())
	verify {

	assert_ok!(
			Invoice::<T>::create_invoice(RawOrigin::Signed(caller.clone()).into(), to.clone(), amount.clone(), msg.clone())
		);
	}


	impl_benchmark_test_suite!(Invoice, crate::mock::new_test_ext(), crate::mock::Test);
}

