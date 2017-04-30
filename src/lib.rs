extern crate nix;
extern crate docopt;
extern crate cargo;
extern crate rustc_serialize;
extern crate gimli;
extern crate object;
extern crate memmap;
extern crate fallible_iterator;
extern crate rustc_demangle;

use std::io;
use std::ptr;
use std::ffi::CString;
use std::path::Path;
use std::collections::HashMap;
use nix::unistd::*;
use nix::libc::pid_t;
use nix::sys::signal;
use nix::sys::wait::*;
use nix::sys::ptrace::ptrace;
use nix::sys::ptrace::ptrace::*;

pub mod tracer;
pub mod collectors;
pub mod breakpoint;
/// Should be unnecessary with a future nix crate release.
mod personality;

use tracer::*;
use breakpoint::*;


pub fn get_test_coverage(root: &Path, test: &Path) {
    match fork() {
        Ok(ForkResult::Parent{ child }) => {
            match collect_coverage(root, test, child) {
                Ok(_) => println!("Coverage successful"),
                Err(e) => println!("Error occurred: \n{}", e),
            }
        }
        Ok(ForkResult::Child) => {
            execute_test(test, true);
        }
        Err(err) => { 
            println!("Failed to run {}", test.display());
            println!("Error {}", err);
        }
    }
}

fn collect_coverage(project_path: &Path, 
                    test_path: &Path, 
                    test: pid_t) -> io::Result<()> {
    let traces = generate_tracer_data(project_path, test_path)?;
    let mut bps: HashMap<u64, Breakpoint> = HashMap::new();
    match waitpid(test, None) {
        Ok(WaitStatus::Stopped(child, signal::SIGTRAP)) => {
            println!("Running test without analysing for now");
            for trace in traces.iter() {
                match Breakpoint::new(test, trace.address) {
                    Ok(bp) => { 
                        let _ = bps.insert(trace.address, bp);
                    },
                    Err(e) => println!("Failed to add trace: {}", e),
                }
            }
            ptrace(PTRACE_CONT, child, ptr::null_mut(), ptr::null_mut())
                .ok()
                .expect("Failed to continue test");
        }
        Ok(_) => println!("Unexpected grab"),   
        Err(err) => println!("{}", err)
    }
    // Now we start hitting lines!
    loop {
        match waitpid(test, None) {
            Ok(WaitStatus::Stopped(child, signal::SIGTRAP)) => {
                println!("Hit an instrumentation point");
                ptrace(PTRACE_CONT, child, ptr::null_mut(), ptr::null_mut())
                    .ok()
                    .expect("Failed to continue test");
                   
            },
            Ok(WaitStatus::Exited(_, _)) => {
                println!("Test finished");
                break;
            },
            _ => {},
        }
    }
    Ok(())
}

fn execute_test(test: &Path, backtrace_on: bool) {
    let exec_path = CString::new(test.to_str().unwrap()).unwrap();

    if let Err(e) = personality::disable_aslr() {
        println!("Disable ASLR failed: {}", e);
    }
    ptrace(PTRACE_TRACEME, 0, ptr::null_mut(), ptr::null_mut())
        .ok()
        .expect("Failed to trace");

    let envars: Vec<CString> = if backtrace_on {
        vec![CString::new("RUST_BACKTRACE=1").unwrap()]
    } else {
        vec![]
    };
    execve(&exec_path, &[exec_path.clone()], envars.as_slice())
        .unwrap();
}