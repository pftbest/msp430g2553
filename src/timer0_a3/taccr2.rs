#[doc = "Register `TACCR2` reader"]
pub struct R(crate::R<TACCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TACCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TACCR2_SPEC>> for R {
    fn from(reader: crate::R<TACCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TACCR2` writer"]
pub struct W(crate::W<TACCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TACCR2_SPEC>;
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
impl core::convert::From<crate::W<TACCR2_SPEC>> for W {
    fn from(writer: crate::W<TACCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TACCR2` reader - Timer A Capture/Compare register 2"]
pub struct TACCR2_R(crate::FieldReader<u16, u16>);
impl TACCR2_R {
    pub(crate) fn new(bits: u16) -> Self {
        TACCR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACCR2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACCR2` writer - Timer A Capture/Compare register 2"]
pub struct TACCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TACCR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&self) -> TACCR2_R {
        TACCR2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&mut self) -> TACCR2_W {
        TACCR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0_A3 Capture/Compare 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taccr2](index.html) module"]
pub struct TACCR2_SPEC;
impl crate::RegisterSpec for TACCR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [taccr2::R](R) reader structure"]
impl crate::Readable for TACCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taccr2::W](W) writer structure"]
impl crate::Writable for TACCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TACCR2 to value 0"]
impl crate::Resettable for TACCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
