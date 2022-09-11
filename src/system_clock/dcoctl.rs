#[doc = "Register `DCOCTL` reader"]
pub struct R(crate::R<DCOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCOCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCOCTL` writer"]
pub struct W(crate::W<DCOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCOCTL_SPEC>;
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
impl From<crate::W<DCOCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOCTL` reader - DCO Clock Frequency Control register"]
pub type DCOCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCOCTL` writer - DCO Clock Frequency Control register"]
pub type DCOCTL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DCOCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `MOD` reader - Modulation Bit 0"]
pub type MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOD` writer - Modulation Bit 0"]
pub type MOD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DCOCTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DCO` reader - DCO Select Bit 0"]
pub type DCO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCO` writer - DCO Select Bit 0"]
pub type DCO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DCOCTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - DCO Clock Frequency Control register"]
    #[inline(always)]
    pub fn dcoctl(&self) -> DCOCTL_R {
        DCOCTL_R::new(self.bits)
    }
    #[doc = "Bits 0:4 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco(&self) -> DCO_R {
        DCO_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCO Clock Frequency Control register"]
    #[inline(always)]
    pub fn dcoctl(&mut self) -> DCOCTL_W<0> {
        DCOCTL_W::new(self)
    }
    #[doc = "Bits 0:4 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W<0> {
        MOD_W::new(self)
    }
    #[doc = "Bits 5:7 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco(&mut self) -> DCO_W<5> {
        DCO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "DCO Clock Frequency Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoctl](index.html) module"]
pub struct DCOCTL_SPEC;
impl crate::RegisterSpec for DCOCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dcoctl::R](R) reader structure"]
impl crate::Readable for DCOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcoctl::W](W) writer structure"]
impl crate::Writable for DCOCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCOCTL to value 0"]
impl crate::Resettable for DCOCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
