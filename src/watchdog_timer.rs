#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control"]
    pub wdtctl: crate::Reg<wdtctl::WDTCTL_SPEC>,
}
#[doc = "WDTCTL register accessor: an alias for `Reg<WDTCTL_SPEC>`"]
pub type WDTCTL = crate::Reg<wdtctl::WDTCTL_SPEC>;
#[doc = "Watchdog Timer Control"]
pub mod wdtctl;
