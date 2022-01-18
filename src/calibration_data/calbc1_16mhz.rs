#[doc = "Register `CALBC1_16MHZ` reader"]
pub struct R(crate::R<CALBC1_16MHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALBC1_16MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALBC1_16MHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALBC1_16MHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALBC1_16MHZ` writer"]
pub struct W(crate::W<CALBC1_16MHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALBC1_16MHZ_SPEC>;
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
impl From<crate::W<CALBC1_16MHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALBC1_16MHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALBC1_16MHZ` reader - BCSCTL1 Calibration Data for 16MHz register"]
pub struct CALBC1_16MHZ_R(crate::FieldReader<u8, u8>);
impl CALBC1_16MHZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CALBC1_16MHZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALBC1_16MHZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALBC1_16MHZ` writer - BCSCTL1 Calibration Data for 16MHz register"]
pub struct CALBC1_16MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CALBC1_16MHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn calbc1_16mhz(&self) -> CALBC1_16MHZ_R {
        CALBC1_16MHZ_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn calbc1_16mhz(&mut self) -> CALBC1_16MHZ_W {
        CALBC1_16MHZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "BCSCTL1 Calibration Data for 16MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calbc1_16mhz](index.html) module"]
pub struct CALBC1_16MHZ_SPEC;
impl crate::RegisterSpec for CALBC1_16MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [calbc1_16mhz::R](R) reader structure"]
impl crate::Readable for CALBC1_16MHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calbc1_16mhz::W](W) writer structure"]
impl crate::Writable for CALBC1_16MHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALBC1_16MHZ to value 0"]
impl crate::Resettable for CALBC1_16MHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
