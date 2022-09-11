#[doc = "Register `UCA0ABCTL` reader"]
pub struct R(crate::R<UCA0ABCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0ABCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0ABCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0ABCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0ABCTL` writer"]
pub struct W(crate::W<UCA0ABCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0ABCTL_SPEC>;
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
impl From<crate::W<UCA0ABCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0ABCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCABDEN` reader - Auto Baud Rate detect enable"]
pub type UCABDEN_R = crate::BitReader<bool>;
#[doc = "Field `UCABDEN` writer - Auto Baud Rate detect enable"]
pub type UCABDEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0ABCTL_SPEC, bool, O>;
#[doc = "Field `UCBTOE` reader - Break Timeout error"]
pub type UCBTOE_R = crate::BitReader<bool>;
#[doc = "Field `UCBTOE` writer - Break Timeout error"]
pub type UCBTOE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0ABCTL_SPEC, bool, O>;
#[doc = "Field `UCSTOE` reader - Sync-Field Timeout error"]
pub type UCSTOE_R = crate::BitReader<bool>;
#[doc = "Field `UCSTOE` writer - Sync-Field Timeout error"]
pub type UCSTOE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0ABCTL_SPEC, bool, O>;
#[doc = "Field `UCDELIM` reader - Break Sync Delimiter 0"]
pub type UCDELIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCDELIM` writer - Break Sync Delimiter 0"]
pub type UCDELIM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, UCA0ABCTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UCABDEN_R {
        UCABDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UCBTOE_R {
        UCBTOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UCSTOE_R {
        UCSTOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim(&self) -> UCDELIM_R {
        UCDELIM_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&mut self) -> UCABDEN_W<0> {
        UCABDEN_W::new(self)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&mut self) -> UCBTOE_W<2> {
        UCBTOE_W::new(self)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&mut self) -> UCSTOE_W<3> {
        UCSTOE_W::new(self)
    }
    #[doc = "Bits 4:5 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim(&mut self) -> UCDELIM_W<4> {
        UCDELIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 LIN Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0abctl](index.html) module"]
pub struct UCA0ABCTL_SPEC;
impl crate::RegisterSpec for UCA0ABCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0abctl::R](R) reader structure"]
impl crate::Readable for UCA0ABCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0abctl::W](W) writer structure"]
impl crate::Writable for UCA0ABCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0ABCTL to value 0"]
impl crate::Resettable for UCA0ABCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
