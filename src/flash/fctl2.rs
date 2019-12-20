#[doc = "Reader of register FCTL2"]
pub type R = crate::R<u16, super::FCTL2>;
#[doc = "Writer for register FCTL2"]
pub type W = crate::W<u16, super::FCTL2>;
#[doc = "Register FCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FN0`"]
pub type FN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FN0`"]
pub struct FN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FN0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `FN1`"]
pub type FN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FN1`"]
pub struct FN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FN1_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FN2`"]
pub type FN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FN2`"]
pub struct FN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FN2_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FN3`"]
pub type FN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FN3`"]
pub struct FN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FN3_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FN4`"]
pub type FN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FN4`"]
pub struct FN4_W<'a> {
    w: &'a mut W,
}
impl<'a> FN4_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FN5`"]
pub type FN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FN5`"]
pub struct FN5_W<'a> {
    w: &'a mut W,
}
impl<'a> FN5_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Flash clock select 0 */ /* to distinguish from USART SSELx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSSEL_A {
    #[doc = "0: Flash clock select: 0 - ACLK"]
    FSSEL_0,
    #[doc = "1: Flash clock select: 1 - MCLK"]
    FSSEL_1,
    #[doc = "2: Flash clock select: 2 - SMCLK"]
    FSSEL_2,
    #[doc = "3: Flash clock select: 3 - SMCLK"]
    FSSEL_3,
}
impl From<FSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSSEL_A) -> Self {
        match variant {
            FSSEL_A::FSSEL_0 => 0,
            FSSEL_A::FSSEL_1 => 1,
            FSSEL_A::FSSEL_2 => 2,
            FSSEL_A::FSSEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `FSSEL`"]
pub type FSSEL_R = crate::R<u8, FSSEL_A>;
impl FSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSSEL_A {
        match self.bits {
            0 => FSSEL_A::FSSEL_0,
            1 => FSSEL_A::FSSEL_1,
            2 => FSSEL_A::FSSEL_2,
            3 => FSSEL_A::FSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FSSEL_0`"]
    #[inline(always)]
    pub fn is_fssel_0(&self) -> bool {
        *self == FSSEL_A::FSSEL_0
    }
    #[doc = "Checks if the value of the field is `FSSEL_1`"]
    #[inline(always)]
    pub fn is_fssel_1(&self) -> bool {
        *self == FSSEL_A::FSSEL_1
    }
    #[doc = "Checks if the value of the field is `FSSEL_2`"]
    #[inline(always)]
    pub fn is_fssel_2(&self) -> bool {
        *self == FSSEL_A::FSSEL_2
    }
    #[doc = "Checks if the value of the field is `FSSEL_3`"]
    #[inline(always)]
    pub fn is_fssel_3(&self) -> bool {
        *self == FSSEL_A::FSSEL_3
    }
}
#[doc = "Write proxy for field `FSSEL`"]
pub struct FSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Flash clock select: 0 - ACLK"]
    #[inline(always)]
    pub fn fssel_0(self) -> &'a mut W {
        self.variant(FSSEL_A::FSSEL_0)
    }
    #[doc = "Flash clock select: 1 - MCLK"]
    #[inline(always)]
    pub fn fssel_1(self) -> &'a mut W {
        self.variant(FSSEL_A::FSSEL_1)
    }
    #[doc = "Flash clock select: 2 - SMCLK"]
    #[inline(always)]
    pub fn fssel_2(self) -> &'a mut W {
        self.variant(FSSEL_A::FSSEL_2)
    }
    #[doc = "Flash clock select: 3 - SMCLK"]
    #[inline(always)]
    pub fn fssel_3(self) -> &'a mut W {
        self.variant(FSSEL_A::FSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn0(&self) -> FN0_R {
        FN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
    #[inline(always)]
    pub fn fn1(&self) -> FN1_R {
        FN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FN2"]
    #[inline(always)]
    pub fn fn2(&self) -> FN2_R {
        FN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FN3"]
    #[inline(always)]
    pub fn fn3(&self) -> FN3_R {
        FN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FN4"]
    #[inline(always)]
    pub fn fn4(&self) -> FN4_R {
        FN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FN5"]
    #[inline(always)]
    pub fn fn5(&self) -> FN5_R {
        FN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&self) -> FSSEL_R {
        FSSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn0(&mut self) -> FN0_W {
        FN0_W { w: self }
    }
    #[doc = "Bit 1 - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
    #[inline(always)]
    pub fn fn1(&mut self) -> FN1_W {
        FN1_W { w: self }
    }
    #[doc = "Bit 2 - FN2"]
    #[inline(always)]
    pub fn fn2(&mut self) -> FN2_W {
        FN2_W { w: self }
    }
    #[doc = "Bit 3 - FN3"]
    #[inline(always)]
    pub fn fn3(&mut self) -> FN3_W {
        FN3_W { w: self }
    }
    #[doc = "Bit 4 - FN4"]
    #[inline(always)]
    pub fn fn4(&mut self) -> FN4_W {
        FN4_W { w: self }
    }
    #[doc = "Bit 5 - FN5"]
    #[inline(always)]
    pub fn fn5(&mut self) -> FN5_W {
        FN5_W { w: self }
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&mut self) -> FSSEL_W {
        FSSEL_W { w: self }
    }
}
