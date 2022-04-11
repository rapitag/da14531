#[doc = "Register `BIST_CTRL_REG` reader"]
pub struct R(crate::R<BIST_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_CTRL_REG` writer"]
pub struct W(crate::W<BIST_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_CTRL_REG_SPEC>;
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
impl From<crate::W<BIST_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSRAM3_BIST_ENABLE` reader - "]
pub struct SYSRAM3_BIST_ENABLE_R(crate::FieldReader<bool, bool>);
impl SYSRAM3_BIST_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM3_BIST_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRAM3_BIST_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM3_BIST_ENABLE` writer - "]
pub struct SYSRAM3_BIST_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM3_BIST_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u16 & 1) << 14);
        self.w
    }
}
#[doc = "Field `RAM_BIST_PATTERN` reader - "]
pub struct RAM_BIST_PATTERN_R(crate::FieldReader<u8, u8>);
impl RAM_BIST_PATTERN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM_BIST_PATTERN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_BIST_PATTERN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_BIST_PATTERN` writer - "]
pub struct RAM_BIST_PATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_BIST_PATTERN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u16 & 3) << 12);
        self.w
    }
}
#[doc = "Field `SYSRAM12_BIST_BUSY` reader - "]
pub struct SYSRAM12_BIST_BUSY_R(crate::FieldReader<bool, bool>);
impl SYSRAM12_BIST_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM12_BIST_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRAM12_BIST_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM12_BIST_FAIL` reader - "]
pub struct SYSRAM12_BIST_FAIL_R(crate::FieldReader<bool, bool>);
impl SYSRAM12_BIST_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM12_BIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRAM12_BIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM3_BIST_BUSY` reader - "]
pub struct SYSRAM3_BIST_BUSY_R(crate::FieldReader<bool, bool>);
impl SYSRAM3_BIST_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM3_BIST_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRAM3_BIST_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM3_BIST_FAIL` reader - "]
pub struct SYSRAM3_BIST_FAIL_R(crate::FieldReader<bool, bool>);
impl SYSRAM3_BIST_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM3_BIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRAM3_BIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_BIST_BUSY` reader - "]
pub struct ROM_BIST_BUSY_R(crate::FieldReader<bool, bool>);
impl ROM_BIST_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_BIST_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_BIST_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM12_BIST_ENABLE` reader - "]
pub struct SYSRAM12_BIST_ENABLE_R(crate::FieldReader<bool, bool>);
impl SYSRAM12_BIST_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM12_BIST_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRAM12_BIST_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM12_BIST_ENABLE` writer - "]
pub struct SYSRAM12_BIST_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM12_BIST_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `ROMBIST_ENABLE` reader - "]
pub struct ROMBIST_ENABLE_R(crate::FieldReader<bool, bool>);
impl ROMBIST_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROMBIST_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMBIST_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROMBIST_ENABLE` writer - "]
pub struct ROMBIST_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMBIST_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RAM_BIST_CONFIG` reader - "]
pub struct RAM_BIST_CONFIG_R(crate::FieldReader<u8, u8>);
impl RAM_BIST_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM_BIST_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_BIST_CONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_BIST_CONFIG` writer - "]
pub struct RAM_BIST_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_BIST_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sysram3_bist_enable(&self) -> SYSRAM3_BIST_ENABLE_R {
        SYSRAM3_BIST_ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ram_bist_pattern(&self) -> RAM_BIST_PATTERN_R {
        RAM_BIST_PATTERN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sysram12_bist_busy(&self) -> SYSRAM12_BIST_BUSY_R {
        SYSRAM12_BIST_BUSY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sysram12_bist_fail(&self) -> SYSRAM12_BIST_FAIL_R {
        SYSRAM12_BIST_FAIL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sysram3_bist_busy(&self) -> SYSRAM3_BIST_BUSY_R {
        SYSRAM3_BIST_BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sysram3_bist_fail(&self) -> SYSRAM3_BIST_FAIL_R {
        SYSRAM3_BIST_FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom_bist_busy(&self) -> ROM_BIST_BUSY_R {
        ROM_BIST_BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sysram12_bist_enable(&self) -> SYSRAM12_BIST_ENABLE_R {
        SYSRAM12_BIST_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rombist_enable(&self) -> ROMBIST_ENABLE_R {
        ROMBIST_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ram_bist_config(&self) -> RAM_BIST_CONFIG_R {
        RAM_BIST_CONFIG_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sysram3_bist_enable(&mut self) -> SYSRAM3_BIST_ENABLE_W {
        SYSRAM3_BIST_ENABLE_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ram_bist_pattern(&mut self) -> RAM_BIST_PATTERN_W {
        RAM_BIST_PATTERN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sysram12_bist_enable(&mut self) -> SYSRAM12_BIST_ENABLE_W {
        SYSRAM12_BIST_ENABLE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rombist_enable(&mut self) -> ROMBIST_ENABLE_W {
        ROMBIST_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ram_bist_config(&mut self) -> RAM_BIST_CONFIG_W {
        RAM_BIST_CONFIG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_ctrl_reg](index.html) module"]
pub struct BIST_CTRL_REG_SPEC;
impl crate::RegisterSpec for BIST_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bist_ctrl_reg::R](R) reader structure"]
impl crate::Readable for BIST_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_ctrl_reg::W](W) writer structure"]
impl crate::Writable for BIST_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_CTRL_REG to value 0x0480"]
impl crate::Resettable for BIST_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0480
    }
}
