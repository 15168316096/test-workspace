#![cfg_attr(not(feature = "native-simulator"), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::default_alloc!();

#[cfg(any(feature = "native-simulator", test))]
extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use ckb_std::{
    ckb_types::{bytes::Bytes, packed::Byte32, prelude::Unpack},
    debug,
    error::SysError,
    syscalls,
};
use core::ffi::CStr;
use spawn_cmd::SpawnCmd;

pub fn program_entry() -> i8 {
    debug!("-A- Spawn-Parent(pid:{}) Begin --", syscalls::process_id());

    let args = {
        let script = ckb_std::high_level::load_script().expect("Load script");
        let args: Bytes = script.args().unpack();
        args.to_vec()
    };
    assert!(args.len() >= 1 + 32, "args is empty"); // cmd + code hash

    let cmd: SpawnCmd = args[0].into();

    let code_hash = Byte32::new(args[1..33].to_vec().try_into().unwrap());
    let args = args[33..].to_vec();

    let rc = match cmd {
        SpawnCmd::Base => spawn_base(&code_hash, &args),
        SpawnCmd::EmptyPipe => spawn_empty_pipe(&code_hash),
        SpawnCmd::BaseIO1 => spawn_base_io1(&code_hash, &args),
        SpawnCmd::BaseIO2 => spawn_base_io2(&code_hash, &args),
        SpawnCmd::BaseIO3 => spawn_base_io3(&code_hash, &args),
    };

    debug!("-A- Spawn-Parent(pid:{}) End --", syscalls::process_id());
    rc
}

fn run_sapwn(
    code_hash: &Byte32,
    cmd: SpawnCmd,
    args: &[String],
    fds: &[u64],
) -> Result<u64, SysError> {
    let cmd: u8 = cmd.into();
    let args = [&[cmd.to_string()], args].concat();
    let args: Vec<Vec<u8>> = args
        .iter()
        .map(|s| alloc::vec![s.as_bytes(), &[0u8]].concat())
        .collect();
    let argv: Vec<&CStr> = args
        .iter()
        .map(|s| CStr::from_bytes_until_nul(&s).unwrap())
        .collect();

    ckb_std::high_level::spawn_cell(
        &code_hash.raw_data().to_vec(),
        ckb_std::ckb_types::core::ScriptHashType::Data2,
        &argv,
        &fds,
    )
}

fn new_pipe() -> ([u64; 2], [u64; 3]) {
    let mut std_fds: [u64; 2] = [0, 0];
    let mut son_fds: [u64; 3] = [0, 0, 0];
    let (r0, w0) = syscalls::pipe().unwrap();
    std_fds[0] = r0;
    son_fds[1] = w0;
    let (r1, w1) = syscalls::pipe().unwrap();
    std_fds[1] = w1;
    son_fds[0] = r1;
    (std_fds, son_fds)
}

fn spawn_base(code_hash: &Byte32, _args: &[u8]) -> i8 {
    debug!("-A- VM Version: {}", syscalls::vm_version().unwrap());

    let (std_fds, son_fds) = new_pipe();
    let pid = run_sapwn(&code_hash, SpawnCmd::Base, &[], &son_fds).expect("run spawn base");
    assert_eq!(pid, 1);

    assert!(syscalls::close(std_fds[0]).is_ok());
    assert!(syscalls::close(std_fds[1]).is_ok());

    assert_eq!(syscalls::close(son_fds[0]), Err(SysError::InvalidFd));
    assert_eq!(syscalls::close(son_fds[1]), Err(SysError::InvalidFd));

    assert_eq!(syscalls::process_id(), 0);

    0
}

fn spawn_empty_pipe(_code_hash: &Byte32) -> i8 {
    let (std_fds, son_fds) = new_pipe();

    assert_eq!(std_fds[0], 2);
    assert_eq!(son_fds[1], 3);
    assert_eq!(son_fds[0], 4);
    assert_eq!(std_fds[1], 5);

    assert!(syscalls::close(std_fds[0]).is_ok());
    assert_eq!(syscalls::close(std_fds[0]), Err(SysError::InvalidFd));
    assert!(syscalls::close(std_fds[1]).is_ok());
    assert!(syscalls::close(son_fds[0]).is_ok());
    assert!(syscalls::close(son_fds[1]).is_ok());
    0
}

fn spawn_base_io1(code_hash: &Byte32, _args: &[u8]) -> i8 {
    let (std_fds, son_fds) = new_pipe();

    let argv = ["hello".to_string(), "world".to_string()];
    debug!("-A- Spawn --");
    let pid = run_sapwn(code_hash, SpawnCmd::BaseIO1, &argv, &son_fds).expect("run spawn base io");
    debug!("-A- Spawn End, pid: {} --", pid);
    assert_eq!(pid, 1);

    debug!("-A- Read --");
    let mut buf: [u8; 256] = [0; 256];
    let len = syscalls::read(std_fds[0], &mut buf).expect("read 1");
    debug!("-A- Read End --");

    assert_eq!(len, 10);
    buf[len] = 0;
    assert_eq!(
        CStr::from_bytes_until_nul(&buf).unwrap().to_str().unwrap(),
        "helloworld"
    );

    0
}

fn spawn_base_io2(code_hash: &Byte32, _args: &[u8]) -> i8 {
    let (std_fds, son_fds) = new_pipe();

    let argv = ["hello".to_string(), "world".to_string()];
    let pid = run_sapwn(code_hash, SpawnCmd::BaseIO2, &argv, &son_fds).expect("run spawn base io");
    assert_eq!(pid, 1);

    debug!("-A- Write --");
    let write_buf = alloc::vec![argv[0].as_bytes(), argv[1].as_bytes()].concat();
    let len = syscalls::write(std_fds[1], &write_buf).expect("write");
    debug!("-A- Write End --");
    assert_eq!(len, write_buf.len());

    0
}

fn spawn_base_io3(code_hash: &Byte32, _args: &[u8]) -> i8 {
    let (std_fds, son_fds) = new_pipe();

    let argv = ["hello".to_string(), "world".to_string()];
    let pid = run_sapwn(code_hash, SpawnCmd::BaseIO3, &argv, &son_fds).expect("run spawn base io");
    assert_eq!(pid, 1);

    let write_buf = alloc::vec![argv[0].as_bytes(), argv[1].as_bytes()].concat();
    let len = syscalls::write(std_fds[1], &write_buf).expect("write");
    assert_eq!(len, write_buf.len());

    0
}
