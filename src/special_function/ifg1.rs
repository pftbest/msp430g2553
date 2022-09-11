#[doc = "Register `IFG1` reader"]
pub struct R(crate::R<IFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFG1` writer"]
pub struct W(crate::W<IFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFG1_SPEC>;
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
impl From<crate::W<IFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIFG` reader - Watchdog Interrupt Flag"]
pub type WDTIFG_R = crate::BitReader<bool>;
#[doc = "Field `WDTIFG` writer - Watchdog Interrupt Flag"]
pub type WDTIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG1_SPEC, bool, O>;
#[doc = "Field `OFIFG` reader - Osc. Fault Interrupt Flag"]
pub type OFIFG_R = crate::BitReader<bool>;
#[doc = "Field `OFIFG` writer - Osc. Fault Interrupt Flag"]
pub type OFIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG1_SPEC, bool, O>;
#[doc = "Field `PORIFG` reader - Power On Interrupt Flag"]
pub type PORIFG_R = crate::BitReader<bool>;
#[doc = "Field `PORIFG` writer - Power On Interrupt Flag"]
pub type PORIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG1_SPEC, bool, O>;
#[doc = "Field `RSTIFG` reader - Reset Interrupt Flag"]
pub type RSTIFG_R = crate::BitReader<bool>;
#[doc = "Field `RSTIFG` writer - Reset Interrupt Flag"]
pub type RSTIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG1_SPEC, bool, O>;
#[doc = "Field `NMIIFG` reader - NMI Interrupt Flag"]
pub type NMIIFG_R = crate::BitReader<bool>;
#[doc = "Field `NMIIFG` writer - NMI Interrupt Flag"]
pub type NMIIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&self) -> PORIFG_R {
        PORIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&self) -> RSTIFG_R {
        RSTIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WDTIFG_W<0> {
        WDTIFG_W::new(self)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OFIFG_W<1> {
        OFIFG_W::new(self)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&mut self) -> PORIFG_W<2> {
        PORIFG_W::new(self)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&mut self) -> RSTIFG_W<3> {
        RSTIFG_W::new(self)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NMIIFG_W<4> {
        NMIIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifg1](index.html) module"]
pub struct IFG1_SPEC;
impl crate::RegisterSpec for IFG1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ifg1::R](R) reader structure"]
impl crate::Readable for IFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifg1::W](W) writer structure"]
impl crate::Writable for IFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFG1 to value 0"]
impl crate::Resettable for IFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
