#[doc = "Register `TACCR1` reader"]
pub struct R(crate::R<TACCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TACCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TACCR1_SPEC>> for R {
    fn from(reader: crate::R<TACCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TACCR1` writer"]
pub struct W(crate::W<TACCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TACCR1_SPEC>;
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
impl core::convert::From<crate::W<TACCR1_SPEC>> for W {
    fn from(writer: crate::W<TACCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TACCR1` reader - Timer A Capture/Compare register 1"]
pub struct TACCR1_R(crate::FieldReader<u16, u16>);
impl TACCR1_R {
    pub(crate) fn new(bits: u16) -> Self {
        TACCR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACCR1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACCR1` writer - Timer A Capture/Compare register 1"]
pub struct TACCR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TACCR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 1"]
    #[inline(always)]
    pub fn taccr1(&self) -> TACCR1_R {
        TACCR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 1"]
    #[inline(always)]
    pub fn taccr1(&mut self) -> TACCR1_W {
        TACCR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0_A3 Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taccr1](index.html) module"]
pub struct TACCR1_SPEC;
impl crate::RegisterSpec for TACCR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [taccr1::R](R) reader structure"]
impl crate::Readable for TACCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taccr1::W](W) writer structure"]
impl crate::Writable for TACCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TACCR1 to value 0"]
impl crate::Resettable for TACCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
