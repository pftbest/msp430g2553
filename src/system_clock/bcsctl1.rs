#[doc = "Reader of register BCSCTL1"]
pub type R = crate::R<u8, super::BCSCTL1>;
#[doc = "Writer for register BCSCTL1"]
pub type W = crate::W<u8, super::BCSCTL1>;
#[doc = "Register BCSCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BCSCTL1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSEL0`"]
pub type RSEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSEL0`"]
pub struct RSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSEL0_W<'a> {
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
#[doc = "Reader of field `RSEL1`"]
pub type RSEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSEL1`"]
pub struct RSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSEL1_W<'a> {
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
#[doc = "Reader of field `RSEL2`"]
pub type RSEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSEL2`"]
pub struct RSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSEL2_W<'a> {
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
#[doc = "Reader of field `RSEL3`"]
pub type RSEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSEL3`"]
pub struct RSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSEL3_W<'a> {
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
#[doc = "ACLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVA_A {
    #[doc = "0: ACLK Divider 0: /1"]
    DIVA_0,
    #[doc = "1: ACLK Divider 1: /2"]
    DIVA_1,
    #[doc = "2: ACLK Divider 2: /4"]
    DIVA_2,
    #[doc = "3: ACLK Divider 3: /8"]
    DIVA_3,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        match variant {
            DIVA_A::DIVA_0 => 0,
            DIVA_A::DIVA_1 => 1,
            DIVA_A::DIVA_2 => 2,
            DIVA_A::DIVA_3 => 3,
        }
    }
}
#[doc = "Reader of field `DIVA`"]
pub type DIVA_R = crate::R<u8, DIVA_A>;
impl DIVA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVA_A {
        match self.bits {
            0 => DIVA_A::DIVA_0,
            1 => DIVA_A::DIVA_1,
            2 => DIVA_A::DIVA_2,
            3 => DIVA_A::DIVA_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVA_0`"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == DIVA_A::DIVA_0
    }
    #[doc = "Checks if the value of the field is `DIVA_1`"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == DIVA_A::DIVA_1
    }
    #[doc = "Checks if the value of the field is `DIVA_2`"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == DIVA_A::DIVA_2
    }
    #[doc = "Checks if the value of the field is `DIVA_3`"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == DIVA_A::DIVA_3
    }
}
#[doc = "Write proxy for field `DIVA`"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ACLK Divider 0: /1"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_0)
    }
    #[doc = "ACLK Divider 1: /2"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_1)
    }
    #[doc = "ACLK Divider 2: /4"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_2)
    }
    #[doc = "ACLK Divider 3: /8"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `XTS`"]
pub type XTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTS`"]
pub struct XTS_W<'a> {
    w: &'a mut W,
}
impl<'a> XTS_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `XT2OFF`"]
pub type XT2OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT2OFF`"]
pub struct XT2OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> XT2OFF_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel0(&self) -> RSEL0_R {
        RSEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Range Select Bit 1"]
    #[inline(always)]
    pub fn rsel1(&self) -> RSEL1_R {
        RSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Range Select Bit 2"]
    #[inline(always)]
    pub fn rsel2(&self) -> RSEL2_R {
        RSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Range Select Bit 3"]
    #[inline(always)]
    pub fn rsel3(&self) -> RSEL3_R {
        RSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&self) -> XT2OFF_R {
        XT2OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel0(&mut self) -> RSEL0_W {
        RSEL0_W { w: self }
    }
    #[doc = "Bit 1 - Range Select Bit 1"]
    #[inline(always)]
    pub fn rsel1(&mut self) -> RSEL1_W {
        RSEL1_W { w: self }
    }
    #[doc = "Bit 2 - Range Select Bit 2"]
    #[inline(always)]
    pub fn rsel2(&mut self) -> RSEL2_W {
        RSEL2_W { w: self }
    }
    #[doc = "Bit 3 - Range Select Bit 3"]
    #[inline(always)]
    pub fn rsel3(&mut self) -> RSEL3_W {
        RSEL3_W { w: self }
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&mut self) -> XTS_W {
        XTS_W { w: self }
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&mut self) -> XT2OFF_W {
        XT2OFF_W { w: self }
    }
}
