//! Competition Control

#[unsafe(no_mangle)]
pub extern "C" fn vexCompetitionStatus() -> u32 {
    super::sdk_unimplemented!("vexCompetitionStatus");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexCompetitionControl(data: u32) {
    super::sdk_unimplemented!("vexCompetitionControl");
}
