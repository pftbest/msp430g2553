#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH Control 1"]
    pub fctl1: FCTL1,
    #[doc = "0x02 - FLASH Control 2"]
    pub fctl2: FCTL2,
    #[doc = "0x04 - FLASH Control 3"]
    pub fctl3: FCTL3,
}
#[doc = "FLASH Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fctl1](fctl1) module"]
pub type FCTL1 = crate::Reg<u16, _FCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTL1;
#[doc = "`read()` method returns [fctl1::R](fctl1::R) reader structure"]
impl crate::Readable for FCTL1 {}
#[doc = "`write(|w| ..)` method takes [fctl1::W](fctl1::W) writer structure"]
impl crate::Writable for FCTL1 {}
#[doc = "FLASH Control 1"]
pub mod fctl1;
#[doc = "FLASH Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fctl2](fctl2) module"]
pub type FCTL2 = crate::Reg<u16, _FCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTL2;
#[doc = "`read()` method returns [fctl2::R](fctl2::R) reader structure"]
impl crate::Readable for FCTL2 {}
#[doc = "`write(|w| ..)` method takes [fctl2::W](fctl2::W) writer structure"]
impl crate::Writable for FCTL2 {}
#[doc = "FLASH Control 2"]
pub mod fctl2;
#[doc = "FLASH Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fctl3](fctl3) module"]
pub type FCTL3 = crate::Reg<u16, _FCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTL3;
#[doc = "`read()` method returns [fctl3::R](fctl3::R) reader structure"]
impl crate::Readable for FCTL3 {}
#[doc = "`write(|w| ..)` method takes [fctl3::W](fctl3::W) writer structure"]
impl crate::Writable for FCTL3 {}
#[doc = "FLASH Control 3"]
pub mod fctl3;
