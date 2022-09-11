#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Comparator A Control 1"]
    pub cactl1: CACTL1,
    #[doc = "0x02 - Comparator A Control 2"]
    pub cactl2: CACTL2,
    #[doc = "0x03 - Comparator A Port Disable"]
    pub capd: CAPD,
}
#[doc = "CACTL1 (rw) register accessor: an alias for `Reg<CACTL1_SPEC>`"]
pub type CACTL1 = crate::Reg<cactl1::CACTL1_SPEC>;
#[doc = "Comparator A Control 1"]
pub mod cactl1;
#[doc = "CACTL2 (rw) register accessor: an alias for `Reg<CACTL2_SPEC>`"]
pub type CACTL2 = crate::Reg<cactl2::CACTL2_SPEC>;
#[doc = "Comparator A Control 2"]
pub mod cactl2;
#[doc = "CAPD (rw) register accessor: an alias for `Reg<CAPD_SPEC>`"]
pub type CAPD = crate::Reg<capd::CAPD_SPEC>;
#[doc = "Comparator A Port Disable"]
pub mod capd;
