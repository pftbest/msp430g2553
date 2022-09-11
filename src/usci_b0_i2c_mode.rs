#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 0"]
    pub ucb0ctl0: UCB0CTL0,
    #[doc = "0x01 - USCI B0 Control Register 1"]
    pub ucb0ctl1: UCB0CTL1,
    #[doc = "0x02 - USCI B0 Baud Rate 0"]
    pub ucb0br0: UCB0BR0,
    #[doc = "0x03 - USCI B0 Baud Rate 1"]
    pub ucb0br1: UCB0BR1,
    #[doc = "0x04 - USCI B0 I2C Interrupt Enable Register"]
    pub ucb0i2cie: UCB0I2CIE,
    #[doc = "0x05 - USCI B0 Status Register"]
    pub ucb0stat: UCB0STAT,
    #[doc = "0x06 - USCI B0 Receive Buffer"]
    pub ucb0rxbuf: UCB0RXBUF,
    #[doc = "0x07 - USCI B0 Transmit Buffer"]
    pub ucb0txbuf: UCB0TXBUF,
    _reserved8: [u8; 0xa8],
    #[doc = "0xb0 - USCI B0 I2C Own Address"]
    pub ucb0i2coa: UCB0I2COA,
    #[doc = "0xb2 - USCI B0 I2C Slave Address"]
    pub ucb0i2csa: UCB0I2CSA,
}
#[doc = "UCB0CTL0 (rw) register accessor: an alias for `Reg<UCB0CTL0_SPEC>`"]
pub type UCB0CTL0 = crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0CTL1 (rw) register accessor: an alias for `Reg<UCB0CTL1_SPEC>`"]
pub type UCB0CTL1 = crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0BR0 (rw) register accessor: an alias for `Reg<UCB0BR0_SPEC>`"]
pub type UCB0BR0 = crate::Reg<ucb0br0::UCB0BR0_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 (rw) register accessor: an alias for `Reg<UCB0BR1_SPEC>`"]
pub type UCB0BR1 = crate::Reg<ucb0br1::UCB0BR1_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0I2CIE (rw) register accessor: an alias for `Reg<UCB0I2CIE_SPEC>`"]
pub type UCB0I2CIE = crate::Reg<ucb0i2cie::UCB0I2CIE_SPEC>;
#[doc = "USCI B0 I2C Interrupt Enable Register"]
pub mod ucb0i2cie;
#[doc = "UCB0STAT (rw) register accessor: an alias for `Reg<UCB0STAT_SPEC>`"]
pub type UCB0STAT = crate::Reg<ucb0stat::UCB0STAT_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat;
#[doc = "UCB0RXBUF (rw) register accessor: an alias for `Reg<UCB0RXBUF_SPEC>`"]
pub type UCB0RXBUF = crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF (rw) register accessor: an alias for `Reg<UCB0TXBUF_SPEC>`"]
pub type UCB0TXBUF = crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "UCB0I2COA (rw) register accessor: an alias for `Reg<UCB0I2COA_SPEC>`"]
pub type UCB0I2COA = crate::Reg<ucb0i2coa::UCB0I2COA_SPEC>;
#[doc = "USCI B0 I2C Own Address"]
pub mod ucb0i2coa;
#[doc = "UCB0I2CSA (rw) register accessor: an alias for `Reg<UCB0I2CSA_SPEC>`"]
pub type UCB0I2CSA = crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>;
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
