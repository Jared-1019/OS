mod context;

use core::arch::global_asm;

use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval,
    stvec,
};

use crate::syscall::syscall;
use crate::task::{suspend_current_and_run_next, exit_current_and_run_next};

pub use context::TrapContext;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            cx.x[10] = result as usize;
        }

        Trap::Exception(Exception::IllegalInstruction) => {
            exit_current_and_run_next();
        }

        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            let _ = stval;
            exit_current_and_run_next();
        }

        _ => {
            exit_current_and_run_next();
        }
    }

    cx
}
