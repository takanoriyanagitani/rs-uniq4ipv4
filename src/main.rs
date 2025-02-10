use std::process::ExitCode;

use std::collections::BTreeSet;

use std::io;

fn stdin2uniq_ips() -> Result<BTreeSet<[u8; 4]>, io::Error> {
    rs_uniq4ipv4::stdin2uniq_ipv4()
}

fn bool2ips2stdout(raw_mode: bool) -> impl FnMut(&BTreeSet<[u8; 4]>) -> Result<(), io::Error> {
    move |ips| rs_uniq4ipv4::ips2stdout(ips, raw_mode)
}

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(io::Error::other)
}

fn raw_mode() -> bool {
    env_val_by_key("ENV_RAW_MODE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(false)
}

fn ips2stdout() -> impl FnMut(&BTreeSet<[u8; 4]>) -> Result<(), io::Error> {
    bool2ips2stdout(raw_mode())
}

fn stdin2uniq_ips2stdout() -> Result<(), io::Error> {
    let ips: BTreeSet<_> = stdin2uniq_ips()?;
    ips2stdout()(&ips)
}

fn main() -> ExitCode {
    stdin2uniq_ips2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
