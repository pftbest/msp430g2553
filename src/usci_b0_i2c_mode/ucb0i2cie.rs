#[doc = "Register `UCB0I2CIE` reader"]
pub struct R(crate::R<UCB0I2CIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2CIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB0I2CIE_SPEC>> for R {
    fn from(reader: crate::R<UCB0I2CIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0I2CIE` writer"]
pub struct W(crate::W<UCB0I2CIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0I2CIE_SPEC>;
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
impl core::convert::From<crate::W<UCB0I2CIE_SPEC>> for W {
    fn from(writer: crate::W<UCB0I2CIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCALIE` reader - Arbitration Lost interrupt enable"]
pub struct UCALIE_R(crate::FieldReader<bool, bool>);
impl UCALIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCALIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCALIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCALIE` writer - Arbitration Lost interrupt enable"]
pub struct UCALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCALIE_W<'a> {
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
#[doc = "Field `UCSTTIE` reader - START Condition interrupt enable"]
pub struct UCSTTIE_R(crate::FieldReader<bool, bool>);
impl UCSTTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTTIE` writer - START Condition interrupt enable"]
pub struct UCSTTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIE_W<'a> {
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
#[doc = "Field `UCSTPIE` reader - STOP Condition interrupt enable"]
pub struct UCSTPIE_R(crate::FieldReader<bool, bool>);
impl UCSTPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTPIE` writer - STOP Condition interrupt enable"]
pub struct UCSTPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPIE_W<'a> {
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
#[doc = "Field `UCNACKIE` reader - NACK Condition interrupt enable"]
pub struct UCNACKIE_R(crate::FieldReader<bool, bool>);
impl UCNACKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCNACKIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCNACKIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCNACKIE` writer - NACK Condition interrupt enable"]
pub struct UCNACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCNACKIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W {
        UCALIE_W { w: self }
    }
    #[doc = "Bit 1 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W {
        UCSTTIE_W { w: self }
    }
    #[doc = "Bit 2 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W {
        UCSTPIE_W { w: self }
    }
    #[doc = "Bit 3 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W {
        UCNACKIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 I2C Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2cie](index.html) module"]
pub struct UCB0I2CIE_SPEC;
impl crate::RegisterSpec for UCB0I2CIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0i2cie::R](R) reader structure"]
impl crate::Readable for UCB0I2CIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0i2cie::W](W) writer structure"]
impl crate::Writable for UCB0I2CIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0I2CIE to value 0"]
impl crate::Resettable for UCB0I2CIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
