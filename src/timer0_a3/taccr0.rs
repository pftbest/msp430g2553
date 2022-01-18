#[doc = "Register `TACCR0` reader"]
pub struct R(crate::R<TACCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TACCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TACCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TACCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TACCR0` writer"]
pub struct W(crate::W<TACCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TACCR0_SPEC>;
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
impl From<crate::W<TACCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TACCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TACCR0` reader - Timer A Capture/Compare register 0"]
pub struct TACCR0_R(crate::FieldReader<u16, u16>);
impl TACCR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TACCR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACCR0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACCR0` writer - Timer A Capture/Compare register 0"]
pub struct TACCR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TACCR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value as u16;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 0"]
    #[inline(always)]
    pub fn taccr0(&self) -> TACCR0_R {
        TACCR0_R::new(self.bits as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 0"]
    #[inline(always)]
    pub fn taccr0(&mut self) -> TACCR0_W {
        TACCR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer0_A3 Capture/Compare 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taccr0](index.html) module"]
pub struct TACCR0_SPEC;
impl crate::RegisterSpec for TACCR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [taccr0::R](R) reader structure"]
impl crate::Readable for TACCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taccr0::W](W) writer structure"]
impl crate::Writable for TACCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TACCR0 to value 0"]
impl crate::Resettable for TACCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
