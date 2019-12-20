#[doc = "Reader of register FCTL1"]
pub type R = crate::R<u16, super::FCTL1>;
#[doc = "Writer for register FCTL1"]
pub type W = crate::W<u16, super::FCTL1>;
#[doc = "Register FCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE`"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
#[doc = "Reader of field `MERAS`"]
pub type MERAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MERAS`"]
pub struct MERAS_W<'a> {
    w: &'a mut W,
}
impl<'a> MERAS_W<'a> {
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
#[doc = "Reader of field `WRT`"]
pub type WRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT`"]
pub struct WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_W<'a> {
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
#[doc = "Reader of field `BLKWRT`"]
pub type BLKWRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKWRT`"]
pub struct BLKWRT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKWRT_W<'a> {
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
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&self) -> MERAS_R {
        MERAS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&self) -> WRT_R {
        WRT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&self) -> BLKWRT_R {
        BLKWRT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&mut self) -> MERAS_W {
        MERAS_W { w: self }
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&mut self) -> WRT_W {
        WRT_W { w: self }
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&mut self) -> BLKWRT_W {
        BLKWRT_W { w: self }
    }
}
