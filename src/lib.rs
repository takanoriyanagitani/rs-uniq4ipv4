use std::collections::BTreeSet;

use std::net::Ipv4Addr;

use std::io;

use std::io::BufWriter;
use std::io::Write;

use std::io::Read;

use std::io::ErrorKind;

pub fn reader2ips4<R>(mut rdr: R) -> impl Iterator<Item = Result<[u8; 4], io::Error>>
where
    R: Read,
{
    std::iter::from_fn(move || {
        let mut buf: [u8; 4] = [0; 4];
        let rslt: Result<[u8; 4], io::Error> = rdr.read_exact(&mut buf).map(|_| buf);
        match rslt {
            Ok(ipv4) => Some(Ok(ipv4)),
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => None,
                _ => Some(Err(e)),
            },
        }
    })
}

pub fn uniq_ipv4<I>(ips: I) -> Result<BTreeSet<[u8; 4]>, io::Error>
where
    I: Iterator<Item = Result<[u8; 4], io::Error>>,
{
    ips.collect()
}

pub fn reader2uniq_ipv4<R>(rdr: R) -> Result<BTreeSet<[u8; 4]>, io::Error>
where
    R: Read,
{
    let ips = reader2ips4(rdr);
    uniq_ipv4(ips)
}

pub fn stdin2uniq_ipv4() -> Result<BTreeSet<[u8; 4]>, io::Error> {
    let i = io::stdin();
    let il = i.lock();
    reader2uniq_ipv4(il)
}

pub fn ips2wtr_fn<W>(ips: &BTreeSet<[u8; 4]>, mut wtr: W) -> Result<(), io::Error>
where
    W: FnMut(&[u8; 4]) -> Result<(), io::Error>,
{
    for ip in ips {
        wtr(ip)?;
    }
    Ok(())
}

pub fn ips2wtr<W>(ips: &BTreeSet<[u8; 4]>, mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    ips2wtr_fn(ips, |i: &[u8; 4]| {
        let ipv4: Ipv4Addr = Ipv4Addr::new(i[0], i[1], i[2], i[3]);
        writeln!(wtr, "{ipv4}")?;
        Ok(())
    })?;
    wtr.flush()
}

pub fn ips2wtr_raw<W>(ips: &BTreeSet<[u8; 4]>, mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    ips2wtr_fn(ips, |i: &[u8; 4]| {
        wtr.write_all(i)?;
        Ok(())
    })?;
    wtr.flush()
}

pub fn ips2stdout_raw(ips: &BTreeSet<[u8; 4]>) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    ips2wtr_raw(ips, bw)?;

    ol.flush()
}

pub fn ips2stdout_str(ips: &BTreeSet<[u8; 4]>) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    ips2wtr(ips, bw)?;

    ol.flush()
}

pub fn ips2stdout(ips: &BTreeSet<[u8; 4]>, raw_mode: bool) -> Result<(), io::Error> {
    match raw_mode {
        true => ips2stdout_raw(ips),
        false => ips2stdout_str(ips),
    }
}
