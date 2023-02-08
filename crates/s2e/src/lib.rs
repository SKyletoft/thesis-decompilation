use autocxx::{prelude::*, subclass::prelude::*};

autocxx::include_cpp! {
	#include "s2e/CorePlugin.h"
	#include "s2e/Plugin.h"
	#include "s2e/S2EExecutionState.h"
	safety!(unsafe_ffi)

	generate!("s2e::Plugin")
	generate!("s2e::S2E")
	generate!("s2e::ExecutionSignal")
	generate!("TranslationBlock")
	generate!("s2e::S2EExecutionState")
}

use crate::ffi::{s2e::{S2E, ExecutionSignal, S2EExecutionState, Plugin}, TranslationBlock};

#[is_subclass(superclass("s2e::Plugin"))]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Example {
	m_traceBlockTranslation: bool,
	m_traceBlockExecution: bool,
}

impl Example {
	unsafe fn new(s2e: *mut S2E) -> Self {
		todo!()
	}

	unsafe fn initialize(&mut self) { todo!() }

	unsafe fn slotTranslateBlockStart(
		&mut self,
		e: *mut ExecutionSignal,
		state: *mut S2EExecutionState,
		tb: *mut TranslationBlock,
		pc: u64,
	)  { todo!()
	}

	unsafe fn slotExecuteBlockStart(&mut self, state: *mut S2EExecutionState, pc: u64) { todo!() }
}

/*
impl CppSubclass<Plugin> for Example {
	fn peer_holder(&self) -> &CppSubclassCppPeerHolder<Plugin> { todo!() }
	fn peer_holder_mut(&mut self) -> &mut CppSubclassCppPeerHolder<Plugin> {todo!()}
}
*/