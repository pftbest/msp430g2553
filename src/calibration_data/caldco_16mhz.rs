#[doc = "Register `CALDCO_16MHZ` reader"]
pub struct R(crate::R<CALDCO_16MHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALDCO_16MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALDCO_16MHZ_SPEC>> for R {
    #[inline(always)]
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
impl From<crate::W<CALDCO_16MHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALDCO_16MHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALDCO_16MHZ` reader - DCOCTL Calibration Data for 16MHz register"]
pub type CALDCO_16MHZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALDCO_16MHZ` writer - DCOCTL Calibration Data for 16MHz register"]
pub type CALDCO_16MHZ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CALDCO_16MHZ_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn caldco_16mhz(&self) -> CALDCO_16MHZ_R {
        CALDCO_16MHZ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn caldco_16mhz(&mut self) -> CALDCO_16MHZ_W<0> {
        CALDCO_16MHZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
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
