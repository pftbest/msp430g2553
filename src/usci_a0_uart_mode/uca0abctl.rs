#[doc = "Reader of register UCA0ABCTL"]
pub type R = crate::R<u8, super::UCA0ABCTL>;
#[doc = "Writer for register UCA0ABCTL"]
pub type W = crate::W<u8, super::UCA0ABCTL>;
#[doc = "Register UCA0ABCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0ABCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCABDEN`"]
pub type UCABDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCABDEN`"]
pub struct UCABDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCABDEN_W<'a> {
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
#[doc = "Reader of field `UCBTOE`"]
pub type UCBTOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBTOE`"]
pub struct UCBTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBTOE_W<'a> {
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
#[doc = "Reader of field `UCSTOE`"]
pub type UCSTOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSTOE`"]
pub struct UCSTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTOE_W<'a> {
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
#[doc = "Reader of field `UCDELIM`"]
pub type UCDELIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCDELIM`"]
pub struct UCDELIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UCDELIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UCABDEN_R {
        UCABDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UCBTOE_R {
        UCBTOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UCSTOE_R {
        UCSTOE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim(&self) -> UCDELIM_R {
        UCDELIM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&mut self) -> UCABDEN_W {
        UCABDEN_W { w: self }
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&mut self) -> UCBTOE_W {
        UCBTOE_W { w: self }
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&mut self) -> UCSTOE_W {
        UCSTOE_W { w: self }
    }
    #[doc = "Bits 4:5 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim(&mut self) -> UCDELIM_W {
        UCDELIM_W { w: self }
    }
}
