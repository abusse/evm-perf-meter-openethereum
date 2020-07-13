extern crate cfg_if;

use crate::measurement::Measurement;

pub struct Cycles;

// WARN: does not check for the cpu feature; but we'd panic anyway so...
fn rdtsc() -> u64 {
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            unsafe {
                core::arch::x86_64::_rdtsc()
            }
        } else if #[cfg(target_arch = "x86")] {
            unsafe {
                core::arch::x86::_rdtsc()
            }
        } else if #[cfg(target_arch = "aarch64")] {
            let x: u64;
            unsafe {
                asm!("mrs {}, cntvct_el0", out(reg) x);
            }
            x
        } else {
            compile_error!("cycles counter not supported on this architecture.");
        }
    }
}

impl Measurement for Cycles {
    type Intermediate = u64;

    fn start(&self) -> Self::Intermediate {
        rdtsc()
    }

    fn end(&self, i: Self::Intermediate) -> serde_json::Number {
        serde_json::Number::from(rdtsc() - i)
    }

    fn id(&self) -> &'static str {
        "cpu_cycles"
    }

    fn name(&self) -> &'static str {
        "CPU Cycles"
    }

    fn unit(&self) -> &'static str {
        "N"
    }
}
