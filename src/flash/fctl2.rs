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
#[doc = "Flash clock select 0 */ /* to distinguish from USART SSELx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSSEL_A {
    #[doc = "0: Flash clock select: 0 - ACLK"]
    FSSEL_0 = 0,
    #[doc = "1: Flash clock select: 1 - MCLK"]
    FSSEL_1 = 1,
    #[doc = "2: Flash clock select: 2 - SMCLK"]
    FSSEL_2 = 2,
    #[doc = "3: Flash clock select: 3 - SMCLK"]
    FSSEL_3 = 3,
}
impl From<FSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSSEL_A) -> Self {
        variant as _
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
#[doc = "FCTL2 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FWKEY_A {
    #[doc = "150: Value always read from the FCTL2 Password register"]
    PASSWORD = 150,
}
impl From<FWKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: FWKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FWKEY`"]
pub type FWKEY_R = crate::R<u8, FWKEY_A>;
impl FWKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FWKEY_A> {
        use crate::Variant::*;
        match self.bits {
            150 => Val(FWKEY_A::PASSWORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASSWORD`"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == FWKEY_A::PASSWORD
    }
}
#[doc = "FCTL2 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FWKEY_AW {
    #[doc = "165: Value which must be written to the FCTL2 Password register"]
    PASSWORD = 165,
}
impl From<FWKEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: FWKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `FWKEY`"]
pub struct FWKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> FWKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWKEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Value which must be written to the FCTL2 Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut W {
        self.variant(FWKEY_AW::PASSWORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FN`"]
pub type FN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FN`"]
pub struct FN_W<'a> {
    w: &'a mut W,
}
impl<'a> FN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&self) -> FSSEL_R {
        FSSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - FCTL2 Password"]
    #[inline(always)]
    pub fn fwkey(&self) -> FWKEY_R {
        FWKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:5 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&mut self) -> FSSEL_W {
        FSSEL_W { w: self }
    }
    #[doc = "Bits 8:15 - FCTL2 Password"]
    #[inline(always)]
    pub fn fwkey(&mut self) -> FWKEY_W {
        FWKEY_W { w: self }
    }
    #[doc = "Bits 0:5 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn_(&mut self) -> FN_W {
        FN_W { w: self }
    }
}
