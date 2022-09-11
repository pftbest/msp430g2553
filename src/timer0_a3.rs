#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0_A3 Interrupt Vector Word"]
    pub taiv: TAIV,
    _reserved1: [u8; 0x30],
    #[doc = "0x32 - Timer0_A3 Control"]
    pub tactl: TACTL,
    #[doc = "0x34 - Timer0_A3 Capture/Compare Control 0"]
    pub tacctl0: TACCTL0,
    #[doc = "0x36 - Timer0_A3 Capture/Compare Control 1"]
    pub tacctl1: TACCTL1,
    #[doc = "0x38 - Timer0_A3 Capture/Compare Control 2"]
    pub tacctl2: TACCTL2,
    _reserved5: [u8; 0x08],
    #[doc = "0x42 - Timer0_A3 Counter Register"]
    pub tar: TAR,
    #[doc = "0x44 - Timer0_A3 Capture/Compare 0"]
    pub taccr0: TACCR0,
    #[doc = "0x46 - Timer0_A3 Capture/Compare 1"]
    pub taccr1: TACCR1,
    #[doc = "0x48 - Timer0_A3 Capture/Compare 2"]
    pub taccr2: TACCR2,
}
#[doc = "TAIV (rw) register accessor: an alias for `Reg<TAIV_SPEC>`"]
pub type TAIV = crate::Reg<taiv::TAIV_SPEC>;
#[doc = "Timer0_A3 Interrupt Vector Word"]
pub mod taiv;
#[doc = "TACTL (rw) register accessor: an alias for `Reg<TACTL_SPEC>`"]
pub type TACTL = crate::Reg<tactl::TACTL_SPEC>;
#[doc = "Timer0_A3 Control"]
pub mod tactl;
#[doc = "TACCTL0 (rw) register accessor: an alias for `Reg<TACCTL0_SPEC>`"]
pub type TACCTL0 = crate::Reg<tacctl0::TACCTL0_SPEC>;
#[doc = "Timer0_A3 Capture/Compare Control 0"]
pub mod tacctl0;
#[doc = "TACCTL1 (rw) register accessor: an alias for `Reg<TACCTL1_SPEC>`"]
pub type TACCTL1 = crate::Reg<tacctl1::TACCTL1_SPEC>;
#[doc = "Timer0_A3 Capture/Compare Control 1"]
pub mod tacctl1;
#[doc = "TACCTL2 (rw) register accessor: an alias for `Reg<TACCTL2_SPEC>`"]
pub type TACCTL2 = crate::Reg<tacctl2::TACCTL2_SPEC>;
#[doc = "Timer0_A3 Capture/Compare Control 2"]
pub mod tacctl2;
#[doc = "TAR (rw) register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "Timer0_A3 Counter Register"]
pub mod tar;
#[doc = "TACCR0 (rw) register accessor: an alias for `Reg<TACCR0_SPEC>`"]
pub type TACCR0 = crate::Reg<taccr0::TACCR0_SPEC>;
#[doc = "Timer0_A3 Capture/Compare 0"]
pub mod taccr0;
#[doc = "TACCR1 (rw) register accessor: an alias for `Reg<TACCR1_SPEC>`"]
pub type TACCR1 = crate::Reg<taccr1::TACCR1_SPEC>;
#[doc = "Timer0_A3 Capture/Compare 1"]
pub mod taccr1;
#[doc = "TACCR2 (rw) register accessor: an alias for `Reg<TACCR2_SPEC>`"]
pub type TACCR2 = crate::Reg<taccr2::TACCR2_SPEC>;
#[doc = "Timer0_A3 Capture/Compare 2"]
pub mod taccr2;
