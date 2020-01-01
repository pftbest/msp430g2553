#[doc = "Reader of register WDTCTL"]
pub type R = crate::R<u16, super::WDTCTL>;
#[doc = "Writer for register WDTCTL"]
pub type W = crate::W<u16, super::WDTCTL>;
#[doc = "Register WDTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTIS0`"]
pub type WDTIS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTIS0`"]
pub struct WDTIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIS0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `WDTIS1`"]
pub type WDTIS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTIS1`"]
pub struct WDTIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIS1_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `WDTSSEL`"]
pub type WDTSSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTSSEL`"]
pub struct WDTSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSSEL_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `WDTCNTCL`"]
pub type WDTCNTCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTCNTCL`"]
pub struct WDTCNTCL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCNTCL_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WDTTMSEL`"]
pub type WDTTMSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTTMSEL`"]
pub struct WDTTMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTTMSEL_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `WDTNMI`"]
pub type WDTNMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTNMI`"]
pub struct WDTNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTNMI_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `WDTNMIES`"]
pub type WDTNMIES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTNMIES`"]
pub struct WDTNMIES_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTNMIES_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `WDTHOLD`"]
pub type WDTHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTHOLD`"]
pub struct WDTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTHOLD_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis0(&self) -> WDTIS0_R {
        WDTIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WDTIS1"]
    #[inline(always)]
    pub fn wdtis1(&self) -> WDTIS1_R {
        WDTIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&self) -> WDTNMI_R {
        WDTNMI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&self) -> WDTNMIES_R {
        WDTNMIES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis0(&mut self) -> WDTIS0_W {
        WDTIS0_W { w: self }
    }
    #[doc = "Bit 1 - WDTIS1"]
    #[inline(always)]
    pub fn wdtis1(&mut self) -> WDTIS1_W {
        WDTIS1_W { w: self }
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W {
        WDTSSEL_W { w: self }
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W {
        WDTCNTCL_W { w: self }
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W {
        WDTTMSEL_W { w: self }
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&mut self) -> WDTNMI_W {
        WDTNMI_W { w: self }
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&mut self) -> WDTNMIES_W {
        WDTNMIES_W { w: self }
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W {
        WDTHOLD_W { w: self }
    }
}
