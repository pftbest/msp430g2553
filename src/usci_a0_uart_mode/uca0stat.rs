#[doc = "Reader of register UCA0STAT"]
pub type R = crate::R<u8, super::UCA0STAT>;
#[doc = "Writer for register UCA0STAT"]
pub type W = crate::W<u8, super::UCA0STAT>;
#[doc = "Register UCA0STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0STAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCBUSY`"]
pub type UCBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBUSY`"]
pub struct UCBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBUSY_W<'a> {
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
#[doc = "Reader of field `UCADDR`"]
pub type UCADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDR`"]
pub struct UCADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDR_W<'a> {
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
#[doc = "Reader of field `UCRXERR`"]
pub type UCRXERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCRXERR`"]
pub struct UCRXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXERR_W<'a> {
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
#[doc = "Reader of field `UCBRK`"]
pub type UCBRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRK`"]
pub struct UCBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRK_W<'a> {
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
#[doc = "Reader of field `UCPE`"]
pub type UCPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCPE`"]
pub struct UCPE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPE_W<'a> {
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
#[doc = "Reader of field `UCOE`"]
pub type UCOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOE`"]
pub struct UCOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOE_W<'a> {
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
#[doc = "Reader of field `UCFE`"]
pub type UCFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCFE`"]
pub struct UCFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCFE_W<'a> {
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
#[doc = "Reader of field `UCLISTEN`"]
pub type UCLISTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCLISTEN`"]
pub struct UCLISTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCLISTEN_W<'a> {
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
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UCBUSY_R {
        UCBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Address received Flag"]
    #[inline(always)]
    pub fn ucaddr(&self) -> UCADDR_R {
        UCADDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USCI RX Error Flag"]
    #[inline(always)]
    pub fn ucrxerr(&self) -> UCRXERR_R {
        UCRXERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USCI Break received"]
    #[inline(always)]
    pub fn ucbrk(&self) -> UCBRK_R {
        UCBRK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USCI Parity Error Flag"]
    #[inline(always)]
    pub fn ucpe(&self) -> UCPE_R {
        UCPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UCOE_R {
        UCOE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UCFE_R {
        UCFE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&mut self) -> UCBUSY_W {
        UCBUSY_W { w: self }
    }
    #[doc = "Bit 1 - USCI Address received Flag"]
    #[inline(always)]
    pub fn ucaddr(&mut self) -> UCADDR_W {
        UCADDR_W { w: self }
    }
    #[doc = "Bit 2 - USCI RX Error Flag"]
    #[inline(always)]
    pub fn ucrxerr(&mut self) -> UCRXERR_W {
        UCRXERR_W { w: self }
    }
    #[doc = "Bit 3 - USCI Break received"]
    #[inline(always)]
    pub fn ucbrk(&mut self) -> UCBRK_W {
        UCBRK_W { w: self }
    }
    #[doc = "Bit 4 - USCI Parity Error Flag"]
    #[inline(always)]
    pub fn ucpe(&mut self) -> UCPE_W {
        UCPE_W { w: self }
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    pub fn ucoe(&mut self) -> UCOE_W {
        UCOE_W { w: self }
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    pub fn ucfe(&mut self) -> UCFE_W {
        UCFE_W { w: self }
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UCLISTEN_W {
        UCLISTEN_W { w: self }
    }
}
