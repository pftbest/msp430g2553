#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC10 Data Transfer Control 0"]
    pub adc10dtc0: ADC10DTC0,
    #[doc = "0x01 - ADC10 Data Transfer Control 1"]
    pub adc10dtc1: ADC10DTC1,
    #[doc = "0x02 - ADC10 Analog Enable 0"]
    pub adc10ae0: ADC10AE0,
    _reserved3: [u8; 357usize],
    #[doc = "0x168 - ADC10 Control 0"]
    pub adc10ctl0: ADC10CTL0,
    #[doc = "0x16a - ADC10 Control 1"]
    pub adc10ctl1: ADC10CTL1,
    #[doc = "0x16c - ADC10 Memory"]
    pub adc10mem: ADC10MEM,
    _reserved6: [u8; 6usize],
    #[doc = "0x174 - ADC10 Data Transfer Start Address"]
    pub adc10sa: ADC10SA,
}
#[doc = "ADC10 Data Transfer Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10dtc0](adc10dtc0) module"]
pub type ADC10DTC0 = crate::Reg<u8, _ADC10DTC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC10DTC0;
#[doc = "`read()` method returns [adc10dtc0::R](adc10dtc0::R) reader structure"]
impl crate::Readable for ADC10DTC0 {}
#[doc = "`write(|w| ..)` method takes [adc10dtc0::W](adc10dtc0::W) writer structure"]
impl crate::Writable for ADC10DTC0 {}
#[doc = "ADC10 Data Transfer Control 0"]
pub mod adc10dtc0;
#[doc = "ADC10 Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ctl0](adc10ctl0) module"]
pub type ADC10CTL0 = crate::Reg<u16, _ADC10CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC10CTL0;
#[doc = "`read()` method returns [adc10ctl0::R](adc10ctl0::R) reader structure"]
impl crate::Readable for ADC10CTL0 {}
#[doc = "`write(|w| ..)` method takes [adc10ctl0::W](adc10ctl0::W) writer structure"]
impl crate::Writable for ADC10CTL0 {}
#[doc = "ADC10 Control 0"]
pub mod adc10ctl0;
#[doc = "ADC10 Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ctl1](adc10ctl1) module"]
pub type ADC10CTL1 = crate::Reg<u16, _ADC10CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC10CTL1;
#[doc = "`read()` method returns [adc10ctl1::R](adc10ctl1::R) reader structure"]
impl crate::Readable for ADC10CTL1 {}
#[doc = "`write(|w| ..)` method takes [adc10ctl1::W](adc10ctl1::W) writer structure"]
impl crate::Writable for ADC10CTL1 {}
#[doc = "ADC10 Control 1"]
pub mod adc10ctl1;
#[doc = "ADC10 Analog Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ae0](adc10ae0) module"]
pub type ADC10AE0 = crate::Reg<u8, _ADC10AE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC10AE0;
#[doc = "`read()` method returns [adc10ae0::R](adc10ae0::R) reader structure"]
impl crate::Readable for ADC10AE0 {}
#[doc = "`write(|w| ..)` method takes [adc10ae0::W](adc10ae0::W) writer structure"]
impl crate::Writable for ADC10AE0 {}
#[doc = "ADC10 Analog Enable 0"]
pub mod adc10ae0;
#[doc = "ADC10 Data Transfer Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10dtc1](adc10dtc1) module"]
pub type ADC10DTC1 = crate::Reg<u8, _ADC10DTC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC10DTC1;
#[doc = "`read()` method returns [adc10dtc1::R](adc10dtc1::R) reader structure"]
impl crate::Readable for ADC10DTC1 {}
#[doc = "`write(|w| ..)` method takes [adc10dtc1::W](adc10dtc1::W) writer structure"]
impl crate::Writable for ADC10DTC1 {}
#[doc = "ADC10 Data Transfer Control 1"]
pub mod adc10dtc1;
#[doc = "ADC10 Memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10mem](adc10mem) module"]
pub type ADC10MEM = crate::Reg<u16, _ADC10MEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC10MEM;
#[doc = "`read()` method returns [adc10mem::R](adc10mem::R) reader structure"]
impl crate::Readable for ADC10MEM {}
#[doc = "`write(|w| ..)` method takes [adc10mem::W](adc10mem::W) writer structure"]
impl crate::Writable for ADC10MEM {}
#[doc = "ADC10 Memory"]
pub mod adc10mem;
#[doc = "ADC10 Data Transfer Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10sa](adc10sa) module"]
pub type ADC10SA = crate::Reg<u16, _ADC10SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC10SA;
#[doc = "`read()` method returns [adc10sa::R](adc10sa::R) reader structure"]
impl crate::Readable for ADC10SA {}
#[doc = "`write(|w| ..)` method takes [adc10sa::W](adc10sa::W) writer structure"]
impl crate::Writable for ADC10SA {}
#[doc = "ADC10 Data Transfer Start Address"]
pub mod adc10sa;
