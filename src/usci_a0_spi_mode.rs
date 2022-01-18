#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 0"]
    pub uca0ctl0: crate::Reg<uca0ctl0::UCA0CTL0_SPEC>,
    #[doc = "0x01 - USCI A0 Control Register 1"]
    pub uca0ctl1: crate::Reg<uca0ctl1::UCA0CTL1_SPEC>,
    #[doc = "0x02 - USCI A0 Baud Rate 0"]
    pub uca0br0: crate::Reg<uca0br0::UCA0BR0_SPEC>,
    #[doc = "0x03 - USCI A0 Baud Rate 1"]
    pub uca0br1: crate::Reg<uca0br1::UCA0BR1_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x05 - USCI A0 Status Register"]
    pub uca0stat: crate::Reg<uca0stat::UCA0STAT_SPEC>,
    #[doc = "0x06 - USCI A0 Receive Buffer"]
    pub uca0rxbuf: crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>,
    #[doc = "0x07 - USCI A0 Transmit Buffer"]
    pub uca0txbuf: crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>,
}
#[doc = "UCA0CTL0 register accessor: an alias for `Reg<UCA0CTL0_SPEC>`"]
pub type UCA0CTL0 = crate::Reg<uca0ctl0::UCA0CTL0_SPEC>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "UCA0CTL1 register accessor: an alias for `Reg<UCA0CTL1_SPEC>`"]
pub type UCA0CTL1 = crate::Reg<uca0ctl1::UCA0CTL1_SPEC>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
#[doc = "UCA0BR0 register accessor: an alias for `Reg<UCA0BR0_SPEC>`"]
pub type UCA0BR0 = crate::Reg<uca0br0::UCA0BR0_SPEC>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0;
#[doc = "UCA0BR1 register accessor: an alias for `Reg<UCA0BR1_SPEC>`"]
pub type UCA0BR1 = crate::Reg<uca0br1::UCA0BR1_SPEC>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1;
#[doc = "UCA0STAT register accessor: an alias for `Reg<UCA0STAT_SPEC>`"]
pub type UCA0STAT = crate::Reg<uca0stat::UCA0STAT_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat;
#[doc = "UCA0RXBUF register accessor: an alias for `Reg<UCA0RXBUF_SPEC>`"]
pub type UCA0RXBUF = crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf;
#[doc = "UCA0TXBUF register accessor: an alias for `Reg<UCA0TXBUF_SPEC>`"]
pub type UCA0TXBUF = crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf;
