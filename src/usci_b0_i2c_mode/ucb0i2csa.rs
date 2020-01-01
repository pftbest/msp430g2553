#[doc = "Reader of register UCB0I2CSA"]
pub type R = crate::R<u16, super::UCB0I2CSA>;
#[doc = "Writer for register UCB0I2CSA"]
pub type W = crate::W<u16, super::UCB0I2CSA>;
#[doc = "Register UCB0I2CSA `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0I2CSA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCSA0`"]
pub type UCSA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA0`"]
pub struct UCSA0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA0_W<'a> {
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
#[doc = "Reader of field `UCSA1`"]
pub type UCSA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA1`"]
pub struct UCSA1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA1_W<'a> {
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
#[doc = "Reader of field `UCSA2`"]
pub type UCSA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA2`"]
pub struct UCSA2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA2_W<'a> {
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
#[doc = "Reader of field `UCSA3`"]
pub type UCSA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA3`"]
pub struct UCSA3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA3_W<'a> {
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
#[doc = "Reader of field `UCSA4`"]
pub type UCSA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA4`"]
pub struct UCSA4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA4_W<'a> {
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
#[doc = "Reader of field `UCSA5`"]
pub type UCSA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA5`"]
pub struct UCSA5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA5_W<'a> {
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
#[doc = "Reader of field `UCSA6`"]
pub type UCSA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA6`"]
pub struct UCSA6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA6_W<'a> {
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
#[doc = "Reader of field `UCSA7`"]
pub type UCSA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA7`"]
pub struct UCSA7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA7_W<'a> {
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
#[doc = "Reader of field `UCSA8`"]
pub type UCSA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA8`"]
pub struct UCSA8_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA8_W<'a> {
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
            (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `UCSA9`"]
pub type UCSA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSA9`"]
pub struct UCSA9_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA9_W<'a> {
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
            (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa0(&self) -> UCSA0_R {
        UCSA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Slave Address 1"]
    #[inline(always)]
    pub fn ucsa1(&self) -> UCSA1_R {
        UCSA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Slave Address 2"]
    #[inline(always)]
    pub fn ucsa2(&self) -> UCSA2_R {
        UCSA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Slave Address 3"]
    #[inline(always)]
    pub fn ucsa3(&self) -> UCSA3_R {
        UCSA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Slave Address 4"]
    #[inline(always)]
    pub fn ucsa4(&self) -> UCSA4_R {
        UCSA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Slave Address 5"]
    #[inline(always)]
    pub fn ucsa5(&self) -> UCSA5_R {
        UCSA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Slave Address 6"]
    #[inline(always)]
    pub fn ucsa6(&self) -> UCSA6_R {
        UCSA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Slave Address 7"]
    #[inline(always)]
    pub fn ucsa7(&self) -> UCSA7_R {
        UCSA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Slave Address 8"]
    #[inline(always)]
    pub fn ucsa8(&self) -> UCSA8_R {
        UCSA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Slave Address 9"]
    #[inline(always)]
    pub fn ucsa9(&self) -> UCSA9_R {
        UCSA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa0(&mut self) -> UCSA0_W {
        UCSA0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Slave Address 1"]
    #[inline(always)]
    pub fn ucsa1(&mut self) -> UCSA1_W {
        UCSA1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Slave Address 2"]
    #[inline(always)]
    pub fn ucsa2(&mut self) -> UCSA2_W {
        UCSA2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Slave Address 3"]
    #[inline(always)]
    pub fn ucsa3(&mut self) -> UCSA3_W {
        UCSA3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Slave Address 4"]
    #[inline(always)]
    pub fn ucsa4(&mut self) -> UCSA4_W {
        UCSA4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Slave Address 5"]
    #[inline(always)]
    pub fn ucsa5(&mut self) -> UCSA5_W {
        UCSA5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Slave Address 6"]
    #[inline(always)]
    pub fn ucsa6(&mut self) -> UCSA6_W {
        UCSA6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Slave Address 7"]
    #[inline(always)]
    pub fn ucsa7(&mut self) -> UCSA7_W {
        UCSA7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Slave Address 8"]
    #[inline(always)]
    pub fn ucsa8(&mut self) -> UCSA8_W {
        UCSA8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Slave Address 9"]
    #[inline(always)]
    pub fn ucsa9(&mut self) -> UCSA9_W {
        UCSA9_W { w: self }
    }
}
