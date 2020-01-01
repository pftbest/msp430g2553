#[doc = "Reader of register CACTL2"]
pub type R = crate::R<u8, super::CACTL2>;
#[doc = "Writer for register CACTL2"]
pub type W = crate::W<u8, super::CACTL2>;
#[doc = "Register CACTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CACTL2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAOUT`"]
pub type CAOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAOUT`"]
pub struct CAOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAOUT_W<'a> {
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
#[doc = "Reader of field `CAF`"]
pub type CAF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAF`"]
pub struct CAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAF_W<'a> {
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
#[doc = "Reader of field `P2CA0`"]
pub type P2CA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2CA0`"]
pub struct P2CA0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2CA0_W<'a> {
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
#[doc = "Reader of field `P2CA1`"]
pub type P2CA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2CA1`"]
pub struct P2CA1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2CA1_W<'a> {
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
#[doc = "Reader of field `P2CA2`"]
pub type P2CA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2CA2`"]
pub struct P2CA2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2CA2_W<'a> {
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
#[doc = "Reader of field `P2CA3`"]
pub type P2CA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2CA3`"]
pub struct P2CA3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2CA3_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `P2CA4`"]
pub type P2CA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2CA4`"]
pub struct P2CA4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2CA4_W<'a> {
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
#[doc = "Reader of field `CASHORT`"]
pub type CASHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CASHORT`"]
pub struct CASHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CASHORT_W<'a> {
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
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&self) -> CAOUT_R {
        CAOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca0(&self) -> P2CA0_R {
        P2CA0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca1(&self) -> P2CA1_R {
        P2CA1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca2(&self) -> P2CA2_R {
        P2CA2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca3(&self) -> P2CA3_R {
        P2CA3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca4(&self) -> P2CA4_R {
        P2CA4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&self) -> CASHORT_R {
        CASHORT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&mut self) -> CAOUT_W {
        CAOUT_W { w: self }
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&mut self) -> CAF_W {
        CAF_W { w: self }
    }
    #[doc = "Bit 2 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca0(&mut self) -> P2CA0_W {
        P2CA0_W { w: self }
    }
    #[doc = "Bit 3 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca1(&mut self) -> P2CA1_W {
        P2CA1_W { w: self }
    }
    #[doc = "Bit 4 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca2(&mut self) -> P2CA2_W {
        P2CA2_W { w: self }
    }
    #[doc = "Bit 5 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca3(&mut self) -> P2CA3_W {
        P2CA3_W { w: self }
    }
    #[doc = "Bit 6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca4(&mut self) -> P2CA4_W {
        P2CA4_W { w: self }
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&mut self) -> CASHORT_W {
        CASHORT_W { w: self }
    }
}
