#[doc = "Register `UCA0IRTCTL` reader"]
pub struct R(crate::R<UCA0IRTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IRTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IRTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IRTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IRTCTL` writer"]
pub struct W(crate::W<UCA0IRTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IRTCTL_SPEC>;
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
impl From<crate::W<UCA0IRTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IRTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIREN` reader - IRDA Encoder/Decoder enable"]
pub struct UCIREN_R(crate::FieldReader<bool, bool>);
impl UCIREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIREN` writer - IRDA Encoder/Decoder enable"]
pub struct UCIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `UCIRTXCLK` reader - IRDA Transmit Pulse Clock Select"]
pub struct UCIRTXCLK_R(crate::FieldReader<bool, bool>);
impl UCIRTXCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXCLK` writer - IRDA Transmit Pulse Clock Select"]
pub struct UCIRTXCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXCLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UCIRTXPL` reader - IRDA Transmit Pulse Length 0"]
pub struct UCIRTXPL_R(crate::FieldReader<u8, u8>);
impl UCIRTXPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UCIRTXPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXPL` writer - IRDA Transmit Pulse Length 0"]
pub struct UCIRTXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x3f << 2)) | ((value as u8 & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl(&self) -> UCIRTXPL_R {
        UCIRTXPL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UCIREN_W {
        UCIREN_W { w: self }
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W {
        UCIRTXCLK_W { w: self }
    }
    #[doc = "Bits 2:7 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl(&mut self) -> UCIRTXPL_W {
        UCIRTXPL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 IrDA Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0irtctl](index.html) module"]
pub struct UCA0IRTCTL_SPEC;
impl crate::RegisterSpec for UCA0IRTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0irtctl::R](R) reader structure"]
impl crate::Readable for UCA0IRTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0irtctl::W](W) writer structure"]
impl crate::Writable for UCA0IRTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IRTCTL to value 0"]
impl crate::Resettable for UCA0IRTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
