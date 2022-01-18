#[doc = "Register `UCB0CTL1` reader"]
pub struct R(crate::R<UCB0CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0CTL1` writer"]
pub struct W(crate::W<UCB0CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0CTL1_SPEC>;
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
impl From<crate::W<UCB0CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSWRST` reader - USCI Software Reset"]
pub struct UCSWRST_R(crate::FieldReader<bool, bool>);
impl UCSWRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSWRST` writer - USCI Software Reset"]
pub struct UCSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSWRST_W<'a> {
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
#[doc = "Field `UCTXSTT` reader - Transmit START"]
pub struct UCTXSTT_R(crate::FieldReader<bool, bool>);
impl UCTXSTT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCTXSTT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXSTT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXSTT` writer - Transmit START"]
pub struct UCTXSTT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXSTT_W<'a> {
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
#[doc = "Field `UCTXSTP` reader - Transmit STOP"]
pub struct UCTXSTP_R(crate::FieldReader<bool, bool>);
impl UCTXSTP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCTXSTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXSTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXSTP` writer - Transmit STOP"]
pub struct UCTXSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXSTP_W<'a> {
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
#[doc = "Field `UCTXNACK` reader - Transmit NACK"]
pub struct UCTXNACK_R(crate::FieldReader<bool, bool>);
impl UCTXNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCTXNACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXNACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXNACK` writer - Transmit NACK"]
pub struct UCTXNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXNACK_W<'a> {
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
#[doc = "Field `UCTR` reader - Transmit/Receive Select/Flag"]
pub struct UCTR_R(crate::FieldReader<bool, bool>);
impl UCTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTR` writer - Transmit/Receive Select/Flag"]
pub struct UCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTR_W<'a> {
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
#[doc = "USCI 1 Clock Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCSSEL_A {
    #[doc = "0: USCI 0 Clock Source: 0"]
    UCSSEL_0 = 0,
    #[doc = "1: USCI 0 Clock Source: 1"]
    UCSSEL_1 = 1,
    #[doc = "2: USCI 0 Clock Source: 2"]
    UCSSEL_2 = 2,
    #[doc = "3: USCI 0 Clock Source: 3"]
    UCSSEL_3 = 3,
}
impl From<UCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCSSEL` reader - USCI 1 Clock Source Select 1"]
pub struct UCSSEL_R(crate::FieldReader<u8, UCSSEL_A>);
impl UCSSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UCSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSSEL_A {
        match self.bits {
            0 => UCSSEL_A::UCSSEL_0,
            1 => UCSSEL_A::UCSSEL_1,
            2 => UCSSEL_A::UCSSEL_2,
            3 => UCSSEL_A::UCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCSSEL_0`"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        **self == UCSSEL_A::UCSSEL_0
    }
    #[doc = "Checks if the value of the field is `UCSSEL_1`"]
    #[inline(always)]
    pub fn is_ucssel_1(&self) -> bool {
        **self == UCSSEL_A::UCSSEL_1
    }
    #[doc = "Checks if the value of the field is `UCSSEL_2`"]
    #[inline(always)]
    pub fn is_ucssel_2(&self) -> bool {
        **self == UCSSEL_A::UCSSEL_2
    }
    #[doc = "Checks if the value of the field is `UCSSEL_3`"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        **self == UCSSEL_A::UCSSEL_3
    }
}
impl core::ops::Deref for UCSSEL_R {
    type Target = crate::FieldReader<u8, UCSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSSEL` writer - USCI 1 Clock Source Select 1"]
pub struct UCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "USCI 0 Clock Source: 0"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_0)
    }
    #[doc = "USCI 0 Clock Source: 1"]
    #[inline(always)]
    pub fn ucssel_1(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_1)
    }
    #[doc = "USCI 0 Clock Source: 2"]
    #[inline(always)]
    pub fn ucssel_2(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_2)
    }
    #[doc = "USCI 0 Clock Source: 3"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit START"]
    #[inline(always)]
    pub fn uctxstt(&self) -> UCTXSTT_R {
        UCTXSTT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit STOP"]
    #[inline(always)]
    pub fn uctxstp(&self) -> UCTXSTP_R {
        UCTXSTP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit NACK"]
    #[inline(always)]
    pub fn uctxnack(&self) -> UCTXNACK_R {
        UCTXNACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit/Receive Select/Flag"]
    #[inline(always)]
    pub fn uctr(&self) -> UCTR_R {
        UCTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - USCI 1 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&self) -> UCSSEL_R {
        UCSSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W {
        UCSWRST_W { w: self }
    }
    #[doc = "Bit 1 - Transmit START"]
    #[inline(always)]
    pub fn uctxstt(&mut self) -> UCTXSTT_W {
        UCTXSTT_W { w: self }
    }
    #[doc = "Bit 2 - Transmit STOP"]
    #[inline(always)]
    pub fn uctxstp(&mut self) -> UCTXSTP_W {
        UCTXSTP_W { w: self }
    }
    #[doc = "Bit 3 - Transmit NACK"]
    #[inline(always)]
    pub fn uctxnack(&mut self) -> UCTXNACK_W {
        UCTXNACK_W { w: self }
    }
    #[doc = "Bit 4 - Transmit/Receive Select/Flag"]
    #[inline(always)]
    pub fn uctr(&mut self) -> UCTR_W {
        UCTR_W { w: self }
    }
    #[doc = "Bits 6:7 - USCI 1 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UCSSEL_W {
        UCSSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl1](index.html) module"]
pub struct UCB0CTL1_SPEC;
impl crate::RegisterSpec for UCB0CTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0ctl1::R](R) reader structure"]
impl crate::Readable for UCB0CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ctl1::W](W) writer structure"]
impl crate::Writable for UCB0CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0CTL1 to value 0"]
impl crate::Resettable for UCB0CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
