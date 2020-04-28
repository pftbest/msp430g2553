#[doc = "Reader of register UCA0BR0"]
pub type R = crate::R<u8, super::UCA0BR0>;
#[doc = "Writer for register UCA0BR0"]
pub type W = crate::W<u8, super::UCA0BR0>;
#[doc = "Register UCA0BR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0BR0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCA0BR0`"]
pub type UCA0BR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCA0BR0`"]
pub struct UCA0BR0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0BR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca0br0(&self) -> UCA0BR0_R {
        UCA0BR0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca0br0(&mut self) -> UCA0BR0_W {
        UCA0BR0_W { w: self }
    }
}
