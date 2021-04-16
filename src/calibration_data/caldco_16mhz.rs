#[doc = "Register `CALDCO_16MHZ` reader"]
pub struct R(crate::R<CALDCO_16MHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALDCO_16MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CALDCO_16MHZ_SPEC>> for R {
    fn from(reader: crate::R<CALDCO_16MHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALDCO_16MHZ` writer"]
pub struct W(crate::W<CALDCO_16MHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALDCO_16MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CALDCO_16MHZ_SPEC>> for W {
    fn from(writer: crate::W<CALDCO_16MHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALDCO_16MHZ` reader - DCOCTL Calibration Data for 16MHz register"]
pub struct CALDCO_16MHZ_R(crate::FieldReader<u8, u8>);
impl CALDCO_16MHZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALDCO_16MHZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALDCO_16MHZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALDCO_16MHZ` writer - DCOCTL Calibration Data for 16MHz register"]
pub struct CALDCO_16MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDCO_16MHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn caldco_16mhz(&self) -> CALDCO_16MHZ_R {
        CALDCO_16MHZ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn caldco_16mhz(&mut self) -> CALDCO_16MHZ_W {
        CALDCO_16MHZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCOCTL Calibration Data for 16MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caldco_16mhz](index.html) module"]
pub struct CALDCO_16MHZ_SPEC;
impl crate::RegisterSpec for CALDCO_16MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [caldco_16mhz::R](R) reader structure"]
impl crate::Readable for CALDCO_16MHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [caldco_16mhz::W](W) writer structure"]
impl crate::Writable for CALDCO_16MHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALDCO_16MHZ to value 0"]
impl crate::Resettable for CALDCO_16MHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
