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
#[doc = "Reader of field `UCOA0`"]
pub type UCOA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA0`"]
pub struct UCOA0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA0_W<'a> {
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
#[doc = "Reader of field `UCOA1`"]
pub type UCOA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA1`"]
pub struct UCOA1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA1_W<'a> {
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
#[doc = "Reader of field `UCOA2`"]
pub type UCOA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA2`"]
pub struct UCOA2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA2_W<'a> {
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
#[doc = "Reader of field `UCOA3`"]
pub type UCOA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA3`"]
pub struct UCOA3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA3_W<'a> {
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
#[doc = "Reader of field `UCOA4`"]
pub type UCOA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA4`"]
pub struct UCOA4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA4_W<'a> {
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
#[doc = "Reader of field `UCOA5`"]
pub type UCOA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA5`"]
pub struct UCOA5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA5_W<'a> {
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
#[doc = "Reader of field `UCOA6`"]
pub type UCOA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA6`"]
pub struct UCOA6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA6_W<'a> {
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
#[doc = "Reader of field `UCOA7`"]
pub type UCOA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA7`"]
pub struct UCOA7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA7_W<'a> {
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
#[doc = "Reader of field `UCOA8`"]
pub type UCOA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA8`"]
pub struct UCOA8_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA8_W<'a> {
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
#[doc = "Reader of field `UCOA9`"]
pub type UCOA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOA9`"]
pub struct UCOA9_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA9_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa0(&self) -> UCOA0_R {
        UCOA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Own Address 1"]
    #[inline(always)]
    pub fn ucoa1(&self) -> UCOA1_R {
        UCOA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Own Address 2"]
    #[inline(always)]
    pub fn ucoa2(&self) -> UCOA2_R {
        UCOA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Own Address 3"]
    #[inline(always)]
    pub fn ucoa3(&self) -> UCOA3_R {
        UCOA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Own Address 4"]
    #[inline(always)]
    pub fn ucoa4(&self) -> UCOA4_R {
        UCOA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Own Address 5"]
    #[inline(always)]
    pub fn ucoa5(&self) -> UCOA5_R {
        UCOA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Own Address 6"]
    #[inline(always)]
    pub fn ucoa6(&self) -> UCOA6_R {
        UCOA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Own Address 7"]
    #[inline(always)]
    pub fn ucoa7(&self) -> UCOA7_R {
        UCOA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Own Address 8"]
    #[inline(always)]
    pub fn ucoa8(&self) -> UCOA8_R {
        UCOA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Own Address 9"]
    #[inline(always)]
    pub fn ucoa9(&self) -> UCOA9_R {
        UCOA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UCGCEN_R {
        UCGCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa0(&mut self) -> UCOA0_W {
        UCOA0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Own Address 1"]
    #[inline(always)]
    pub fn ucoa1(&mut self) -> UCOA1_W {
        UCOA1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Own Address 2"]
    #[inline(always)]
    pub fn ucoa2(&mut self) -> UCOA2_W {
        UCOA2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Own Address 3"]
    #[inline(always)]
    pub fn ucoa3(&mut self) -> UCOA3_W {
        UCOA3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Own Address 4"]
    #[inline(always)]
    pub fn ucoa4(&mut self) -> UCOA4_W {
        UCOA4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Own Address 5"]
    #[inline(always)]
    pub fn ucoa5(&mut self) -> UCOA5_W {
        UCOA5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Own Address 6"]
    #[inline(always)]
    pub fn ucoa6(&mut self) -> UCOA6_W {
        UCOA6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Own Address 7"]
    #[inline(always)]
    pub fn ucoa7(&mut self) -> UCOA7_W {
        UCOA7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Own Address 8"]
    #[inline(always)]
    pub fn ucoa8(&mut self) -> UCOA8_W {
        UCOA8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Own Address 9"]
    #[inline(always)]
    pub fn ucoa9(&mut self) -> UCOA9_W {
        UCOA9_W { w: self }
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UCGCEN_W {
        UCGCEN_W { w: self }
    }
}
