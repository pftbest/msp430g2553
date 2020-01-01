#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer1_A3 Interrupt Vector Word"]
    pub ta1iv: TA1IV,
    _reserved1: [u8; 96usize],
    #[doc = "0x62 - Timer1_A3 Control"]
    pub ta1ctl: TA1CTL,
    #[doc = "0x64 - Timer1_A3 Capture/Compare Control 0"]
    pub ta1cctl0: TA1CCTL0,
    #[doc = "0x66 - Timer1_A3 Capture/Compare Control 1"]
    pub ta1cctl1: TA1CCTL1,
    #[doc = "0x68 - Timer1_A3 Capture/Compare Control 2"]
    pub ta1cctl2: TA1CCTL2,
    _reserved5: [u8; 8usize],
    #[doc = "0x72 - Timer1_A3 Counter Register"]
    pub ta1r: TA1R,
    #[doc = "0x74 - Timer1_A3 Capture/Compare 0"]
    pub ta1ccr0: TA1CCR0,
    #[doc = "0x76 - Timer1_A3 Capture/Compare 1"]
    pub ta1ccr1: TA1CCR1,
    #[doc = "0x78 - Timer1_A3 Capture/Compare 2"]
    pub ta1ccr2: TA1CCR2,
}
#[doc = "Timer1_A3 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1iv](ta1iv) module"]
pub type TA1IV = crate::Reg<u16, _TA1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1IV;
#[doc = "`read()` method returns [ta1iv::R](ta1iv::R) reader structure"]
impl crate::Readable for TA1IV {}
#[doc = "`write(|w| ..)` method takes [ta1iv::W](ta1iv::W) writer structure"]
impl crate::Writable for TA1IV {}
#[doc = "Timer1_A3 Interrupt Vector Word"]
pub mod ta1iv;
#[doc = "Timer1_A3 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ctl](ta1ctl) module"]
pub type TA1CTL = crate::Reg<u16, _TA1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CTL;
#[doc = "`read()` method returns [ta1ctl::R](ta1ctl::R) reader structure"]
impl crate::Readable for TA1CTL {}
#[doc = "`write(|w| ..)` method takes [ta1ctl::W](ta1ctl::W) writer structure"]
impl crate::Writable for TA1CTL {}
#[doc = "Timer1_A3 Control"]
pub mod ta1ctl;
#[doc = "Timer1_A3 Capture/Compare Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1cctl0](ta1cctl0) module"]
pub type TA1CCTL0 = crate::Reg<u16, _TA1CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCTL0;
#[doc = "`read()` method returns [ta1cctl0::R](ta1cctl0::R) reader structure"]
impl crate::Readable for TA1CCTL0 {}
#[doc = "`write(|w| ..)` method takes [ta1cctl0::W](ta1cctl0::W) writer structure"]
impl crate::Writable for TA1CCTL0 {}
#[doc = "Timer1_A3 Capture/Compare Control 0"]
pub mod ta1cctl0;
#[doc = "Timer1_A3 Capture/Compare Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1cctl1](ta1cctl1) module"]
pub type TA1CCTL1 = crate::Reg<u16, _TA1CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCTL1;
#[doc = "`read()` method returns [ta1cctl1::R](ta1cctl1::R) reader structure"]
impl crate::Readable for TA1CCTL1 {}
#[doc = "`write(|w| ..)` method takes [ta1cctl1::W](ta1cctl1::W) writer structure"]
impl crate::Writable for TA1CCTL1 {}
#[doc = "Timer1_A3 Capture/Compare Control 1"]
pub mod ta1cctl1;
#[doc = "Timer1_A3 Capture/Compare Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1cctl2](ta1cctl2) module"]
pub type TA1CCTL2 = crate::Reg<u16, _TA1CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCTL2;
#[doc = "`read()` method returns [ta1cctl2::R](ta1cctl2::R) reader structure"]
impl crate::Readable for TA1CCTL2 {}
#[doc = "`write(|w| ..)` method takes [ta1cctl2::W](ta1cctl2::W) writer structure"]
impl crate::Writable for TA1CCTL2 {}
#[doc = "Timer1_A3 Capture/Compare Control 2"]
pub mod ta1cctl2;
#[doc = "Timer1_A3 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1r](ta1r) module"]
pub type TA1R = crate::Reg<u16, _TA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1R;
#[doc = "`read()` method returns [ta1r::R](ta1r::R) reader structure"]
impl crate::Readable for TA1R {}
#[doc = "`write(|w| ..)` method takes [ta1r::W](ta1r::W) writer structure"]
impl crate::Writable for TA1R {}
#[doc = "Timer1_A3 Counter Register"]
pub mod ta1r;
#[doc = "Timer1_A3 Capture/Compare 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ccr0](ta1ccr0) module"]
pub type TA1CCR0 = crate::Reg<u16, _TA1CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCR0;
#[doc = "`read()` method returns [ta1ccr0::R](ta1ccr0::R) reader structure"]
impl crate::Readable for TA1CCR0 {}
#[doc = "`write(|w| ..)` method takes [ta1ccr0::W](ta1ccr0::W) writer structure"]
impl crate::Writable for TA1CCR0 {}
#[doc = "Timer1_A3 Capture/Compare 0"]
pub mod ta1ccr0;
#[doc = "Timer1_A3 Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ccr1](ta1ccr1) module"]
pub type TA1CCR1 = crate::Reg<u16, _TA1CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCR1;
#[doc = "`read()` method returns [ta1ccr1::R](ta1ccr1::R) reader structure"]
impl crate::Readable for TA1CCR1 {}
#[doc = "`write(|w| ..)` method takes [ta1ccr1::W](ta1ccr1::W) writer structure"]
impl crate::Writable for TA1CCR1 {}
#[doc = "Timer1_A3 Capture/Compare 1"]
pub mod ta1ccr1;
#[doc = "Timer1_A3 Capture/Compare 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ccr2](ta1ccr2) module"]
pub type TA1CCR2 = crate::Reg<u16, _TA1CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCR2;
#[doc = "`read()` method returns [ta1ccr2::R](ta1ccr2::R) reader structure"]
impl crate::Readable for TA1CCR2 {}
#[doc = "`write(|w| ..)` method takes [ta1ccr2::W](ta1ccr2::W) writer structure"]
impl crate::Writable for TA1CCR2 {}
#[doc = "Timer1_A3 Capture/Compare 2"]
pub mod ta1ccr2;
