#[doc = "Register `CALBC1_8MHZ` reader"]
pub struct R(crate::R<CALBC1_8MHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALBC1_8MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALBC1_8MHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALBC1_8MHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALBC1_8MHZ` writer"]
pub struct W(crate::W<CALBC1_8MHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALBC1_8MHZ_SPEC>;
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
impl From<crate::W<CALBC1_8MHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALBC1_8MHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALBC1_8MHZ` reader - BCSCTL1 Calibration Data for 8MHz register"]
pub type CALBC1_8MHZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALBC1_8MHZ` writer - BCSCTL1 Calibration Data for 8MHz register"]
pub type CALBC1_8MHZ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CALBC1_8MHZ_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 8MHz register"]
    #[inline(always)]
    pub fn calbc1_8mhz(&self) -> CALBC1_8MHZ_R {
        CALBC1_8MHZ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 8MHz register"]
    #[inline(always)]
    pub fn calbc1_8mhz(&mut self) -> CALBC1_8MHZ_W<0> {
        CALBC1_8MHZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "BCSCTL1 Calibration Data for 8MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calbc1_8mhz](index.html) module"]
pub struct CALBC1_8MHZ_SPEC;
impl crate::RegisterSpec for CALBC1_8MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [calbc1_8mhz::R](R) reader structure"]
impl crate::Readable for CALBC1_8MHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calbc1_8mhz::W](W) writer structure"]
impl crate::Writable for CALBC1_8MHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALBC1_8MHZ to value 0"]
impl crate::Resettable for CALBC1_8MHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
