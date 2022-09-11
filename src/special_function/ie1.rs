#[doc = "Register `IE1` reader"]
pub struct R(crate::R<IE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE1` writer"]
pub struct W(crate::W<IE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE1_SPEC>;
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
impl From<crate::W<IE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIE` reader - Watchdog Interrupt Enable"]
pub type WDTIE_R = crate::BitReader<bool>;
#[doc = "Field `WDTIE` writer - Watchdog Interrupt Enable"]
pub type WDTIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE1_SPEC, bool, O>;
#[doc = "Field `OFIE` reader - Osc. Fault Interrupt Enable"]
pub type OFIE_R = crate::BitReader<bool>;
#[doc = "Field `OFIE` writer - Osc. Fault Interrupt Enable"]
pub type OFIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE1_SPEC, bool, O>;
#[doc = "Field `NMIIE` reader - NMI Interrupt Enable"]
pub type NMIIE_R = crate::BitReader<bool>;
#[doc = "Field `NMIIE` writer - NMI Interrupt Enable"]
pub type NMIIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE1_SPEC, bool, O>;
#[doc = "Field `ACCVIE` reader - Flash Access Violation Interrupt Enable"]
pub type ACCVIE_R = crate::BitReader<bool>;
#[doc = "Field `ACCVIE` writer - Flash Access Violation Interrupt Enable"]
pub type ACCVIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IE1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&self) -> ACCVIE_R {
        ACCVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WDTIE_W<0> {
        WDTIE_W::new(self)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OFIE_W<1> {
        OFIE_W::new(self)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NMIIE_W<4> {
        NMIIE_W::new(self)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&mut self) -> ACCVIE_W<5> {
        ACCVIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie1](index.html) module"]
pub struct IE1_SPEC;
impl crate::RegisterSpec for IE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ie1::R](R) reader structure"]
impl crate::Readable for IE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie1::W](W) writer structure"]
impl crate::Writable for IE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE1 to value 0"]
impl crate::Resettable for IE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
