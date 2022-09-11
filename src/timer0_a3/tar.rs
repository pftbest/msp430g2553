#[doc = "Register `TAR` reader"]
pub struct R(crate::R<TAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAR` writer"]
pub struct W(crate::W<TAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAR_SPEC>;
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
impl From<crate::W<TAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAR` reader - Timer A Counter Register"]
pub type TAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TAR` writer - Timer A Counter Register"]
pub type TAR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TAR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer A Counter Register"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Counter Register"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<0> {
        TAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer0_A3 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](index.html) module"]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tar::R](R) reader structure"]
impl crate::Readable for TAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tar::W](W) writer structure"]
impl crate::Writable for TAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAR to value 0"]
impl crate::Resettable for TAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
