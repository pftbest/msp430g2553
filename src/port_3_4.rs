#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Resistor Enable"]
    pub p3ren: P3REN,
    _reserved1: [u8; 0x07],
    #[doc = "0x08 - Port 3 Input"]
    pub p3in: P3IN,
    #[doc = "0x09 - Port 3 Output"]
    pub p3out: P3OUT,
    #[doc = "0x0a - Port 3 Direction"]
    pub p3dir: P3DIR,
    #[doc = "0x0b - Port 3 Selection"]
    pub p3sel: P3SEL,
    _reserved5: [u8; 0x27],
    #[doc = "0x33 - Port 3 Selection 2"]
    pub p3sel2: P3SEL2,
}
#[doc = "P3REN (rw) register accessor: an alias for `Reg<P3REN_SPEC>`"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P3IN (rw) register accessor: an alias for `Reg<P3IN_SPEC>`"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT (rw) register accessor: an alias for `Reg<P3OUT_SPEC>`"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR (rw) register accessor: an alias for `Reg<P3DIR_SPEC>`"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3SEL (rw) register accessor: an alias for `Reg<P3SEL_SPEC>`"]
pub type P3SEL = crate::Reg<p3sel::P3SEL_SPEC>;
#[doc = "Port 3 Selection"]
pub mod p3sel;
#[doc = "P3SEL2 (rw) register accessor: an alias for `Reg<P3SEL2_SPEC>`"]
pub type P3SEL2 = crate::Reg<p3sel2::P3SEL2_SPEC>;
#[doc = "Port 3 Selection 2"]
pub mod p3sel2;
