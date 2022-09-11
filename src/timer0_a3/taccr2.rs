#[doc = "Register `TACCR2` reader"]
pub struct R(crate::R<TACCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TACCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TACCR2_SPEC>> for R {
    #[inline(always)]
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
impl From<crate::W<TACCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TACCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TACCR2` reader - Timer A Capture/Compare register 2"]
pub type TACCR2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TACCR2` writer - Timer A Capture/Compare register 2"]
pub type TACCR2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TACCR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&self) -> TACCR2_R {
        TACCR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&mut self) -> TACCR2_W<0> {
        TACCR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
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
