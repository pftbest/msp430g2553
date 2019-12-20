#[doc = "Reader of register IFG1"]
pub type R = crate::R<u8, super::IFG1>;
#[doc = "Writer for register IFG1"]
pub type W = crate::W<u8, super::IFG1>;
#[doc = "Register IFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IFG1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTIFG`"]
pub type WDTIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTIFG`"]
pub struct WDTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIFG_W<'a> {
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
#[doc = "Reader of field `OFIFG`"]
pub type OFIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFIFG`"]
pub struct OFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> OFIFG_W<'a> {
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
#[doc = "Reader of field `PORIFG`"]
pub type PORIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORIFG`"]
pub struct PORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PORIFG_W<'a> {
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
#[doc = "Reader of field `RSTIFG`"]
pub type RSTIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIFG`"]
pub struct RSTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIFG_W<'a> {
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
#[doc = "Reader of field `NMIIFG`"]
pub type NMIIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIIFG`"]
pub struct NMIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIIFG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&self) -> PORIFG_R {
        PORIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&self) -> RSTIFG_R {
        RSTIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WDTIFG_W {
        WDTIFG_W { w: self }
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OFIFG_W {
        OFIFG_W { w: self }
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&mut self) -> PORIFG_W {
        PORIFG_W { w: self }
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&mut self) -> RSTIFG_W {
        RSTIFG_W { w: self }
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NMIIFG_W {
        NMIIFG_W { w: self }
    }
}
