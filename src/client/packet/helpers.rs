use binrw::BinResult;

use crate::client::packet::fields::NullTerminated;

#[binrw::writer(writer)]
pub fn null_terminated(s: &NullTerminated<String>) -> BinResult<()> {
    writer.write_all(s.0.as_bytes())?;
    writer.write_all(&[0])?;
    Ok(())
}

#[binrw::writer(writer)]
pub fn reversed_null_terminated(s: &NullTerminated<String>) -> BinResult<()> {
    let r: String = s.0.chars().rev().collect();
    writer.write_all(r.as_bytes())?;
    writer.write_all(&[0])?;
    Ok(())
}

#[binrw::writer(writer)]
pub fn reversed(s: &String) -> BinResult<()> {
    let r: String = s.chars().rev().collect();
    writer.write_all(r.as_bytes())?;
    Ok(())
}

#[binrw::writer(writer)]
pub fn just_bytes(s: &String) -> BinResult<()> {
    writer.write_all(s.as_bytes())?;
    Ok(())
}