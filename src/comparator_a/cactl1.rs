#[doc = "Reader of register CACTL1"]
pub type R = crate::R<u8, super::CACTL1>;
#[doc = "Writer for register CACTL1"]
pub type W = crate::W<u8, super::CACTL1>;
#[doc = "Register CACTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CACTL1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAIFG`"]
pub type CAIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAIFG`"]
pub struct CAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CAIFG_W<'a> {
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
#[doc = "Reader of field `CAIE`"]
pub type CAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAIE`"]
pub struct CAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAIE_W<'a> {
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
#[doc = "Reader of field `CAIES`"]
pub type CAIES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAIES`"]
pub struct CAIES_W<'a> {
    w: &'a mut W,
}
impl<'a> CAIES_W<'a> {
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
#[doc = "Reader of field `CAON`"]
pub type CAON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAON`"]
pub struct CAON_W<'a> {
    w: &'a mut W,
}
impl<'a> CAON_W<'a> {
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
#[doc = "Comp. A Internal Reference Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAREF_A {
    #[doc = "0: Comp. A Int. Ref. Select 0 : Off"]
    CAREF_0,
    #[doc = "1: Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    CAREF_1,
    #[doc = "2: Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    CAREF_2,
    #[doc = "3: Comp. A Int. Ref. Select 3 : Vt"]
    CAREF_3,
}
impl From<CAREF_A> for u8 {
    #[inline(always)]
    fn from(variant: CAREF_A) -> Self {
        match variant {
            CAREF_A::CAREF_0 => 0,
            CAREF_A::CAREF_1 => 1,
            CAREF_A::CAREF_2 => 2,
            CAREF_A::CAREF_3 => 3,
        }
    }
}
#[doc = "Reader of field `CAREF`"]
pub type CAREF_R = crate::R<u8, CAREF_A>;
impl CAREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAREF_A {
        match self.bits {
            0 => CAREF_A::CAREF_0,
            1 => CAREF_A::CAREF_1,
            2 => CAREF_A::CAREF_2,
            3 => CAREF_A::CAREF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAREF_0`"]
    #[inline(always)]
    pub fn is_caref_0(&self) -> bool {
        *self == CAREF_A::CAREF_0
    }
    #[doc = "Checks if the value of the field is `CAREF_1`"]
    #[inline(always)]
    pub fn is_caref_1(&self) -> bool {
        *self == CAREF_A::CAREF_1
    }
    #[doc = "Checks if the value of the field is `CAREF_2`"]
    #[inline(always)]
    pub fn is_caref_2(&self) -> bool {
        *self == CAREF_A::CAREF_2
    }
    #[doc = "Checks if the value of the field is `CAREF_3`"]
    #[inline(always)]
    pub fn is_caref_3(&self) -> bool {
        *self == CAREF_A::CAREF_3
    }
}
#[doc = "Write proxy for field `CAREF`"]
pub struct CAREF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAREF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. A Int. Ref. Select 0 : Off"]
    #[inline(always)]
    pub fn caref_0(self) -> &'a mut W {
        self.variant(CAREF_A::CAREF_0)
    }
    #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    #[inline(always)]
    pub fn caref_1(self) -> &'a mut W {
        self.variant(CAREF_A::CAREF_1)
    }
    #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    #[inline(always)]
    pub fn caref_2(self) -> &'a mut W {
        self.variant(CAREF_A::CAREF_2)
    }
    #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
    #[inline(always)]
    pub fn caref_3(self) -> &'a mut W {
        self.variant(CAREF_A::CAREF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CARSEL`"]
pub type CARSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARSEL`"]
pub struct CARSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARSEL_W<'a> {
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
#[doc = "Reader of field `CAEX`"]
pub type CAEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAEX`"]
pub struct CAEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEX_W<'a> {
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
    #[doc = "Bit 0 - Comp. A Interrupt Flag"]
    #[inline(always)]
    pub fn caifg(&self) -> CAIFG_R {
        CAIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. A Interrupt Enable"]
    #[inline(always)]
    pub fn caie(&self) -> CAIE_R {
        CAIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
    #[inline(always)]
    pub fn caies(&self) -> CAIES_R {
        CAIES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comp. A enable"]
    #[inline(always)]
    pub fn caon(&self) -> CAON_R {
        CAON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
    #[inline(always)]
    pub fn caref(&self) -> CAREF_R {
        CAREF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
    #[inline(always)]
    pub fn carsel(&self) -> CARSEL_R {
        CARSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comp. A Exchange Inputs"]
    #[inline(always)]
    pub fn caex(&self) -> CAEX_R {
        CAEX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Interrupt Flag"]
    #[inline(always)]
    pub fn caifg(&mut self) -> CAIFG_W {
        CAIFG_W { w: self }
    }
    #[doc = "Bit 1 - Comp. A Interrupt Enable"]
    #[inline(always)]
    pub fn caie(&mut self) -> CAIE_W {
        CAIE_W { w: self }
    }
    #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
    #[inline(always)]
    pub fn caies(&mut self) -> CAIES_W {
        CAIES_W { w: self }
    }
    #[doc = "Bit 3 - Comp. A enable"]
    #[inline(always)]
    pub fn caon(&mut self) -> CAON_W {
        CAON_W { w: self }
    }
    #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
    #[inline(always)]
    pub fn caref(&mut self) -> CAREF_W {
        CAREF_W { w: self }
    }
    #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
    #[inline(always)]
    pub fn carsel(&mut self) -> CARSEL_W {
        CARSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comp. A Exchange Inputs"]
    #[inline(always)]
    pub fn caex(&mut self) -> CAEX_W {
        CAEX_W { w: self }
    }
}
