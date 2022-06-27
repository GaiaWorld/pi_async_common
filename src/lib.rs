#[macro_use]
extern crate lazy_static;

use pi_async::rt::{
	multi_thread::{MultiTaskRuntime, StealableTaskPool}, AsyncRuntimeBuilder
};

lazy_static! {
    pub static ref MULTI_RUNTIME: MultiTaskRuntime<(), StealableTaskPool<()>> = AsyncRuntimeBuilder::default_multi_thread(
		None,
		None,
		None,
		None,
	);
}
