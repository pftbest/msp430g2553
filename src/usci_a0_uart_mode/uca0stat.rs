#[doc = "Register `UCA0STAT` reader"]
pub struct R(crate::R<UCA0STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCA0STAT_SPEC>> for R {
    fn from(reader: crate::R<UCA0STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0STAT` writer"]
pub struct W(crate::W<UCA0STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0STAT_SPEC>;
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
impl core::convert::From<crate::W<UCA0STAT_SPEC>> for W {
    fn from(writer: crate::W<UCA0STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBUSY` reader - USCI Busy Flag"]
pub struct UCBUSY_R(crate::FieldReader<bool, bool>);
impl UCBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBUSY` writer - USCI Busy Flag"]
pub struct UCBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBUSY_W<'a> {
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
#[doc = "Field `UCADDR` reader - USCI Address received Flag"]
pub struct UCADDR_R(crate::FieldReader<bool, bool>);
impl UCADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDR` writer - USCI Address received Flag"]
pub struct UCADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDR_W<'a> {
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
#[doc = "Field `UCRXERR` reader - USCI RX Error Flag"]
pub struct UCRXERR_R(crate::FieldReader<bool, bool>);
impl UCRXERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXERR` writer - USCI RX Error Flag"]
pub struct UCRXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXERR_W<'a> {
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
#[doc = "Field `UCBRK` reader - USCI Break received"]
pub struct UCBRK_R(crate::FieldReader<bool, bool>);
impl UCBRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBRK` writer - USCI Break received"]
pub struct UCBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRK_W<'a> {
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
#[doc = "Field `UCPE` reader - USCI Parity Error Flag"]
pub struct UCPE_R(crate::FieldReader<bool, bool>);
impl UCPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPE` writer - USCI Parity Error Flag"]
pub struct UCPE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPE_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UCOE` reader - USCI Overrun Error Flag"]
pub struct UCOE_R(crate::FieldReader<bool, bool>);
impl UCOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOE` writer - USCI Overrun Error Flag"]
pub struct UCOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOE_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UCFE` reader - USCI Frame Error Flag"]
pub struct UCFE_R(crate::FieldReader<bool, bool>);
impl UCFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCFE` writer - USCI Frame Error Flag"]
pub struct UCFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCFE_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `UCLISTEN` reader - USCI Listen mode"]
pub struct UCLISTEN_R(crate::FieldReader<bool, bool>);
impl UCLISTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCLISTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCLISTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCLISTEN` writer - USCI Listen mode"]
pub struct UCLISTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCLISTEN_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `UCIDLE` reader - Idle line detected"]
pub struct UCIDLE_R(crate::FieldReader<bool, bool>);
impl UCIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIDLE` writer - Idle line detected"]
pub struct UCIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIDLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UCBUSY_R {
        UCBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Address received Flag"]
    #[inline(always)]
    pub fn ucaddr(&self) -> UCADDR_R {
        UCADDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USCI RX Error Flag"]
    #[inline(always)]
    pub fn ucrxerr(&self) -> UCRXERR_R {
        UCRXERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USCI Break received"]
    #[inline(always)]
    pub fn ucbrk(&self) -> UCBRK_R {
        UCBRK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USCI Parity Error Flag"]
    #[inline(always)]
    pub fn ucpe(&self) -> UCPE_R {
        UCPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UCOE_R {
        UCOE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UCFE_R {
        UCFE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Idle line detected"]
    #[inline(always)]
    pub fn ucidle(&self) -> UCIDLE_R {
        UCIDLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&mut self) -> UCBUSY_W {
        UCBUSY_W { w: self }
    }
    #[doc = "Bit 1 - USCI Address received Flag"]
    #[inline(always)]
    pub fn ucaddr(&mut self) -> UCADDR_W {
        UCADDR_W { w: self }
    }
    #[doc = "Bit 2 - USCI RX Error Flag"]
    #[inline(always)]
    pub fn ucrxerr(&mut self) -> UCRXERR_W {
        UCRXERR_W { w: self }
    }
    #[doc = "Bit 3 - USCI Break received"]
    #[inline(always)]
    pub fn ucbrk(&mut self) -> UCBRK_W {
        UCBRK_W { w: self }
    }
    #[doc = "Bit 4 - USCI Parity Error Flag"]
    #[inline(always)]
    pub fn ucpe(&mut self) -> UCPE_W {
        UCPE_W { w: self }
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    pub fn ucoe(&mut self) -> UCOE_W {
        UCOE_W { w: self }
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    pub fn ucfe(&mut self) -> UCFE_W {
        UCFE_W { w: self }
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UCLISTEN_W {
        UCLISTEN_W { w: self }
    }
    #[doc = "Bit 1 - Idle line detected"]
    #[inline(always)]
    pub fn ucidle(&mut self) -> UCIDLE_W {
        UCIDLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0stat](index.html) module"]
pub struct UCA0STAT_SPEC;
impl crate::RegisterSpec for UCA0STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0stat::R](R) reader structure"]
impl crate::Readable for UCA0STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0stat::W](W) writer structure"]
impl crate::Writable for UCA0STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0STAT to value 0"]
impl crate::Resettable for UCA0STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
