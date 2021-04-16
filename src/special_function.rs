#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    pub ie1: crate::Reg<ie1::IE1_SPEC>,
    #[doc = "0x01 - Interrupt Enable 2"]
    pub ie2: crate::Reg<ie2::IE2_SPEC>,
    #[doc = "0x02 - Interrupt Flag 1"]
    pub ifg1: crate::Reg<ifg1::IFG1_SPEC>,
    #[doc = "0x03 - Interrupt Flag 2"]
    pub ifg2: crate::Reg<ifg2::IFG2_SPEC>,
}
#[doc = "IE1 register accessor: an alias for `Reg<IE1_SPEC>`"]
pub type IE1 = crate::Reg<ie1::IE1_SPEC>;
#[doc = "Interrupt Enable 1"]
pub mod ie1;
#[doc = "IE2 register accessor: an alias for `Reg<IE2_SPEC>`"]
pub type IE2 = crate::Reg<ie2::IE2_SPEC>;
#[doc = "Interrupt Enable 2"]
pub mod ie2;
#[doc = "IFG1 register accessor: an alias for `Reg<IFG1_SPEC>`"]
pub type IFG1 = crate::Reg<ifg1::IFG1_SPEC>;
#[doc = "Interrupt Flag 1"]
pub mod ifg1;
#[doc = "IFG2 register accessor: an alias for `Reg<IFG2_SPEC>`"]
pub type IFG2 = crate::Reg<ifg2::IFG2_SPEC>;
#[doc = "Interrupt Flag 2"]
pub mod ifg2;
