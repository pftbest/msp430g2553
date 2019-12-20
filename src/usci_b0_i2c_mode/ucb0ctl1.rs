#[doc = "Reader of register UCB0CTL1"]
pub type R = crate::R<u8, super::UCB0CTL1>;
#[doc = "Writer for register UCB0CTL1"]
pub type W = crate::W<u8, super::UCB0CTL1>;
#[doc = "Register UCB0CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0CTL1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCSWRST`"]
pub type UCSWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSWRST`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `UCTXSTT`"]
pub type UCTXSTT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXSTT`"]
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
            (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `UCTXSTP`"]
pub type UCTXSTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXSTP`"]
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
            (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UCTXNACK`"]
pub type UCTXNACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXNACK`"]
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
            (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UCTR`"]
pub type UCTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTR`"]
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
            (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "USCI 1 Clock Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSSEL_A {
    #[doc = "0: USCI 0 Clock Source: 0"]
    UCSSEL_0,
    #[doc = "1: USCI 0 Clock Source: 1"]
    UCSSEL_1,
    #[doc = "2: USCI 0 Clock Source: 2"]
    UCSSEL_2,
    #[doc = "3: USCI 0 Clock Source: 3"]
    UCSSEL_3,
}
impl From<UCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSSEL_A) -> Self {
        match variant {
            UCSSEL_A::UCSSEL_0 => 0,
            UCSSEL_A::UCSSEL_1 => 1,
            UCSSEL_A::UCSSEL_2 => 2,
            UCSSEL_A::UCSSEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `UCSSEL`"]
pub type UCSSEL_R = crate::R<u8, UCSSEL_A>;
impl UCSSEL_R {
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
        *self == UCSSEL_A::UCSSEL_0
    }
    #[doc = "Checks if the value of the field is `UCSSEL_1`"]
    #[inline(always)]
    pub fn is_ucssel_1(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_1
    }
    #[doc = "Checks if the value of the field is `UCSSEL_2`"]
    #[inline(always)]
    pub fn is_ucssel_2(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_2
    }
    #[doc = "Checks if the value of the field is `UCSSEL_3`"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_3
    }
}
#[doc = "Write proxy for field `UCSSEL`"]
pub struct UCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
            (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
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
}
