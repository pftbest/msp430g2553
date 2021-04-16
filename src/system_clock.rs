#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Basic Clock System Control 3"]
    pub bcsctl3: crate::Reg<bcsctl3::BCSCTL3_SPEC>,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - DCO Clock Frequency Control"]
    pub dcoctl: crate::Reg<dcoctl::DCOCTL_SPEC>,
    #[doc = "0x05 - Basic Clock System Control 1"]
    pub bcsctl1: crate::Reg<bcsctl1::BCSCTL1_SPEC>,
    #[doc = "0x06 - Basic Clock System Control 2"]
    pub bcsctl2: crate::Reg<bcsctl2::BCSCTL2_SPEC>,
}
#[doc = "BCSCTL3 register accessor: an alias for `Reg<BCSCTL3_SPEC>`"]
pub type BCSCTL3 = crate::Reg<bcsctl3::BCSCTL3_SPEC>;
#[doc = "Basic Clock System Control 3"]
pub mod bcsctl3;
#[doc = "DCOCTL register accessor: an alias for `Reg<DCOCTL_SPEC>`"]
pub type DCOCTL = crate::Reg<dcoctl::DCOCTL_SPEC>;
#[doc = "DCO Clock Frequency Control"]
pub mod dcoctl;
#[doc = "BCSCTL1 register accessor: an alias for `Reg<BCSCTL1_SPEC>`"]
pub type BCSCTL1 = crate::Reg<bcsctl1::BCSCTL1_SPEC>;
#[doc = "Basic Clock System Control 1"]
pub mod bcsctl1;
#[doc = "BCSCTL2 register accessor: an alias for `Reg<BCSCTL2_SPEC>`"]
pub type BCSCTL2 = crate::Reg<bcsctl2::BCSCTL2_SPEC>;
#[doc = "Basic Clock System Control 2"]
pub mod bcsctl2;
