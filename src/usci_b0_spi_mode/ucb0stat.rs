#[doc = "Register `UCB0STAT` reader"]
pub struct R(crate::R<UCB0STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0STAT` writer"]
pub struct W(crate::W<UCB0STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0STAT_SPEC>;
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
impl From<crate::W<UCB0STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBUSY` reader - USCI Busy Flag"]
pub struct UCBUSY_R(crate::FieldReader<bool, bool>);
impl UCBUSY_R {
    #[inline(always)]
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
#[doc = "Field `UCOE` reader - USCI Overrun Error Flag"]
pub struct UCOE_R(crate::FieldReader<bool, bool>);
impl UCOE_R {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
impl R {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UCBUSY_R {
        UCBUSY_R::new((self.bits & 0x01) != 0)
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
}
impl W {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&mut self) -> UCBUSY_W {
        UCBUSY_W { w: self }
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0stat](index.html) module"]
pub struct UCB0STAT_SPEC;
impl crate::RegisterSpec for UCB0STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0stat::R](R) reader structure"]
impl crate::Readable for UCB0STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0stat::W](W) writer structure"]
impl crate::Writable for UCB0STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0STAT to value 0"]
impl crate::Resettable for UCB0STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
