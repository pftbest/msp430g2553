#[doc = "Reader of register IE2"]
pub type R = crate::R<u8, super::IE2>;
#[doc = "Writer for register IE2"]
pub type W = crate::W<u8, super::IE2>;
#[doc = "Register IE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IE2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCA0RXIE`"]
pub type UCA0RXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCA0RXIE`"]
pub struct UCA0RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0RXIE_W<'a> {
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
#[doc = "Reader of field `UCA0TXIE`"]
pub type UCA0TXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCA0TXIE`"]
pub struct UCA0TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0TXIE_W<'a> {
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
#[doc = "Reader of field `UCB0RXIE`"]
pub type UCB0RXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCB0RXIE`"]
pub struct UCB0RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0RXIE_W<'a> {
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
#[doc = "Reader of field `UCB0TXIE`"]
pub type UCB0TXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCB0TXIE`"]
pub struct UCB0TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0TXIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - UCA0RXIE"]
    #[inline(always)]
    pub fn uca0rxie(&self) -> UCA0RXIE_R {
        UCA0RXIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIE"]
    #[inline(always)]
    pub fn uca0txie(&self) -> UCA0TXIE_R {
        UCA0TXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIE"]
    #[inline(always)]
    pub fn ucb0rxie(&self) -> UCB0RXIE_R {
        UCB0RXIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIE"]
    #[inline(always)]
    pub fn ucb0txie(&self) -> UCB0TXIE_R {
        UCB0TXIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIE"]
    #[inline(always)]
    pub fn uca0rxie(&mut self) -> UCA0RXIE_W {
        UCA0RXIE_W { w: self }
    }
    #[doc = "Bit 1 - UCA0TXIE"]
    #[inline(always)]
    pub fn uca0txie(&mut self) -> UCA0TXIE_W {
        UCA0TXIE_W { w: self }
    }
    #[doc = "Bit 2 - UCB0RXIE"]
    #[inline(always)]
    pub fn ucb0rxie(&mut self) -> UCB0RXIE_W {
        UCB0RXIE_W { w: self }
    }
    #[doc = "Bit 3 - UCB0TXIE"]
    #[inline(always)]
    pub fn ucb0txie(&mut self) -> UCB0TXIE_W {
        UCB0TXIE_W { w: self }
    }
}
