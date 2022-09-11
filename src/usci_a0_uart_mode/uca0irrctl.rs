#[doc = "Register `UCA0IRRCTL` reader"]
pub struct R(crate::R<UCA0IRRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IRRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IRRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IRRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IRRCTL` writer"]
pub struct W(crate::W<UCA0IRRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IRRCTL_SPEC>;
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
impl From<crate::W<UCA0IRRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IRRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIRRXFE` reader - IRDA Receive Filter enable"]
pub type UCIRRXFE_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFE` writer - IRDA Receive Filter enable"]
pub type UCIRRXFE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXPL` reader - IRDA Receive Input Polarity"]
pub type UCIRRXPL_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXPL` writer - IRDA Receive Input Polarity"]
pub type UCIRRXPL_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXFL` reader - IRDA Receive Filter Length 0"]
pub type UCIRRXFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCIRRXFL` writer - IRDA Receive Filter Length 0"]
pub type UCIRRXFL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, UCA0IRRCTL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl(&self) -> UCIRRXFL_R {
        UCIRRXFL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W<0> {
        UCIRRXFE_W::new(self)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W<1> {
        UCIRRXPL_W::new(self)
    }
    #[doc = "Bits 2:7 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl(&mut self) -> UCIRRXFL_W<2> {
        UCIRRXFL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 IrDA Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0irrctl](index.html) module"]
pub struct UCA0IRRCTL_SPEC;
impl crate::RegisterSpec for UCA0IRRCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0irrctl::R](R) reader structure"]
impl crate::Readable for UCA0IRRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0irrctl::W](W) writer structure"]
impl crate::Writable for UCA0IRRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IRRCTL to value 0"]
impl crate::Resettable for UCA0IRRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
