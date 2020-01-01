#[doc = "Reader of register UCA0IRRCTL"]
pub type R = crate::R<u8, super::UCA0IRRCTL>;
#[doc = "Writer for register UCA0IRRCTL"]
pub type W = crate::W<u8, super::UCA0IRRCTL>;
#[doc = "Register UCA0IRRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0IRRCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCIRRXFE`"]
pub type UCIRRXFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRRXFE`"]
pub struct UCIRRXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFE_W<'a> {
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
#[doc = "Reader of field `UCIRRXPL`"]
pub type UCIRRXPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRRXPL`"]
pub struct UCIRRXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXPL_W<'a> {
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
#[doc = "Reader of field `UCIRRXFL0`"]
pub type UCIRRXFL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRRXFL0`"]
pub struct UCIRRXFL0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL0_W<'a> {
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
#[doc = "Reader of field `UCIRRXFL1`"]
pub type UCIRRXFL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRRXFL1`"]
pub struct UCIRRXFL1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL1_W<'a> {
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
#[doc = "Reader of field `UCIRRXFL2`"]
pub type UCIRRXFL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRRXFL2`"]
pub struct UCIRRXFL2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL2_W<'a> {
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
#[doc = "Reader of field `UCIRRXFL3`"]
pub type UCIRRXFL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRRXFL3`"]
pub struct UCIRRXFL3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL3_W<'a> {
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
#[doc = "Reader of field `UCIRRXFL4`"]
pub type UCIRRXFL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRRXFL4`"]
pub struct UCIRRXFL4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL4_W<'a> {
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
#[doc = "Reader of field `UCIRRXFL5`"]
pub type UCIRRXFL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRRXFL5`"]
pub struct UCIRRXFL5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL5_W<'a> {
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
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl0(&self) -> UCIRRXFL0_R {
        UCIRRXFL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    pub fn ucirrxfl1(&self) -> UCIRRXFL1_R {
        UCIRRXFL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    pub fn ucirrxfl2(&self) -> UCIRRXFL2_R {
        UCIRRXFL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    pub fn ucirrxfl3(&self) -> UCIRRXFL3_R {
        UCIRRXFL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    pub fn ucirrxfl4(&self) -> UCIRRXFL4_R {
        UCIRRXFL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    pub fn ucirrxfl5(&self) -> UCIRRXFL5_R {
        UCIRRXFL5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W {
        UCIRRXFE_W { w: self }
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W {
        UCIRRXPL_W { w: self }
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl0(&mut self) -> UCIRRXFL0_W {
        UCIRRXFL0_W { w: self }
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    pub fn ucirrxfl1(&mut self) -> UCIRRXFL1_W {
        UCIRRXFL1_W { w: self }
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    pub fn ucirrxfl2(&mut self) -> UCIRRXFL2_W {
        UCIRRXFL2_W { w: self }
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    pub fn ucirrxfl3(&mut self) -> UCIRRXFL3_W {
        UCIRRXFL3_W { w: self }
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    pub fn ucirrxfl4(&mut self) -> UCIRRXFL4_W {
        UCIRRXFL4_W { w: self }
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    pub fn ucirrxfl5(&mut self) -> UCIRRXFL5_W {
        UCIRRXFL5_W { w: self }
    }
}
