#[doc = "Reader of register UCA0IRTCTL"]
pub type R = crate::R<u8, super::UCA0IRTCTL>;
#[doc = "Writer for register UCA0IRTCTL"]
pub type W = crate::W<u8, super::UCA0IRTCTL>;
#[doc = "Register UCA0IRTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0IRTCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCIREN`"]
pub type UCIREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIREN`"]
pub struct UCIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIREN_W<'a> {
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
#[doc = "Reader of field `UCIRTXCLK`"]
pub type UCIRTXCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRTXCLK`"]
pub struct UCIRTXCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXCLK_W<'a> {
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
#[doc = "Reader of field `UCIRTXPL0`"]
pub type UCIRTXPL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRTXPL0`"]
pub struct UCIRTXPL0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL0_W<'a> {
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
#[doc = "Reader of field `UCIRTXPL1`"]
pub type UCIRTXPL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRTXPL1`"]
pub struct UCIRTXPL1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL1_W<'a> {
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
#[doc = "Reader of field `UCIRTXPL2`"]
pub type UCIRTXPL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRTXPL2`"]
pub struct UCIRTXPL2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL2_W<'a> {
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
#[doc = "Reader of field `UCIRTXPL3`"]
pub type UCIRTXPL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRTXPL3`"]
pub struct UCIRTXPL3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL3_W<'a> {
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
#[doc = "Reader of field `UCIRTXPL4`"]
pub type UCIRTXPL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRTXPL4`"]
pub struct UCIRTXPL4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL4_W<'a> {
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
#[doc = "Reader of field `UCIRTXPL5`"]
pub type UCIRTXPL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCIRTXPL5`"]
pub struct UCIRTXPL5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL5_W<'a> {
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
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl0(&self) -> UCIRTXPL0_R {
        UCIRTXPL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IRDA Transmit Pulse Length 1"]
    #[inline(always)]
    pub fn ucirtxpl1(&self) -> UCIRTXPL1_R {
        UCIRTXPL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IRDA Transmit Pulse Length 2"]
    #[inline(always)]
    pub fn ucirtxpl2(&self) -> UCIRTXPL2_R {
        UCIRTXPL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IRDA Transmit Pulse Length 3"]
    #[inline(always)]
    pub fn ucirtxpl3(&self) -> UCIRTXPL3_R {
        UCIRTXPL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IRDA Transmit Pulse Length 4"]
    #[inline(always)]
    pub fn ucirtxpl4(&self) -> UCIRTXPL4_R {
        UCIRTXPL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IRDA Transmit Pulse Length 5"]
    #[inline(always)]
    pub fn ucirtxpl5(&self) -> UCIRTXPL5_R {
        UCIRTXPL5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UCIREN_W {
        UCIREN_W { w: self }
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W {
        UCIRTXCLK_W { w: self }
    }
    #[doc = "Bit 2 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl0(&mut self) -> UCIRTXPL0_W {
        UCIRTXPL0_W { w: self }
    }
    #[doc = "Bit 3 - IRDA Transmit Pulse Length 1"]
    #[inline(always)]
    pub fn ucirtxpl1(&mut self) -> UCIRTXPL1_W {
        UCIRTXPL1_W { w: self }
    }
    #[doc = "Bit 4 - IRDA Transmit Pulse Length 2"]
    #[inline(always)]
    pub fn ucirtxpl2(&mut self) -> UCIRTXPL2_W {
        UCIRTXPL2_W { w: self }
    }
    #[doc = "Bit 5 - IRDA Transmit Pulse Length 3"]
    #[inline(always)]
    pub fn ucirtxpl3(&mut self) -> UCIRTXPL3_W {
        UCIRTXPL3_W { w: self }
    }
    #[doc = "Bit 6 - IRDA Transmit Pulse Length 4"]
    #[inline(always)]
    pub fn ucirtxpl4(&mut self) -> UCIRTXPL4_W {
        UCIRTXPL4_W { w: self }
    }
    #[doc = "Bit 7 - IRDA Transmit Pulse Length 5"]
    #[inline(always)]
    pub fn ucirtxpl5(&mut self) -> UCIRTXPL5_W {
        UCIRTXPL5_W { w: self }
    }
}
