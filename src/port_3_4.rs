#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Resistor Enable"]
    pub p3ren: crate::Reg<p3ren::P3REN_SPEC>,
    _reserved1: [u8; 7usize],
    #[doc = "0x08 - Port 3 Input"]
    pub p3in: crate::Reg<p3in::P3IN_SPEC>,
    #[doc = "0x09 - Port 3 Output"]
    pub p3out: crate::Reg<p3out::P3OUT_SPEC>,
    #[doc = "0x0a - Port 3 Direction"]
    pub p3dir: crate::Reg<p3dir::P3DIR_SPEC>,
    #[doc = "0x0b - Port 3 Selection"]
    pub p3sel: crate::Reg<p3sel::P3SEL_SPEC>,
    _reserved5: [u8; 39usize],
    #[doc = "0x33 - Port 3 Selection 2"]
    pub p3sel2: crate::Reg<p3sel2::P3SEL2_SPEC>,
}
#[doc = "P3REN register accessor: an alias for `Reg<P3REN_SPEC>`"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P3IN register accessor: an alias for `Reg<P3IN_SPEC>`"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT register accessor: an alias for `Reg<P3OUT_SPEC>`"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR register accessor: an alias for `Reg<P3DIR_SPEC>`"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3SEL register accessor: an alias for `Reg<P3SEL_SPEC>`"]
pub type P3SEL = crate::Reg<p3sel::P3SEL_SPEC>;
#[doc = "Port 3 Selection"]
pub mod p3sel;
#[doc = "P3SEL2 register accessor: an alias for `Reg<P3SEL2_SPEC>`"]
pub type P3SEL2 = crate::Reg<p3sel2::P3SEL2_SPEC>;
#[doc = "Port 3 Selection 2"]
pub mod p3sel2;
