use std::collections::HashMap;
use std::sync::Arc;

use bytes::Bytes;
use ethereum_types::{Address, H256, U256};
use vm::{
    self, CallType, ContractCreateResult, CreateContractAddress, EnvInfo, Ext, MessageCallResult,
    ReturnData, Schedule, TrapKind,
};

/// Zero externalities structure.
#[derive(Default)]
pub struct ZeroExt {
    pub store: HashMap<H256, H256>,
    pub depth: usize,
    pub blockhashes: HashMap<U256, H256>,
    pub codes: HashMap<Address, Arc<Bytes>>,
    pub info: EnvInfo,
    pub schedule: Schedule,
    pub balances: HashMap<Address, U256>,
    pub tracing: bool,
}

impl ZeroExt {
    /// New zero externalities
    pub fn new() -> Self {
        ZeroExt::default()
    }
}

/// We will basically panic for everything that is somehow external
impl Ext for ZeroExt {
    fn initial_storage_at(&self, _key: &H256) -> vm::Result<H256> {
        unimplemented!();
    }

    fn storage_at(&self, _key: &H256) -> vm::Result<H256> {
        unimplemented!();
    }

    fn set_storage(&mut self, _key: H256, _value: H256) -> vm::Result<()> {
        unimplemented!();
    }

    fn exists(&self, _address: &Address) -> vm::Result<bool> {
        unimplemented!();
    }

    fn exists_and_not_null(&self, _address: &Address) -> vm::Result<bool> {
        unimplemented!();
    }

    fn origin_balance(&self) -> vm::Result<U256> {
        unimplemented!()
    }

    fn balance(&self, _address: &Address) -> vm::Result<U256> {
        unimplemented!()
    }

    fn blockhash(&mut self, _number: &U256) -> H256 {
        unimplemented!()
    }

    fn create(
        &mut self,
        _gas: &U256,
        _value: &U256,
        _code: &[u8],
        _address: CreateContractAddress,
        _trap: bool,
    ) -> ::std::result::Result<ContractCreateResult, TrapKind> {
        unimplemented!();
    }

    fn call(
        &mut self,
        _gas: &U256,
        _sender_address: &Address,
        _receive_address: &Address,
        _value: Option<U256>,
        _data: &[u8],
        _code_address: &Address,
        _call_type: CallType,
        _trap: bool,
    ) -> ::std::result::Result<MessageCallResult, TrapKind> {
        unimplemented!();
    }

    fn extcode(&self, _address: &Address) -> vm::Result<Option<Arc<Bytes>>> {
        unimplemented!()
    }

    fn extcodesize(&self, _address: &Address) -> vm::Result<Option<usize>> {
        unimplemented!()
    }

    fn extcodehash(&self, _address: &Address) -> vm::Result<Option<H256>> {
        unimplemented!()
    }

    fn log(&mut self, _topics: Vec<H256>, _data: &[u8]) -> vm::Result<()> {
        unimplemented!()
    }

    fn ret(self, _gas: &U256, _data: &ReturnData, _apply_state: bool) -> vm::Result<U256> {
        unimplemented!()
    }

    fn suicide(&mut self, _refund_address: &Address) -> vm::Result<()> {
        unimplemented!()
    }

    fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    fn env_info(&self) -> &EnvInfo {
        &self.info
    }

    fn chain_id(&self) -> u64 {
        unimplemented!()
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn is_static(&self) -> bool {
        unimplemented!()
    }

    fn add_sstore_refund(&mut self, _value: usize) {
        unimplemented!()
    }

    fn sub_sstore_refund(&mut self, _value: usize) {
        unimplemented!()
    }

    fn trace_next_instruction(&mut self, _pc: usize, _instruction: u8, _gas: U256) -> bool {
        self.tracing
    }
}
