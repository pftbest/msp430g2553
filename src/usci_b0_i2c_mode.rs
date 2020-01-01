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
    _reserved8: [u8; 168usize],
    #[doc = "0xb0 - USCI B0 I2C Own Address"]
    pub ucb0i2coa: UCB0I2COA,
    #[doc = "0xb2 - USCI B0 I2C Slave Address"]
    pub ucb0i2csa: UCB0I2CSA,
}
#[doc = "USCI B0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl0](ucb0ctl0) module"]
pub type UCB0CTL0 = crate::Reg<u8, _UCB0CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTL0;
#[doc = "`read()` method returns [ucb0ctl0::R](ucb0ctl0::R) reader structure"]
impl crate::Readable for UCB0CTL0 {}
#[doc = "`write(|w| ..)` method takes [ucb0ctl0::W](ucb0ctl0::W) writer structure"]
impl crate::Writable for UCB0CTL0 {}
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "USCI B0 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl1](ucb0ctl1) module"]
pub type UCB0CTL1 = crate::Reg<u8, _UCB0CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTL1;
#[doc = "`read()` method returns [ucb0ctl1::R](ucb0ctl1::R) reader structure"]
impl crate::Readable for UCB0CTL1 {}
#[doc = "`write(|w| ..)` method takes [ucb0ctl1::W](ucb0ctl1::W) writer structure"]
impl crate::Writable for UCB0CTL1 {}
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "USCI B0 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0br0](ucb0br0) module"]
pub type UCB0BR0 = crate::Reg<u8, _UCB0BR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BR0;
#[doc = "`read()` method returns [ucb0br0::R](ucb0br0::R) reader structure"]
impl crate::Readable for UCB0BR0 {}
#[doc = "`write(|w| ..)` method takes [ucb0br0::W](ucb0br0::W) writer structure"]
impl crate::Writable for UCB0BR0 {}
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "USCI B0 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0br1](ucb0br1) module"]
pub type UCB0BR1 = crate::Reg<u8, _UCB0BR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BR1;
#[doc = "`read()` method returns [ucb0br1::R](ucb0br1::R) reader structure"]
impl crate::Readable for UCB0BR1 {}
#[doc = "`write(|w| ..)` method takes [ucb0br1::W](ucb0br1::W) writer structure"]
impl crate::Writable for UCB0BR1 {}
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "USCI B0 I2C Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2cie](ucb0i2cie) module"]
pub type UCB0I2CIE = crate::Reg<u8, _UCB0I2CIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2CIE;
#[doc = "`read()` method returns [ucb0i2cie::R](ucb0i2cie::R) reader structure"]
impl crate::Readable for UCB0I2CIE {}
#[doc = "`write(|w| ..)` method takes [ucb0i2cie::W](ucb0i2cie::W) writer structure"]
impl crate::Writable for UCB0I2CIE {}
#[doc = "USCI B0 I2C Interrupt Enable Register"]
pub mod ucb0i2cie;
#[doc = "USCI B0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0stat](ucb0stat) module"]
pub type UCB0STAT = crate::Reg<u8, _UCB0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0STAT;
#[doc = "`read()` method returns [ucb0stat::R](ucb0stat::R) reader structure"]
impl crate::Readable for UCB0STAT {}
#[doc = "`write(|w| ..)` method takes [ucb0stat::W](ucb0stat::W) writer structure"]
impl crate::Writable for UCB0STAT {}
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat;
#[doc = "USCI B0 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0rxbuf](ucb0rxbuf) module"]
pub type UCB0RXBUF = crate::Reg<u8, _UCB0RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0RXBUF;
#[doc = "`read()` method returns [ucb0rxbuf::R](ucb0rxbuf::R) reader structure"]
impl crate::Readable for UCB0RXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb0rxbuf::W](ucb0rxbuf::W) writer structure"]
impl crate::Writable for UCB0RXBUF {}
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "USCI B0 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0txbuf](ucb0txbuf) module"]
pub type UCB0TXBUF = crate::Reg<u8, _UCB0TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0TXBUF;
#[doc = "`read()` method returns [ucb0txbuf::R](ucb0txbuf::R) reader structure"]
impl crate::Readable for UCB0TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb0txbuf::W](ucb0txbuf::W) writer structure"]
impl crate::Writable for UCB0TXBUF {}
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "USCI B0 I2C Own Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2coa](ucb0i2coa) module"]
pub type UCB0I2COA = crate::Reg<u16, _UCB0I2COA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA;
#[doc = "`read()` method returns [ucb0i2coa::R](ucb0i2coa::R) reader structure"]
impl crate::Readable for UCB0I2COA {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa::W](ucb0i2coa::W) writer structure"]
impl crate::Writable for UCB0I2COA {}
#[doc = "USCI B0 I2C Own Address"]
pub mod ucb0i2coa;
#[doc = "USCI B0 I2C Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2csa](ucb0i2csa) module"]
pub type UCB0I2CSA = crate::Reg<u16, _UCB0I2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2CSA;
#[doc = "`read()` method returns [ucb0i2csa::R](ucb0i2csa::R) reader structure"]
impl crate::Readable for UCB0I2CSA {}
#[doc = "`write(|w| ..)` method takes [ucb0i2csa::W](ucb0i2csa::W) writer structure"]
impl crate::Writable for UCB0I2CSA {}
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
