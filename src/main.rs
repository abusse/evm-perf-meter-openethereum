#![feature(asm)]

#[macro_use]
extern crate cfg_if;

extern crate ethereum_types;
extern crate evm;
extern crate parity_bytes as bytes;
extern crate rustc_hex;
extern crate serde;
extern crate serde_json;
extern crate sys_info;
extern crate vm;

mod bare_evm;
mod measurement;
mod walltime;

mod cycles;

use crate::bare_evm::ZeroExt;
use crate::cycles::Cycles;
use crate::walltime::WallTime;
use ethereum_types::{Address, U256};
use evm::Factory;
use rustc_hex::FromHex;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::sync::Arc;
use vm::{ActionParams, Ext, GasLeft};

cfg_if! {
    if #[cfg(target_os = "linux")] {
        extern crate perfcnt;

        mod perf;

        use crate::perf::Perf;
        use perfcnt::linux::PerfCounterBuilderLinux as Builder;
        use perfcnt::linux::SoftwareEventType as Software;

        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        use perfcnt::linux::HardwareEventType as Hardware;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Run {
    iterations: u64,
    code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    id: String,
    name: String,
    unit: String,
    hostname: String,
    values: Vec<Number>,
    gas: Vec<U256>,
}

fn run_benchmark<I>(
    code: &Vec<u8>,
    iterations: u64,
    measure: Box<dyn crate::measurement::Measurement<Intermediate = I>>,
) -> Result {
    let mut values: Vec<Number> = Vec::new();
    let mut gas: Vec<U256> = Vec::new();
    let factory = Factory::default();
    let mut ext = ZeroExt::new();

    let address = Address::from_str("4f2045b7faefb00a230f910505db1a09c2790fc4").unwrap();

    for _ in 0..iterations {
        let mut params = ActionParams::default();
        params.address = address.clone();
        params.gas = U256::MAX;
        params.code = Some(Arc::new(code.clone()));

        let vm = factory.create(params, ext.schedule(), ext.depth());

        let start = measure.start();

        let execution = vm.exec(&mut ext);

        let value = measure.end(start);

        let execution = match execution {
            Ok(r) => r,
            Err(_) => panic!("EVM code failed to execute"),
        };

        let execution = match execution {
            Ok(r) => r,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let gas_left = match execution {
            GasLeft::Known(gas_left) => gas_left,
            GasLeft::NeedsReturn { gas_left, .. } => gas_left,
        };

        values.push(value);
        gas.push(U256::MAX - gas_left);
    }

    Result {
        id: measure.id().to_string(),
        name: measure.name().to_string(),
        unit: measure.unit().to_string(),
        hostname: match sys_info::hostname() {
            Ok(h) => h,
            Err(_e) => "unkown".to_string(),
        },
        values: values,
        gas: gas,
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let run: Run = match serde_json::from_str(&input) {
        Err(e) => panic!("Malformed input: {}", e),
        Ok(r) => r,
    };

    let iterations = run.iterations;
    let code = String::from(run.code.replace("0x", "")).from_hex().unwrap();

    let mut results = Vec::new();

    results.push(run_benchmark(&code, iterations, Box::new(WallTime)));

    results.push(run_benchmark(&code, iterations, Box::new(Cycles)));

    cfg_if! {
        if #[cfg(target_os = "linux")] {
            let perf_clock = Perf::new(Builder::from_software_event(Software::TaskClock), "task_clock", "Task Clock", "ns");
            results.push(run_benchmark(&code, iterations, Box::new(perf_clock)));

            cfg_if! {
                if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                    let perf_cache_references = Perf::new(Builder::from_hardware_event(Hardware::CacheReferences), "cache_references", "Cache References", "N");
                    let perf_cache_misses = Perf::new(Builder::from_hardware_event(Hardware::CacheMisses), "cache_misses", "Cache Misses", "N");

                    results.push(run_benchmark(&code, iterations, Box::new(perf_cache_references)));
                    results.push(run_benchmark(&code, iterations, Box::new(perf_cache_misses)));
                }
            }
        }
    }

    match serde_json::to_string(&results) {
        Err(e) => println!("{}", e),
        Ok(j) => println!("{}", j),
    };
}
