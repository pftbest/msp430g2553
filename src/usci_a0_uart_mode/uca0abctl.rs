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
pub struct UCABDEN_R(crate::FieldReader<bool, bool>);
impl UCABDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCABDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCABDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCABDEN` writer - Auto Baud Rate detect enable"]
pub struct UCABDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCABDEN_W<'a> {
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
#[doc = "Field `UCBTOE` reader - Break Timeout error"]
pub struct UCBTOE_R(crate::FieldReader<bool, bool>);
impl UCBTOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCBTOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBTOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBTOE` writer - Break Timeout error"]
pub struct UCBTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBTOE_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `UCSTOE` reader - Sync-Field Timeout error"]
pub struct UCSTOE_R(crate::FieldReader<bool, bool>);
impl UCSTOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSTOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTOE` writer - Sync-Field Timeout error"]
pub struct UCSTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTOE_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `UCDELIM` reader - Break Sync Delimiter 0"]
pub struct UCDELIM_R(crate::FieldReader<u8, u8>);
impl UCDELIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UCDELIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCDELIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCDELIM` writer - Break Sync Delimiter 0"]
pub struct UCDELIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UCDELIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UCABDEN_R {
        UCABDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UCBTOE_R {
        UCBTOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UCSTOE_R {
        UCSTOE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim(&self) -> UCDELIM_R {
        UCDELIM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&mut self) -> UCABDEN_W {
        UCABDEN_W { w: self }
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&mut self) -> UCBTOE_W {
        UCBTOE_W { w: self }
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&mut self) -> UCSTOE_W {
        UCSTOE_W { w: self }
    }
    #[doc = "Bits 4:5 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim(&mut self) -> UCDELIM_W {
        UCDELIM_W { w: self }
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
