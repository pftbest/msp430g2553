#[doc = "Reader of register IFG2"]
pub type R = crate::R<u8, super::IFG2>;
#[doc = "Writer for register IFG2"]
pub type W = crate::W<u8, super::IFG2>;
#[doc = "Register IFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IFG2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCA0RXIFG`"]
pub type UCA0RXIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCA0RXIFG`"]
pub struct UCA0RXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0RXIFG_W<'a> {
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
#[doc = "Reader of field `UCA0TXIFG`"]
pub type UCA0TXIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCA0TXIFG`"]
pub struct UCA0TXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0TXIFG_W<'a> {
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
#[doc = "Reader of field `UCB0RXIFG`"]
pub type UCB0RXIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCB0RXIFG`"]
pub struct UCB0RXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0RXIFG_W<'a> {
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
#[doc = "Reader of field `UCB0TXIFG`"]
pub type UCB0TXIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCB0TXIFG`"]
pub struct UCB0TXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0TXIFG_W<'a> {
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
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&self) -> UCA0RXIFG_R {
        UCA0RXIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&self) -> UCA0TXIFG_R {
        UCA0TXIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&self) -> UCB0RXIFG_R {
        UCB0RXIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&self) -> UCB0TXIFG_R {
        UCB0TXIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&mut self) -> UCA0RXIFG_W {
        UCA0RXIFG_W { w: self }
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&mut self) -> UCA0TXIFG_W {
        UCA0TXIFG_W { w: self }
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&mut self) -> UCB0RXIFG_W {
        UCB0RXIFG_W { w: self }
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&mut self) -> UCB0TXIFG_W {
        UCB0TXIFG_W { w: self }
    }
}
