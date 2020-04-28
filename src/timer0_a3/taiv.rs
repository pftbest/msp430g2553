#[doc = "Reader of register TAIV"]
pub type R = crate::R<u16, super::TAIV>;
#[doc = "Writer for register TAIV"]
pub type W = crate::W<u16, super::TAIV>;
#[doc = "Register TAIV `reset()`'s with value 0"]
impl crate::ResetValue for super::TAIV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer A Interrupt Vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Capture/Compare 1"]
    TACCR1 = 2,
    #[doc = "4: Capture/Compare 2"]
    TACCR2 = 4,
    #[doc = "10: Timer overflow"]
    TAIFG = 10,
}
impl From<TAIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TAIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAIV`"]
pub type TAIV_R = crate::R<u8, TAIV_A>;
impl TAIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TAIV_A::NONE),
            2 => Val(TAIV_A::TACCR1),
            4 => Val(TAIV_A::TACCR2),
            10 => Val(TAIV_A::TAIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TAIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `TACCR1`"]
    #[inline(always)]
    pub fn is_taccr1(&self) -> bool {
        *self == TAIV_A::TACCR1
    }
    #[doc = "Checks if the value of the field is `TACCR2`"]
    #[inline(always)]
    pub fn is_taccr2(&self) -> bool {
        *self == TAIV_A::TACCR2
    }
    #[doc = "Checks if the value of the field is `TAIFG`"]
    #[inline(always)]
    pub fn is_taifg(&self) -> bool {
        *self == TAIV_A::TAIFG
    }
}
#[doc = "Write proxy for field `TAIV`"]
pub struct TAIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TAIV_A::NONE)
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn taccr1(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR1)
    }
    #[doc = "Capture/Compare 2"]
    #[inline(always)]
    pub fn taccr2(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR2)
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn taifg(self) -> &'a mut W {
        self.variant(TAIV_A::TAIFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer A Interrupt Vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TAIV_R {
        TAIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer A Interrupt Vector value"]
    #[inline(always)]
    pub fn taiv(&mut self) -> TAIV_W {
        TAIV_W { w: self }
    }
}
