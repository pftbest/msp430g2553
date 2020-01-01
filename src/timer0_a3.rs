#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0_A3 Interrupt Vector Word"]
    pub ta0iv: TA0IV,
    _reserved1: [u8; 48usize],
    #[doc = "0x32 - Timer0_A3 Control"]
    pub ta0ctl: TA0CTL,
    #[doc = "0x34 - Timer0_A3 Capture/Compare Control 0"]
    pub ta0cctl0: TA0CCTL0,
    #[doc = "0x36 - Timer0_A3 Capture/Compare Control 1"]
    pub ta0cctl1: TA0CCTL1,
    #[doc = "0x38 - Timer0_A3 Capture/Compare Control 2"]
    pub ta0cctl2: TA0CCTL2,
    _reserved5: [u8; 8usize],
    #[doc = "0x42 - Timer0_A3 Counter Register"]
    pub ta0r: TA0R,
    #[doc = "0x44 - Timer0_A3 Capture/Compare 0"]
    pub ta0ccr0: TA0CCR0,
    #[doc = "0x46 - Timer0_A3 Capture/Compare 1"]
    pub ta0ccr1: TA0CCR1,
    #[doc = "0x48 - Timer0_A3 Capture/Compare 2"]
    pub ta0ccr2: TA0CCR2,
}
#[doc = "Timer0_A3 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0iv](ta0iv) module"]
pub type TA0IV = crate::Reg<u16, _TA0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0IV;
#[doc = "`read()` method returns [ta0iv::R](ta0iv::R) reader structure"]
impl crate::Readable for TA0IV {}
#[doc = "`write(|w| ..)` method takes [ta0iv::W](ta0iv::W) writer structure"]
impl crate::Writable for TA0IV {}
#[doc = "Timer0_A3 Interrupt Vector Word"]
pub mod ta0iv;
#[doc = "Timer0_A3 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ctl](ta0ctl) module"]
pub type TA0CTL = crate::Reg<u16, _TA0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CTL;
#[doc = "`read()` method returns [ta0ctl::R](ta0ctl::R) reader structure"]
impl crate::Readable for TA0CTL {}
#[doc = "`write(|w| ..)` method takes [ta0ctl::W](ta0ctl::W) writer structure"]
impl crate::Writable for TA0CTL {}
#[doc = "Timer0_A3 Control"]
pub mod ta0ctl;
#[doc = "Timer0_A3 Capture/Compare Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0cctl0](ta0cctl0) module"]
pub type TA0CCTL0 = crate::Reg<u16, _TA0CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCTL0;
#[doc = "`read()` method returns [ta0cctl0::R](ta0cctl0::R) reader structure"]
impl crate::Readable for TA0CCTL0 {}
#[doc = "`write(|w| ..)` method takes [ta0cctl0::W](ta0cctl0::W) writer structure"]
impl crate::Writable for TA0CCTL0 {}
#[doc = "Timer0_A3 Capture/Compare Control 0"]
pub mod ta0cctl0;
#[doc = "Timer0_A3 Capture/Compare Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0cctl1](ta0cctl1) module"]
pub type TA0CCTL1 = crate::Reg<u16, _TA0CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCTL1;
#[doc = "`read()` method returns [ta0cctl1::R](ta0cctl1::R) reader structure"]
impl crate::Readable for TA0CCTL1 {}
#[doc = "`write(|w| ..)` method takes [ta0cctl1::W](ta0cctl1::W) writer structure"]
impl crate::Writable for TA0CCTL1 {}
#[doc = "Timer0_A3 Capture/Compare Control 1"]
pub mod ta0cctl1;
#[doc = "Timer0_A3 Capture/Compare Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0cctl2](ta0cctl2) module"]
pub type TA0CCTL2 = crate::Reg<u16, _TA0CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCTL2;
#[doc = "`read()` method returns [ta0cctl2::R](ta0cctl2::R) reader structure"]
impl crate::Readable for TA0CCTL2 {}
#[doc = "`write(|w| ..)` method takes [ta0cctl2::W](ta0cctl2::W) writer structure"]
impl crate::Writable for TA0CCTL2 {}
#[doc = "Timer0_A3 Capture/Compare Control 2"]
pub mod ta0cctl2;
#[doc = "Timer0_A3 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0r](ta0r) module"]
pub type TA0R = crate::Reg<u16, _TA0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0R;
#[doc = "`read()` method returns [ta0r::R](ta0r::R) reader structure"]
impl crate::Readable for TA0R {}
#[doc = "`write(|w| ..)` method takes [ta0r::W](ta0r::W) writer structure"]
impl crate::Writable for TA0R {}
#[doc = "Timer0_A3 Counter Register"]
pub mod ta0r;
#[doc = "Timer0_A3 Capture/Compare 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr0](ta0ccr0) module"]
pub type TA0CCR0 = crate::Reg<u16, _TA0CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCR0;
#[doc = "`read()` method returns [ta0ccr0::R](ta0ccr0::R) reader structure"]
impl crate::Readable for TA0CCR0 {}
#[doc = "`write(|w| ..)` method takes [ta0ccr0::W](ta0ccr0::W) writer structure"]
impl crate::Writable for TA0CCR0 {}
#[doc = "Timer0_A3 Capture/Compare 0"]
pub mod ta0ccr0;
#[doc = "Timer0_A3 Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr1](ta0ccr1) module"]
pub type TA0CCR1 = crate::Reg<u16, _TA0CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCR1;
#[doc = "`read()` method returns [ta0ccr1::R](ta0ccr1::R) reader structure"]
impl crate::Readable for TA0CCR1 {}
#[doc = "`write(|w| ..)` method takes [ta0ccr1::W](ta0ccr1::W) writer structure"]
impl crate::Writable for TA0CCR1 {}
#[doc = "Timer0_A3 Capture/Compare 1"]
pub mod ta0ccr1;
#[doc = "Timer0_A3 Capture/Compare 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr2](ta0ccr2) module"]
pub type TA0CCR2 = crate::Reg<u16, _TA0CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCR2;
#[doc = "`read()` method returns [ta0ccr2::R](ta0ccr2::R) reader structure"]
impl crate::Readable for TA0CCR2 {}
#[doc = "`write(|w| ..)` method takes [ta0ccr2::W](ta0ccr2::W) writer structure"]
impl crate::Writable for TA0CCR2 {}
#[doc = "Timer0_A3 Capture/Compare 2"]
pub mod ta0ccr2;
