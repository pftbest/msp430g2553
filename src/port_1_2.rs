#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 1 Input"]
    pub p1in: P1IN,
    #[doc = "0x01 - Port 1 Output"]
    pub p1out: P1OUT,
    #[doc = "0x02 - Port 1 Direction"]
    pub p1dir: P1DIR,
    #[doc = "0x03 - Port 1 Interrupt Flag"]
    pub p1ifg: P1IFG,
    #[doc = "0x04 - Port 1 Interrupt Edge Select"]
    pub p1ies: P1IES,
    #[doc = "0x05 - Port 1 Interrupt Enable"]
    pub p1ie: P1IE,
    #[doc = "0x06 - Port 1 Selection"]
    pub p1sel: P1SEL,
    #[doc = "0x07 - Port 1 Resistor Enable"]
    pub p1ren: P1REN,
    #[doc = "0x08 - Port 2 Input"]
    pub p2in: P2IN,
    #[doc = "0x09 - Port 2 Output"]
    pub p2out: P2OUT,
    #[doc = "0x0a - Port 2 Direction"]
    pub p2dir: P2DIR,
    #[doc = "0x0b - Port 2 Interrupt Flag"]
    pub p2ifg: P2IFG,
    #[doc = "0x0c - Port 2 Interrupt Edge Select"]
    pub p2ies: P2IES,
    #[doc = "0x0d - Port 2 Interrupt Enable"]
    pub p2ie: P2IE,
    #[doc = "0x0e - Port 2 Selection"]
    pub p2sel: P2SEL,
    #[doc = "0x0f - Port 2 Resistor Enable"]
    pub p2ren: P2REN,
    _reserved16: [u8; 0x11],
    #[doc = "0x21 - Port 1 Selection 2"]
    pub p1sel2: P1SEL2,
    #[doc = "0x22 - Port 2 Selection 2"]
    pub p2sel2: P2SEL2,
}
#[doc = "P1IN (rw) register accessor: an alias for `Reg<P1IN_SPEC>`"]
pub type P1IN = crate::Reg<p1in::P1IN_SPEC>;
#[doc = "Port 1 Input"]
pub mod p1in;
#[doc = "P1OUT (rw) register accessor: an alias for `Reg<P1OUT_SPEC>`"]
pub type P1OUT = crate::Reg<p1out::P1OUT_SPEC>;
#[doc = "Port 1 Output"]
pub mod p1out;
#[doc = "P1DIR (rw) register accessor: an alias for `Reg<P1DIR_SPEC>`"]
pub type P1DIR = crate::Reg<p1dir::P1DIR_SPEC>;
#[doc = "Port 1 Direction"]
pub mod p1dir;
#[doc = "P1IFG (rw) register accessor: an alias for `Reg<P1IFG_SPEC>`"]
pub type P1IFG = crate::Reg<p1ifg::P1IFG_SPEC>;
#[doc = "Port 1 Interrupt Flag"]
pub mod p1ifg;
#[doc = "P1IES (rw) register accessor: an alias for `Reg<P1IES_SPEC>`"]
pub type P1IES = crate::Reg<p1ies::P1IES_SPEC>;
#[doc = "Port 1 Interrupt Edge Select"]
pub mod p1ies;
#[doc = "P1IE (rw) register accessor: an alias for `Reg<P1IE_SPEC>`"]
pub type P1IE = crate::Reg<p1ie::P1IE_SPEC>;
#[doc = "Port 1 Interrupt Enable"]
pub mod p1ie;
#[doc = "P1SEL (rw) register accessor: an alias for `Reg<P1SEL_SPEC>`"]
pub type P1SEL = crate::Reg<p1sel::P1SEL_SPEC>;
#[doc = "Port 1 Selection"]
pub mod p1sel;
#[doc = "P1REN (rw) register accessor: an alias for `Reg<P1REN_SPEC>`"]
pub type P1REN = crate::Reg<p1ren::P1REN_SPEC>;
#[doc = "Port 1 Resistor Enable"]
pub mod p1ren;
#[doc = "P2IN (rw) register accessor: an alias for `Reg<P2IN_SPEC>`"]
pub type P2IN = crate::Reg<p2in::P2IN_SPEC>;
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "P2OUT (rw) register accessor: an alias for `Reg<P2OUT_SPEC>`"]
pub type P2OUT = crate::Reg<p2out::P2OUT_SPEC>;
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "P2DIR (rw) register accessor: an alias for `Reg<P2DIR_SPEC>`"]
pub type P2DIR = crate::Reg<p2dir::P2DIR_SPEC>;
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "P2IFG (rw) register accessor: an alias for `Reg<P2IFG_SPEC>`"]
pub type P2IFG = crate::Reg<p2ifg::P2IFG_SPEC>;
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "P2IES (rw) register accessor: an alias for `Reg<P2IES_SPEC>`"]
pub type P2IES = crate::Reg<p2ies::P2IES_SPEC>;
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "P2IE (rw) register accessor: an alias for `Reg<P2IE_SPEC>`"]
pub type P2IE = crate::Reg<p2ie::P2IE_SPEC>;
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "P2SEL (rw) register accessor: an alias for `Reg<P2SEL_SPEC>`"]
pub type P2SEL = crate::Reg<p2sel::P2SEL_SPEC>;
#[doc = "Port 2 Selection"]
pub mod p2sel;
#[doc = "P2REN (rw) register accessor: an alias for `Reg<P2REN_SPEC>`"]
pub type P2REN = crate::Reg<p2ren::P2REN_SPEC>;
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
#[doc = "P1SEL2 (rw) register accessor: an alias for `Reg<P1SEL2_SPEC>`"]
pub type P1SEL2 = crate::Reg<p1sel2::P1SEL2_SPEC>;
#[doc = "Port 1 Selection 2"]
pub mod p1sel2;
#[doc = "P2SEL2 (rw) register accessor: an alias for `Reg<P2SEL2_SPEC>`"]
pub type P2SEL2 = crate::Reg<p2sel2::P2SEL2_SPEC>;
#[doc = "Port 2 Selection 2"]
pub mod p2sel2;
