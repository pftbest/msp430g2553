#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH Control 1"]
    pub fctl1: crate::Reg<fctl1::FCTL1_SPEC>,
    #[doc = "0x02 - FLASH Control 2"]
    pub fctl2: crate::Reg<fctl2::FCTL2_SPEC>,
    #[doc = "0x04 - FLASH Control 3"]
    pub fctl3: crate::Reg<fctl3::FCTL3_SPEC>,
}
#[doc = "FCTL1 register accessor: an alias for `Reg<FCTL1_SPEC>`"]
pub type FCTL1 = crate::Reg<fctl1::FCTL1_SPEC>;
#[doc = "FLASH Control 1"]
pub mod fctl1;
#[doc = "FCTL2 register accessor: an alias for `Reg<FCTL2_SPEC>`"]
pub type FCTL2 = crate::Reg<fctl2::FCTL2_SPEC>;
#[doc = "FLASH Control 2"]
pub mod fctl2;
#[doc = "FCTL3 register accessor: an alias for `Reg<FCTL3_SPEC>`"]
pub type FCTL3 = crate::Reg<fctl3::FCTL3_SPEC>;
#[doc = "FLASH Control 3"]
pub mod fctl3;
