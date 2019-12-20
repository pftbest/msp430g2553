#[doc = "Reader of register BCSCTL3"]
pub type R = crate::R<u8, super::BCSCTL3>;
#[doc = "Writer for register BCSCTL3"]
pub type W = crate::W<u8, super::BCSCTL3>;
#[doc = "Register BCSCTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::BCSCTL3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LFXT1OF`"]
pub type LFXT1OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXT1OF`"]
pub struct LFXT1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXT1OF_W<'a> {
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
#[doc = "Reader of field `XT2OF`"]
pub type XT2OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT2OF`"]
pub struct XT2OF_W<'a> {
    w: &'a mut W,
}
impl<'a> XT2OF_W<'a> {
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
#[doc = "XIN/XOUT Cap 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XCAP_A {
    #[doc = "0: XIN/XOUT Cap : 0 pF"]
    XCAP_0,
    #[doc = "1: XIN/XOUT Cap : 6 pF"]
    XCAP_1,
    #[doc = "2: XIN/XOUT Cap : 10 pF"]
    XCAP_2,
    #[doc = "3: XIN/XOUT Cap : 12.5 pF"]
    XCAP_3,
}
impl From<XCAP_A> for u8 {
    #[inline(always)]
    fn from(variant: XCAP_A) -> Self {
        match variant {
            XCAP_A::XCAP_0 => 0,
            XCAP_A::XCAP_1 => 1,
            XCAP_A::XCAP_2 => 2,
            XCAP_A::XCAP_3 => 3,
        }
    }
}
#[doc = "Reader of field `XCAP`"]
pub type XCAP_R = crate::R<u8, XCAP_A>;
impl XCAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XCAP_A {
        match self.bits {
            0 => XCAP_A::XCAP_0,
            1 => XCAP_A::XCAP_1,
            2 => XCAP_A::XCAP_2,
            3 => XCAP_A::XCAP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XCAP_0`"]
    #[inline(always)]
    pub fn is_xcap_0(&self) -> bool {
        *self == XCAP_A::XCAP_0
    }
    #[doc = "Checks if the value of the field is `XCAP_1`"]
    #[inline(always)]
    pub fn is_xcap_1(&self) -> bool {
        *self == XCAP_A::XCAP_1
    }
    #[doc = "Checks if the value of the field is `XCAP_2`"]
    #[inline(always)]
    pub fn is_xcap_2(&self) -> bool {
        *self == XCAP_A::XCAP_2
    }
    #[doc = "Checks if the value of the field is `XCAP_3`"]
    #[inline(always)]
    pub fn is_xcap_3(&self) -> bool {
        *self == XCAP_A::XCAP_3
    }
}
#[doc = "Write proxy for field `XCAP`"]
pub struct XCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> XCAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XCAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "XIN/XOUT Cap : 0 pF"]
    #[inline(always)]
    pub fn xcap_0(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_0)
    }
    #[doc = "XIN/XOUT Cap : 6 pF"]
    #[inline(always)]
    pub fn xcap_1(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_1)
    }
    #[doc = "XIN/XOUT Cap : 10 pF"]
    #[inline(always)]
    pub fn xcap_2(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_2)
    }
    #[doc = "XIN/XOUT Cap : 12.5 pF"]
    #[inline(always)]
    pub fn xcap_3(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
#[doc = "Mode 0 for LFXT1 (XTS = 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXT1S_A {
    #[doc = "0: Mode 0 for LFXT1 : Normal operation"]
    LFXT1S_0,
    #[doc = "1: Mode 1 for LFXT1 : Reserved"]
    LFXT1S_1,
    #[doc = "2: Mode 2 for LFXT1 : VLO"]
    LFXT1S_2,
    #[doc = "3: Mode 3 for LFXT1 : Digital input signal"]
    LFXT1S_3,
}
impl From<LFXT1S_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXT1S_A) -> Self {
        match variant {
            LFXT1S_A::LFXT1S_0 => 0,
            LFXT1S_A::LFXT1S_1 => 1,
            LFXT1S_A::LFXT1S_2 => 2,
            LFXT1S_A::LFXT1S_3 => 3,
        }
    }
}
#[doc = "Reader of field `LFXT1S`"]
pub type LFXT1S_R = crate::R<u8, LFXT1S_A>;
impl LFXT1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXT1S_A {
        match self.bits {
            0 => LFXT1S_A::LFXT1S_0,
            1 => LFXT1S_A::LFXT1S_1,
            2 => LFXT1S_A::LFXT1S_2,
            3 => LFXT1S_A::LFXT1S_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LFXT1S_0`"]
    #[inline(always)]
    pub fn is_lfxt1s_0(&self) -> bool {
        *self == LFXT1S_A::LFXT1S_0
    }
    #[doc = "Checks if the value of the field is `LFXT1S_1`"]
    #[inline(always)]
    pub fn is_lfxt1s_1(&self) -> bool {
        *self == LFXT1S_A::LFXT1S_1
    }
    #[doc = "Checks if the value of the field is `LFXT1S_2`"]
    #[inline(always)]
    pub fn is_lfxt1s_2(&self) -> bool {
        *self == LFXT1S_A::LFXT1S_2
    }
    #[doc = "Checks if the value of the field is `LFXT1S_3`"]
    #[inline(always)]
    pub fn is_lfxt1s_3(&self) -> bool {
        *self == LFXT1S_A::LFXT1S_3
    }
}
#[doc = "Write proxy for field `LFXT1S`"]
pub struct LFXT1S_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXT1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXT1S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Mode 0 for LFXT1 : Normal operation"]
    #[inline(always)]
    pub fn lfxt1s_0(self) -> &'a mut W {
        self.variant(LFXT1S_A::LFXT1S_0)
    }
    #[doc = "Mode 1 for LFXT1 : Reserved"]
    #[inline(always)]
    pub fn lfxt1s_1(self) -> &'a mut W {
        self.variant(LFXT1S_A::LFXT1S_1)
    }
    #[doc = "Mode 2 for LFXT1 : VLO"]
    #[inline(always)]
    pub fn lfxt1s_2(self) -> &'a mut W {
        self.variant(LFXT1S_A::LFXT1S_2)
    }
    #[doc = "Mode 3 for LFXT1 : Digital input signal"]
    #[inline(always)]
    pub fn lfxt1s_3(self) -> &'a mut W {
        self.variant(LFXT1S_A::LFXT1S_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Mode 0 for XT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT2S_A {
    #[doc = "0: Mode 0 for XT2 : 0.4 - 1 MHz"]
    XT2S_0,
    #[doc = "1: Mode 1 for XT2 : 1 - 4 MHz"]
    XT2S_1,
    #[doc = "2: Mode 2 for XT2 : 2 - 16 MHz"]
    XT2S_2,
    #[doc = "3: Mode 3 for XT2 : Digital input signal"]
    XT2S_3,
}
impl From<XT2S_A> for u8 {
    #[inline(always)]
    fn from(variant: XT2S_A) -> Self {
        match variant {
            XT2S_A::XT2S_0 => 0,
            XT2S_A::XT2S_1 => 1,
            XT2S_A::XT2S_2 => 2,
            XT2S_A::XT2S_3 => 3,
        }
    }
}
#[doc = "Reader of field `XT2S`"]
pub type XT2S_R = crate::R<u8, XT2S_A>;
impl XT2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT2S_A {
        match self.bits {
            0 => XT2S_A::XT2S_0,
            1 => XT2S_A::XT2S_1,
            2 => XT2S_A::XT2S_2,
            3 => XT2S_A::XT2S_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT2S_0`"]
    #[inline(always)]
    pub fn is_xt2s_0(&self) -> bool {
        *self == XT2S_A::XT2S_0
    }
    #[doc = "Checks if the value of the field is `XT2S_1`"]
    #[inline(always)]
    pub fn is_xt2s_1(&self) -> bool {
        *self == XT2S_A::XT2S_1
    }
    #[doc = "Checks if the value of the field is `XT2S_2`"]
    #[inline(always)]
    pub fn is_xt2s_2(&self) -> bool {
        *self == XT2S_A::XT2S_2
    }
    #[doc = "Checks if the value of the field is `XT2S_3`"]
    #[inline(always)]
    pub fn is_xt2s_3(&self) -> bool {
        *self == XT2S_A::XT2S_3
    }
}
#[doc = "Write proxy for field `XT2S`"]
pub struct XT2S_W<'a> {
    w: &'a mut W,
}
impl<'a> XT2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT2S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Mode 0 for XT2 : 0.4 - 1 MHz"]
    #[inline(always)]
    pub fn xt2s_0(self) -> &'a mut W {
        self.variant(XT2S_A::XT2S_0)
    }
    #[doc = "Mode 1 for XT2 : 1 - 4 MHz"]
    #[inline(always)]
    pub fn xt2s_1(self) -> &'a mut W {
        self.variant(XT2S_A::XT2S_1)
    }
    #[doc = "Mode 2 for XT2 : 2 - 16 MHz"]
    #[inline(always)]
    pub fn xt2s_2(self) -> &'a mut W {
        self.variant(XT2S_A::XT2S_2)
    }
    #[doc = "Mode 3 for XT2 : Digital input signal"]
    #[inline(always)]
    pub fn xt2s_3(self) -> &'a mut W {
        self.variant(XT2S_A::XT2S_3)
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
    #[doc = "Bit 0 - Low/high Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxt1of(&self) -> LFXT1OF_R {
        LFXT1OF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High frequency oscillator 2 fault flag"]
    #[inline(always)]
    pub fn xt2of(&self) -> XT2OF_R {
        XT2OF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn xcap(&self) -> XCAP_R {
        XCAP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Mode 0 for LFXT1 (XTS = 0)"]
    #[inline(always)]
    pub fn lfxt1s(&self) -> LFXT1S_R {
        LFXT1S_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Mode 0 for XT2"]
    #[inline(always)]
    pub fn xt2s(&self) -> XT2S_R {
        XT2S_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low/high Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxt1of(&mut self) -> LFXT1OF_W {
        LFXT1OF_W { w: self }
    }
    #[doc = "Bit 1 - High frequency oscillator 2 fault flag"]
    #[inline(always)]
    pub fn xt2of(&mut self) -> XT2OF_W {
        XT2OF_W { w: self }
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn xcap(&mut self) -> XCAP_W {
        XCAP_W { w: self }
    }
    #[doc = "Bits 4:5 - Mode 0 for LFXT1 (XTS = 0)"]
    #[inline(always)]
    pub fn lfxt1s(&mut self) -> LFXT1S_W {
        LFXT1S_W { w: self }
    }
    #[doc = "Bits 6:7 - Mode 0 for XT2"]
    #[inline(always)]
    pub fn xt2s(&mut self) -> XT2S_W {
        XT2S_W { w: self }
    }
}
