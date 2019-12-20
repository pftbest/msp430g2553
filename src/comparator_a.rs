#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Comparator A Control 1"]
    pub cactl1: CACTL1,
    #[doc = "0x02 - Comparator A Control 2"]
    pub cactl2: CACTL2,
    #[doc = "0x03 - Comparator A Port Disable"]
    pub capd: CAPD,
}
#[doc = "Comparator A Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cactl1](cactl1) module"]
pub type CACTL1 = crate::Reg<u8, _CACTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACTL1;
#[doc = "`read()` method returns [cactl1::R](cactl1::R) reader structure"]
impl crate::Readable for CACTL1 {}
#[doc = "`write(|w| ..)` method takes [cactl1::W](cactl1::W) writer structure"]
impl crate::Writable for CACTL1 {}
#[doc = "Comparator A Control 1"]
pub mod cactl1;
#[doc = "Comparator A Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cactl2](cactl2) module"]
pub type CACTL2 = crate::Reg<u8, _CACTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACTL2;
#[doc = "`read()` method returns [cactl2::R](cactl2::R) reader structure"]
impl crate::Readable for CACTL2 {}
#[doc = "`write(|w| ..)` method takes [cactl2::W](cactl2::W) writer structure"]
impl crate::Writable for CACTL2 {}
#[doc = "Comparator A Control 2"]
pub mod cactl2;
#[doc = "Comparator A Port Disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [capd](capd) module"]
pub type CAPD = crate::Reg<u8, _CAPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPD;
#[doc = "`read()` method returns [capd::R](capd::R) reader structure"]
impl crate::Readable for CAPD {}
#[doc = "`write(|w| ..)` method takes [capd::W](capd::W) writer structure"]
impl crate::Writable for CAPD {}
#[doc = "Comparator A Port Disable"]
pub mod capd;
