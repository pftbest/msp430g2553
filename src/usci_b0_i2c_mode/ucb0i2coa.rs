#[doc = "Reader of register UCB0I2COA"]
pub type R = crate::R<u16, super::UCB0I2COA>;
#[doc = "Writer for register UCB0I2COA"]
pub type W = crate::W<u16, super::UCB0I2COA>;
#[doc = "Register UCB0I2COA `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0I2COA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCGCEN`"]
pub type UCGCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCGCEN`"]
pub struct UCGCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGCEN_W<'a> {
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
            (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `UCOA`"]
pub type UCOA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UCOA`"]
pub struct UCOA_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UCGCEN_R {
        UCGCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&self) -> UCOA_R {
        UCOA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UCGCEN_W {
        UCGCEN_W { w: self }
    }
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&mut self) -> UCOA_W {
        UCOA_W { w: self }
    }
}
