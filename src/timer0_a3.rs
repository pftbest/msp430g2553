#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer A Interrupt Vector Word"]
    pub taiv: TAIV,
    _reserved1: [u8; 48usize],
    #[doc = "0x32 - Timer0_A3 Control"]
    pub tactl: TACTL,
    #[doc = "0x34 - Timer0_A3 Capture/Compare Control 0"]
    pub tacctl0: TACCTL0,
    #[doc = "0x36 - Timer0_A3 Capture/Compare Control 1"]
    pub tacctl1: TACCTL1,
    #[doc = "0x38 - Timer0_A3 Capture/Compare Control 2"]
    pub tacctl2: TACCTL2,
    _reserved5: [u8; 8usize],
    #[doc = "0x42 - Timer A Counter Register"]
    pub tar: TAR,
    #[doc = "0x44 - Timer A Capture/Compare 0"]
    pub taccr0: TACCR0,
    #[doc = "0x46 - Timer A Capture/Compare 1"]
    pub taccr1: TACCR1,
    #[doc = "0x48 - Timer A Capture/Compare 2"]
    pub taccr2: TACCR2,
}
#[doc = "Timer0_A3 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tactl](tactl) module"]
pub type TACTL = crate::Reg<u16, _TACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TACTL;
#[doc = "`read()` method returns [tactl::R](tactl::R) reader structure"]
impl crate::Readable for TACTL {}
#[doc = "`write(|w| ..)` method takes [tactl::W](tactl::W) writer structure"]
impl crate::Writable for TACTL {}
#[doc = "Timer0_A3 Control"]
pub mod tactl;
#[doc = "Timer0_A3 Capture/Compare Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tacctl0](tacctl0) module"]
pub type TACCTL0 = crate::Reg<u16, _TACCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TACCTL0;
#[doc = "`read()` method returns [tacctl0::R](tacctl0::R) reader structure"]
impl crate::Readable for TACCTL0 {}
#[doc = "`write(|w| ..)` method takes [tacctl0::W](tacctl0::W) writer structure"]
impl crate::Writable for TACCTL0 {}
#[doc = "Timer0_A3 Capture/Compare Control 0"]
pub mod tacctl0;
#[doc = "Timer0_A3 Capture/Compare Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tacctl1](tacctl1) module"]
pub type TACCTL1 = crate::Reg<u16, _TACCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TACCTL1;
#[doc = "`read()` method returns [tacctl1::R](tacctl1::R) reader structure"]
impl crate::Readable for TACCTL1 {}
#[doc = "`write(|w| ..)` method takes [tacctl1::W](tacctl1::W) writer structure"]
impl crate::Writable for TACCTL1 {}
#[doc = "Timer0_A3 Capture/Compare Control 1"]
pub mod tacctl1;
#[doc = "Timer0_A3 Capture/Compare Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tacctl2](tacctl2) module"]
pub type TACCTL2 = crate::Reg<u16, _TACCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TACCTL2;
#[doc = "`read()` method returns [tacctl2::R](tacctl2::R) reader structure"]
impl crate::Readable for TACCTL2 {}
#[doc = "`write(|w| ..)` method takes [tacctl2::W](tacctl2::W) writer structure"]
impl crate::Writable for TACCTL2 {}
#[doc = "Timer0_A3 Capture/Compare Control 2"]
pub mod tacctl2;
#[doc = "Timer A Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taiv](taiv) module"]
pub type TAIV = crate::Reg<u16, _TAIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAIV;
#[doc = "`read()` method returns [taiv::R](taiv::R) reader structure"]
impl crate::Readable for TAIV {}
#[doc = "`write(|w| ..)` method takes [taiv::W](taiv::W) writer structure"]
impl crate::Writable for TAIV {}
#[doc = "Timer A Interrupt Vector Word"]
pub mod taiv;
#[doc = "Timer A Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](tar) module"]
pub type TAR = crate::Reg<u16, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "`write(|w| ..)` method takes [tar::W](tar::W) writer structure"]
impl crate::Writable for TAR {}
#[doc = "Timer A Counter Register"]
pub mod tar;
#[doc = "Timer A Capture/Compare 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taccr0](taccr0) module"]
pub type TACCR0 = crate::Reg<u16, _TACCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TACCR0;
#[doc = "`read()` method returns [taccr0::R](taccr0::R) reader structure"]
impl crate::Readable for TACCR0 {}
#[doc = "`write(|w| ..)` method takes [taccr0::W](taccr0::W) writer structure"]
impl crate::Writable for TACCR0 {}
#[doc = "Timer A Capture/Compare 0"]
pub mod taccr0;
#[doc = "Timer A Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taccr1](taccr1) module"]
pub type TACCR1 = crate::Reg<u16, _TACCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TACCR1;
#[doc = "`read()` method returns [taccr1::R](taccr1::R) reader structure"]
impl crate::Readable for TACCR1 {}
#[doc = "`write(|w| ..)` method takes [taccr1::W](taccr1::W) writer structure"]
impl crate::Writable for TACCR1 {}
#[doc = "Timer A Capture/Compare 1"]
pub mod taccr1;
#[doc = "Timer A Capture/Compare 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taccr2](taccr2) module"]
pub type TACCR2 = crate::Reg<u16, _TACCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TACCR2;
#[doc = "`read()` method returns [taccr2::R](taccr2::R) reader structure"]
impl crate::Readable for TACCR2 {}
#[doc = "`write(|w| ..)` method takes [taccr2::W](taccr2::W) writer structure"]
impl crate::Writable for TACCR2 {}
#[doc = "Timer A Capture/Compare 2"]
pub mod taccr2;
