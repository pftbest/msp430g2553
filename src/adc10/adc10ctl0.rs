#[doc = "Register `ADC10CTL0` reader"]
pub struct R(crate::R<ADC10CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC10CTL0_SPEC>> for R {
    fn from(reader: crate::R<ADC10CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10CTL0` writer"]
pub struct W(crate::W<ADC10CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<ADC10CTL0_SPEC>> for W {
    fn from(writer: crate::W<ADC10CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10SC` reader - ADC10 Start Conversion"]
pub struct ADC10SC_R(crate::FieldReader<bool, bool>);
impl ADC10SC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC10SC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10SC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10SC` writer - ADC10 Start Conversion"]
pub struct ADC10SC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10SC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `ENC` reader - ADC10 Enable Conversion"]
pub struct ENC_R(crate::FieldReader<bool, bool>);
impl ENC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENC` writer - ADC10 Enable Conversion"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ADC10IFG` reader - ADC10 Interrupt Flag"]
pub struct ADC10IFG_R(crate::FieldReader<bool, bool>);
impl ADC10IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC10IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10IFG` writer - ADC10 Interrupt Flag"]
pub struct ADC10IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10IFG_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ADC10IE` reader - ADC10 Interrupt Enalbe"]
pub struct ADC10IE_R(crate::FieldReader<bool, bool>);
impl ADC10IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC10IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10IE` writer - ADC10 Interrupt Enalbe"]
pub struct ADC10IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10IE_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ADC10ON` reader - ADC10 On/Enable"]
pub struct ADC10ON_R(crate::FieldReader<bool, bool>);
impl ADC10ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC10ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10ON` writer - ADC10 On/Enable"]
pub struct ADC10ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10ON_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `REFON` reader - ADC10 Reference on"]
pub struct REFON_R(crate::FieldReader<bool, bool>);
impl REFON_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFON` writer - ADC10 Reference on"]
pub struct REFON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFON_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `REF2_5V` reader - ADC10 Ref 0:1.5V / 1:2.5V"]
pub struct REF2_5V_R(crate::FieldReader<bool, bool>);
impl REF2_5V_R {
    pub(crate) fn new(bits: bool) -> Self {
        REF2_5V_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF2_5V_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF2_5V` writer - ADC10 Ref 0:1.5V / 1:2.5V"]
pub struct REF2_5V_W<'a> {
    w: &'a mut W,
}
impl<'a> REF2_5V_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `MSC` reader - ADC10 Multiple SampleConversion"]
pub struct MSC_R(crate::FieldReader<bool, bool>);
impl MSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSC` writer - ADC10 Multiple SampleConversion"]
pub struct MSC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSC_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `REFBURST` reader - ADC10 Reference Burst Mode"]
pub struct REFBURST_R(crate::FieldReader<bool, bool>);
impl REFBURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFBURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFBURST` writer - ADC10 Reference Burst Mode"]
pub struct REFBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBURST_W<'a> {
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
            (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `REFOUT` reader - ADC10 Enalbe output of Ref."]
pub struct REFOUT_R(crate::FieldReader<bool, bool>);
impl REFOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFOUT` writer - ADC10 Enalbe output of Ref."]
pub struct REFOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOUT_W<'a> {
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
            (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ADC10SR` reader - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
pub struct ADC10SR_R(crate::FieldReader<bool, bool>);
impl ADC10SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC10SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10SR` writer - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
pub struct ADC10SR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10SR_W<'a> {
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
            (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "ADC10 Sample Hold Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10SHT_A {
    #[doc = "0: 4 x ADC10CLKs"]
    ADC10SHT_0 = 0,
    #[doc = "1: 8 x ADC10CLKs"]
    ADC10SHT_1 = 1,
    #[doc = "2: 16 x ADC10CLKs"]
    ADC10SHT_2 = 2,
    #[doc = "3: 64 x ADC10CLKs"]
    ADC10SHT_3 = 3,
}
impl From<ADC10SHT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10SHT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10SHT` reader - ADC10 Sample Hold Select Bit: 0"]
pub struct ADC10SHT_R(crate::FieldReader<u8, ADC10SHT_A>);
impl ADC10SHT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC10SHT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10SHT_A {
        match self.bits {
            0 => ADC10SHT_A::ADC10SHT_0,
            1 => ADC10SHT_A::ADC10SHT_1,
            2 => ADC10SHT_A::ADC10SHT_2,
            3 => ADC10SHT_A::ADC10SHT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_0`"]
    #[inline(always)]
    pub fn is_adc10sht_0(&self) -> bool {
        **self == ADC10SHT_A::ADC10SHT_0
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_1`"]
    #[inline(always)]
    pub fn is_adc10sht_1(&self) -> bool {
        **self == ADC10SHT_A::ADC10SHT_1
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_2`"]
    #[inline(always)]
    pub fn is_adc10sht_2(&self) -> bool {
        **self == ADC10SHT_A::ADC10SHT_2
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_3`"]
    #[inline(always)]
    pub fn is_adc10sht_3(&self) -> bool {
        **self == ADC10SHT_A::ADC10SHT_3
    }
}
impl core::ops::Deref for ADC10SHT_R {
    type Target = crate::FieldReader<u8, ADC10SHT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10SHT` writer - ADC10 Sample Hold Select Bit: 0"]
pub struct ADC10SHT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10SHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC10SHT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "4 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_0(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_0)
    }
    #[doc = "8 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_1(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_1)
    }
    #[doc = "16 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_2(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_2)
    }
    #[doc = "64 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_3(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 11)) | ((value as u16 & 0x03) << 11);
        self.w
    }
}
#[doc = "ADC10 Reference Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SREF_A {
    #[doc = "0: VR+ = AVCC and VR- = AVSS"]
    SREF_0 = 0,
    #[doc = "1: VR+ = VREF+ and VR- = AVSS"]
    SREF_1 = 1,
    #[doc = "2: VR+ = VEREF+ and VR- = AVSS"]
    SREF_2 = 2,
    #[doc = "3: VR+ = VEREF+ and VR- = AVSS"]
    SREF_3 = 3,
    #[doc = "4: VR+ = AVCC and VR- = VREF-/VEREF-"]
    SREF_4 = 4,
    #[doc = "5: VR+ = VREF+ and VR- = VREF-/VEREF-"]
    SREF_5 = 5,
    #[doc = "6: VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    SREF_6 = 6,
    #[doc = "7: VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    SREF_7 = 7,
}
impl From<SREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SREF` reader - ADC10 Reference Select Bit: 0"]
pub struct SREF_R(crate::FieldReader<u8, SREF_A>);
impl SREF_R {
    pub(crate) fn new(bits: u8) -> Self {
        SREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREF_A {
        match self.bits {
            0 => SREF_A::SREF_0,
            1 => SREF_A::SREF_1,
            2 => SREF_A::SREF_2,
            3 => SREF_A::SREF_3,
            4 => SREF_A::SREF_4,
            5 => SREF_A::SREF_5,
            6 => SREF_A::SREF_6,
            7 => SREF_A::SREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SREF_0`"]
    #[inline(always)]
    pub fn is_sref_0(&self) -> bool {
        **self == SREF_A::SREF_0
    }
    #[doc = "Checks if the value of the field is `SREF_1`"]
    #[inline(always)]
    pub fn is_sref_1(&self) -> bool {
        **self == SREF_A::SREF_1
    }
    #[doc = "Checks if the value of the field is `SREF_2`"]
    #[inline(always)]
    pub fn is_sref_2(&self) -> bool {
        **self == SREF_A::SREF_2
    }
    #[doc = "Checks if the value of the field is `SREF_3`"]
    #[inline(always)]
    pub fn is_sref_3(&self) -> bool {
        **self == SREF_A::SREF_3
    }
    #[doc = "Checks if the value of the field is `SREF_4`"]
    #[inline(always)]
    pub fn is_sref_4(&self) -> bool {
        **self == SREF_A::SREF_4
    }
    #[doc = "Checks if the value of the field is `SREF_5`"]
    #[inline(always)]
    pub fn is_sref_5(&self) -> bool {
        **self == SREF_A::SREF_5
    }
    #[doc = "Checks if the value of the field is `SREF_6`"]
    #[inline(always)]
    pub fn is_sref_6(&self) -> bool {
        **self == SREF_A::SREF_6
    }
    #[doc = "Checks if the value of the field is `SREF_7`"]
    #[inline(always)]
    pub fn is_sref_7(&self) -> bool {
        **self == SREF_A::SREF_7
    }
}
impl core::ops::Deref for SREF_R {
    type Target = crate::FieldReader<u8, SREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SREF` writer - ADC10 Reference Select Bit: 0"]
pub struct SREF_W<'a> {
    w: &'a mut W,
}
impl<'a> SREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SREF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "VR+ = AVCC and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_0(self) -> &'a mut W {
        self.variant(SREF_A::SREF_0)
    }
    #[doc = "VR+ = VREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_1(self) -> &'a mut W {
        self.variant(SREF_A::SREF_1)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_2(self) -> &'a mut W {
        self.variant(SREF_A::SREF_2)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_3(self) -> &'a mut W {
        self.variant(SREF_A::SREF_3)
    }
    #[doc = "VR+ = AVCC and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_4(self) -> &'a mut W {
        self.variant(SREF_A::SREF_4)
    }
    #[doc = "VR+ = VREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_5(self) -> &'a mut W {
        self.variant(SREF_A::SREF_5)
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_6(self) -> &'a mut W {
        self.variant(SREF_A::SREF_6)
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_7(self) -> &'a mut W {
        self.variant(SREF_A::SREF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 13)) | ((value as u16 & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&self) -> ADC10SC_R {
        ADC10SC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg(&self) -> ADC10IFG_R {
        ADC10IFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC10 Interrupt Enalbe"]
    #[inline(always)]
    pub fn adc10ie(&self) -> ADC10IE_R {
        ADC10IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC10 On/Enable"]
    #[inline(always)]
    pub fn adc10on(&self) -> ADC10ON_R {
        ADC10ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC10 Reference on"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC10 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&self) -> REF2_5V_R {
        REF2_5V_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC10 Reference Burst Mode"]
    #[inline(always)]
    pub fn refburst(&self) -> REFBURST_R {
        REFBURST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC10 Enalbe output of Ref."]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
    #[inline(always)]
    pub fn adc10sr(&self) -> ADC10SR_R {
        ADC10SR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&self) -> ADC10SHT_R {
        ADC10SHT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - ADC10 Reference Select Bit: 0"]
    #[inline(always)]
    pub fn sref(&self) -> SREF_R {
        SREF_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&mut self) -> ADC10SC_W {
        ADC10SC_W { w: self }
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bit 2 - ADC10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg(&mut self) -> ADC10IFG_W {
        ADC10IFG_W { w: self }
    }
    #[doc = "Bit 3 - ADC10 Interrupt Enalbe"]
    #[inline(always)]
    pub fn adc10ie(&mut self) -> ADC10IE_W {
        ADC10IE_W { w: self }
    }
    #[doc = "Bit 4 - ADC10 On/Enable"]
    #[inline(always)]
    pub fn adc10on(&mut self) -> ADC10ON_W {
        ADC10ON_W { w: self }
    }
    #[doc = "Bit 5 - ADC10 Reference on"]
    #[inline(always)]
    pub fn refon(&mut self) -> REFON_W {
        REFON_W { w: self }
    }
    #[doc = "Bit 6 - ADC10 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&mut self) -> REF2_5V_W {
        REF2_5V_W { w: self }
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&mut self) -> MSC_W {
        MSC_W { w: self }
    }
    #[doc = "Bit 8 - ADC10 Reference Burst Mode"]
    #[inline(always)]
    pub fn refburst(&mut self) -> REFBURST_W {
        REFBURST_W { w: self }
    }
    #[doc = "Bit 9 - ADC10 Enalbe output of Ref."]
    #[inline(always)]
    pub fn refout(&mut self) -> REFOUT_W {
        REFOUT_W { w: self }
    }
    #[doc = "Bit 10 - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
    #[inline(always)]
    pub fn adc10sr(&mut self) -> ADC10SR_W {
        ADC10SR_W { w: self }
    }
    #[doc = "Bits 11:12 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&mut self) -> ADC10SHT_W {
        ADC10SHT_W { w: self }
    }
    #[doc = "Bits 13:15 - ADC10 Reference Select Bit: 0"]
    #[inline(always)]
    pub fn sref(&mut self) -> SREF_W {
        SREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ctl0](index.html) module"]
pub struct ADC10CTL0_SPEC;
impl crate::RegisterSpec for ADC10CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10ctl0::R](R) reader structure"]
impl crate::Readable for ADC10CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10ctl0::W](W) writer structure"]
impl crate::Writable for ADC10CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10CTL0 to value 0"]
impl crate::Resettable for ADC10CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
