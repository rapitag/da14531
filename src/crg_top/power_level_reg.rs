#[doc = "Register `POWER_LEVEL_REG` reader"]
pub struct R(crate::R<POWER_LEVEL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_LEVEL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_LEVEL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_LEVEL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_LEVEL_REG` writer"]
pub struct W(crate::W<POWER_LEVEL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_LEVEL_REG_SPEC>;
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
impl From<crate::W<POWER_LEVEL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_LEVEL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDC_TRIM` reader - Delta from DCDC_LEVEL nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV"]
pub struct DCDC_TRIM_R(crate::FieldReader<u8, u8>);
impl DCDC_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCDC_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_TRIM` writer - Delta from DCDC_LEVEL nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV"]
pub struct DCDC_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 11)) | ((value as u16 & 7) << 11);
        self.w
    }
}
#[doc = "Field `DCDC_LEVEL` reader - 00: 1.1 V 01: 1.8 V (default) 10: 2.5 V 11: 3.0 V"]
pub struct DCDC_LEVEL_R(crate::FieldReader<u8, u8>);
impl DCDC_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCDC_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_LEVEL` writer - 00: 1.1 V 01: 1.8 V (default) 10: 2.5 V 11: 3.0 V"]
pub struct DCDC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 9)) | ((value as u16 & 3) << 9);
        self.w
    }
}
#[doc = "Field `LDO_CORE_RET_CUR_TRIM` reader - "]
pub struct LDO_CORE_RET_CUR_TRIM_R(crate::FieldReader<u8, u8>);
impl LDO_CORE_RET_CUR_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_CORE_RET_CUR_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_CORE_RET_CUR_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_CORE_RET_CUR_TRIM` writer - "]
pub struct LDO_CORE_RET_CUR_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_CORE_RET_CUR_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 7)) | ((value as u16 & 3) << 7);
        self.w
    }
}
#[doc = "Field `LDO_XTAL_TRIM` reader - Delta from 0.9 V nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV"]
pub struct LDO_XTAL_TRIM_R(crate::FieldReader<u8, u8>);
impl LDO_XTAL_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_XTAL_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_XTAL_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_XTAL_TRIM` writer - Delta from 0.9 V nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV"]
pub struct LDO_XTAL_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_XTAL_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u16 & 7) << 4);
        self.w
    }
}
#[doc = "Field `LDO_LOW_TRIM` reader - Delta from 1.1 V nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV (coldboot)"]
pub struct LDO_LOW_TRIM_R(crate::FieldReader<u8, u8>);
impl LDO_LOW_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_LOW_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_LOW_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_LOW_TRIM` writer - Delta from 1.1 V nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV (coldboot)"]
pub struct LDO_LOW_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_LOW_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 1)) | ((value as u16 & 7) << 1);
        self.w
    }
}
#[doc = "Field `LDO_CORE_LEVEL` reader - "]
pub struct LDO_CORE_LEVEL_R(crate::FieldReader<bool, bool>);
impl LDO_CORE_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_CORE_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_CORE_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_CORE_LEVEL` writer - "]
pub struct LDO_CORE_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_CORE_LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:13 - Delta from DCDC_LEVEL nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV"]
    #[inline(always)]
    pub fn dcdc_trim(&self) -> DCDC_TRIM_R {
        DCDC_TRIM_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 9:10 - 00: 1.1 V 01: 1.8 V (default) 10: 2.5 V 11: 3.0 V"]
    #[inline(always)]
    pub fn dcdc_level(&self) -> DCDC_LEVEL_R {
        DCDC_LEVEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn ldo_core_ret_cur_trim(&self) -> LDO_CORE_RET_CUR_TRIM_R {
        LDO_CORE_RET_CUR_TRIM_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Delta from 0.9 V nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV"]
    #[inline(always)]
    pub fn ldo_xtal_trim(&self) -> LDO_XTAL_TRIM_R {
        LDO_XTAL_TRIM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 1:3 - Delta from 1.1 V nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV (coldboot)"]
    #[inline(always)]
    pub fn ldo_low_trim(&self) -> LDO_LOW_TRIM_R {
        LDO_LOW_TRIM_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ldo_core_level(&self) -> LDO_CORE_LEVEL_R {
        LDO_CORE_LEVEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 11:13 - Delta from DCDC_LEVEL nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV"]
    #[inline(always)]
    pub fn dcdc_trim(&mut self) -> DCDC_TRIM_W {
        DCDC_TRIM_W { w: self }
    }
    #[doc = "Bits 9:10 - 00: 1.1 V 01: 1.8 V (default) 10: 2.5 V 11: 3.0 V"]
    #[inline(always)]
    pub fn dcdc_level(&mut self) -> DCDC_LEVEL_W {
        DCDC_LEVEL_W { w: self }
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn ldo_core_ret_cur_trim(&mut self) -> LDO_CORE_RET_CUR_TRIM_W {
        LDO_CORE_RET_CUR_TRIM_W { w: self }
    }
    #[doc = "Bits 4:6 - Delta from 0.9 V nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV"]
    #[inline(always)]
    pub fn ldo_xtal_trim(&mut self) -> LDO_XTAL_TRIM_W {
        LDO_XTAL_TRIM_W { w: self }
    }
    #[doc = "Bits 1:3 - Delta from 1.1 V nominal value 000: -75 mV 001: -50 mV 010: -25 mV 011: 0 (default) 100: +25 mV 101: +50 mV 110: +75 mV 111: +100 mV (coldboot)"]
    #[inline(always)]
    pub fn ldo_low_trim(&mut self) -> LDO_LOW_TRIM_W {
        LDO_LOW_TRIM_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ldo_core_level(&mut self) -> LDO_CORE_LEVEL_W {
        LDO_CORE_LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power management level and trim settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_level_reg](index.html) module"]
pub struct POWER_LEVEL_REG_SPEC;
impl crate::RegisterSpec for POWER_LEVEL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [power_level_reg::R](R) reader structure"]
impl crate::Readable for POWER_LEVEL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_level_reg::W](W) writer structure"]
impl crate::Writable for POWER_LEVEL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWER_LEVEL_REG to value 0x1a3e"]
impl crate::Resettable for POWER_LEVEL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1a3e
    }
}
