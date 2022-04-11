#[doc = "Register `BLE_DIAGCNTL_REG` reader"]
pub struct R(crate::R<BLE_DIAGCNTL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_DIAGCNTL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_DIAGCNTL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_DIAGCNTL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_DIAGCNTL_REG` writer"]
pub struct W(crate::W<BLE_DIAGCNTL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_DIAGCNTL_REG_SPEC>;
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
impl From<crate::W<BLE_DIAGCNTL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_DIAGCNTL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAG3_EN` reader - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG3_EN_R(crate::FieldReader<bool, bool>);
impl DIAG3_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG3_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG3_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG3_EN` writer - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG3_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Field `DIAG3` reader - Only relevant when DIAG3_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG3."]
pub struct DIAG3_R(crate::FieldReader<u8, u8>);
impl DIAG3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG3` writer - Only relevant when DIAG3_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG3."]
pub struct DIAG3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `DIAG2_EN` reader - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG2_EN_R(crate::FieldReader<bool, bool>);
impl DIAG2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG2_EN` writer - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `DIAG2` reader - Only relevant when DIAG2_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG2."]
pub struct DIAG2_R(crate::FieldReader<u8, u8>);
impl DIAG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG2` writer - Only relevant when DIAG2_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG2."]
pub struct DIAG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `DIAG1_EN` reader - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG1_EN_R(crate::FieldReader<bool, bool>);
impl DIAG1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG1_EN` writer - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `DIAG1` reader - Only relevant when DIAG1_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG1."]
pub struct DIAG1_R(crate::FieldReader<u8, u8>);
impl DIAG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG1` writer - Only relevant when DIAG1_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG1."]
pub struct DIAG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `DIAG0_EN` reader - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG0_EN_R(crate::FieldReader<bool, bool>);
impl DIAG0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG0_EN` writer - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `DIAG0` reader - Only relevant when DIAG0_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG0."]
pub struct DIAG0_R(crate::FieldReader<u8, u8>);
impl DIAG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG0` writer - Only relevant when DIAG0_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG0."]
pub struct DIAG0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag3_en(&self) -> DIAG3_EN_R {
        DIAG3_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Only relevant when DIAG3_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG3."]
    #[inline(always)]
    pub fn diag3(&self) -> DIAG3_R {
        DIAG3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag2_en(&self) -> DIAG2_EN_R {
        DIAG2_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Only relevant when DIAG2_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG2."]
    #[inline(always)]
    pub fn diag2(&self) -> DIAG2_R {
        DIAG2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag1_en(&self) -> DIAG1_EN_R {
        DIAG1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Only relevant when DIAG1_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG1."]
    #[inline(always)]
    pub fn diag1(&self) -> DIAG1_R {
        DIAG1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag0_en(&self) -> DIAG0_EN_R {
        DIAG0_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Only relevant when DIAG0_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG0."]
    #[inline(always)]
    pub fn diag0(&self) -> DIAG0_R {
        DIAG0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag3_en(&mut self) -> DIAG3_EN_W {
        DIAG3_EN_W { w: self }
    }
    #[doc = "Bits 24:29 - Only relevant when DIAG3_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG3."]
    #[inline(always)]
    pub fn diag3(&mut self) -> DIAG3_W {
        DIAG3_W { w: self }
    }
    #[doc = "Bit 23 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag2_en(&mut self) -> DIAG2_EN_W {
        DIAG2_EN_W { w: self }
    }
    #[doc = "Bits 16:21 - Only relevant when DIAG2_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG2."]
    #[inline(always)]
    pub fn diag2(&mut self) -> DIAG2_W {
        DIAG2_W { w: self }
    }
    #[doc = "Bit 15 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag1_en(&mut self) -> DIAG1_EN_W {
        DIAG1_EN_W { w: self }
    }
    #[doc = "Bits 8:13 - Only relevant when DIAG1_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG1."]
    #[inline(always)]
    pub fn diag1(&mut self) -> DIAG1_W {
        DIAG1_W { w: self }
    }
    #[doc = "Bit 7 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag0_en(&mut self) -> DIAG0_EN_W {
        DIAG0_EN_W { w: self }
    }
    #[doc = "Bits 0:5 - Only relevant when DIAG0_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG0."]
    #[inline(always)]
    pub fn diag0(&mut self) -> DIAG0_W {
        DIAG0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Diagnostics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_diagcntl_reg](index.html) module"]
pub struct BLE_DIAGCNTL_REG_SPEC;
impl crate::RegisterSpec for BLE_DIAGCNTL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_diagcntl_reg::R](R) reader structure"]
impl crate::Readable for BLE_DIAGCNTL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_diagcntl_reg::W](W) writer structure"]
impl crate::Writable for BLE_DIAGCNTL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_DIAGCNTL_REG to value 0"]
impl crate::Resettable for BLE_DIAGCNTL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
