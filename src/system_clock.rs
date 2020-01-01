#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Basic Clock System Control 3"]
    pub bcsctl3: BCSCTL3,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - DCO Clock Frequency Control"]
    pub dcoctl: DCOCTL,
    #[doc = "0x05 - Basic Clock System Control 1"]
    pub bcsctl1: BCSCTL1,
    #[doc = "0x06 - Basic Clock System Control 2"]
    pub bcsctl2: BCSCTL2,
}
#[doc = "Basic Clock System Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcsctl3](bcsctl3) module"]
pub type BCSCTL3 = crate::Reg<u8, _BCSCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCSCTL3;
#[doc = "`read()` method returns [bcsctl3::R](bcsctl3::R) reader structure"]
impl crate::Readable for BCSCTL3 {}
#[doc = "`write(|w| ..)` method takes [bcsctl3::W](bcsctl3::W) writer structure"]
impl crate::Writable for BCSCTL3 {}
#[doc = "Basic Clock System Control 3"]
pub mod bcsctl3;
#[doc = "DCO Clock Frequency Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoctl](dcoctl) module"]
pub type DCOCTL = crate::Reg<u8, _DCOCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOCTL;
#[doc = "`read()` method returns [dcoctl::R](dcoctl::R) reader structure"]
impl crate::Readable for DCOCTL {}
#[doc = "`write(|w| ..)` method takes [dcoctl::W](dcoctl::W) writer structure"]
impl crate::Writable for DCOCTL {}
#[doc = "DCO Clock Frequency Control"]
pub mod dcoctl;
#[doc = "Basic Clock System Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcsctl1](bcsctl1) module"]
pub type BCSCTL1 = crate::Reg<u8, _BCSCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCSCTL1;
#[doc = "`read()` method returns [bcsctl1::R](bcsctl1::R) reader structure"]
impl crate::Readable for BCSCTL1 {}
#[doc = "`write(|w| ..)` method takes [bcsctl1::W](bcsctl1::W) writer structure"]
impl crate::Writable for BCSCTL1 {}
#[doc = "Basic Clock System Control 1"]
pub mod bcsctl1;
#[doc = "Basic Clock System Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcsctl2](bcsctl2) module"]
pub type BCSCTL2 = crate::Reg<u8, _BCSCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCSCTL2;
#[doc = "`read()` method returns [bcsctl2::R](bcsctl2::R) reader structure"]
impl crate::Readable for BCSCTL2 {}
#[doc = "`write(|w| ..)` method takes [bcsctl2::W](bcsctl2::W) writer structure"]
impl crate::Writable for BCSCTL2 {}
#[doc = "Basic Clock System Control 2"]
pub mod bcsctl2;
