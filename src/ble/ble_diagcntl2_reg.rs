#[doc = "Register `BLE_DIAGCNTL2_REG` reader"]
pub struct R(crate::R<BLE_DIAGCNTL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_DIAGCNTL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_DIAGCNTL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_DIAGCNTL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_DIAGCNTL2_REG` writer"]
pub struct W(crate::W<BLE_DIAGCNTL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_DIAGCNTL2_REG_SPEC>;
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
impl From<crate::W<BLE_DIAGCNTL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_DIAGCNTL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAG7_EN` reader - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG7_EN_R(crate::FieldReader<bool, bool>);
impl DIAG7_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG7_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG7_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG7_EN` writer - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG7_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG7_EN_W<'a> {
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
#[doc = "Field `DIAG7` reader - Only relevant when DIAG7_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG7."]
pub struct DIAG7_R(crate::FieldReader<u8, u8>);
impl DIAG7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG7` writer - Only relevant when DIAG7_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG7."]
pub struct DIAG7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `DIAG6_EN` reader - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG6_EN_R(crate::FieldReader<bool, bool>);
impl DIAG6_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG6_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG6_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG6_EN` writer - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG6_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG6_EN_W<'a> {
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
#[doc = "Field `DIAG6` reader - Only relevant when DIAG6_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG6."]
pub struct DIAG6_R(crate::FieldReader<u8, u8>);
impl DIAG6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG6` writer - Only relevant when DIAG6_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG6."]
pub struct DIAG6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `DIAG5_EN` reader - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG5_EN_R(crate::FieldReader<bool, bool>);
impl DIAG5_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG5_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG5_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG5_EN` writer - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG5_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG5_EN_W<'a> {
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
#[doc = "Field `DIAG5` reader - Only relevant when DIAG5_EN= 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG5."]
pub struct DIAG5_R(crate::FieldReader<u8, u8>);
impl DIAG5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG5` writer - Only relevant when DIAG5_EN= 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG5."]
pub struct DIAG5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `DIAG4_EN` reader - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG4_EN_R(crate::FieldReader<bool, bool>);
impl DIAG4_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG4_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG4_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG4_EN` writer - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
pub struct DIAG4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG4_EN_W<'a> {
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
#[doc = "Field `DIAG4` reader - Only relevant when DIAG4_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG4."]
pub struct DIAG4_R(crate::FieldReader<u8, u8>);
impl DIAG4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG4` writer - Only relevant when DIAG4_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG4."]
pub struct DIAG4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG4_W<'a> {
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
    pub fn diag7_en(&self) -> DIAG7_EN_R {
        DIAG7_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Only relevant when DIAG7_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG7."]
    #[inline(always)]
    pub fn diag7(&self) -> DIAG7_R {
        DIAG7_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag6_en(&self) -> DIAG6_EN_R {
        DIAG6_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Only relevant when DIAG6_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG6."]
    #[inline(always)]
    pub fn diag6(&self) -> DIAG6_R {
        DIAG6_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag5_en(&self) -> DIAG5_EN_R {
        DIAG5_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Only relevant when DIAG5_EN= 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG5."]
    #[inline(always)]
    pub fn diag5(&self) -> DIAG5_R {
        DIAG5_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag4_en(&self) -> DIAG4_EN_R {
        DIAG4_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Only relevant when DIAG4_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG4."]
    #[inline(always)]
    pub fn diag4(&self) -> DIAG4_R {
        DIAG4_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag7_en(&mut self) -> DIAG7_EN_W {
        DIAG7_EN_W { w: self }
    }
    #[doc = "Bits 24:29 - Only relevant when DIAG7_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG7."]
    #[inline(always)]
    pub fn diag7(&mut self) -> DIAG7_W {
        DIAG7_W { w: self }
    }
    #[doc = "Bit 23 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag6_en(&mut self) -> DIAG6_EN_W {
        DIAG6_EN_W { w: self }
    }
    #[doc = "Bits 16:21 - Only relevant when DIAG6_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG6."]
    #[inline(always)]
    pub fn diag6(&mut self) -> DIAG6_W {
        DIAG6_W { w: self }
    }
    #[doc = "Bit 15 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag5_en(&mut self) -> DIAG5_EN_W {
        DIAG5_EN_W { w: self }
    }
    #[doc = "Bits 8:13 - Only relevant when DIAG5_EN= 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG5."]
    #[inline(always)]
    pub fn diag5(&mut self) -> DIAG5_W {
        DIAG5_W { w: self }
    }
    #[doc = "Bit 7 - 0: Disable diagnostic port 0 output. All outputs are set to 0x0. 1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag4_en(&mut self) -> DIAG4_EN_W {
        DIAG4_EN_W { w: self }
    }
    #[doc = "Bits 0:5 - Only relevant when DIAG4_EN = 1. Selection of the outputs that must be driven to the diagnostic port BLE_DIAG4."]
    #[inline(always)]
    pub fn diag4(&mut self) -> DIAG4_W {
        DIAG4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug use only\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_diagcntl2_reg](index.html) module"]
pub struct BLE_DIAGCNTL2_REG_SPEC;
impl crate::RegisterSpec for BLE_DIAGCNTL2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_diagcntl2_reg::R](R) reader structure"]
impl crate::Readable for BLE_DIAGCNTL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_diagcntl2_reg::W](W) writer structure"]
impl crate::Writable for BLE_DIAGCNTL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_DIAGCNTL2_REG to value 0"]
impl crate::Resettable for BLE_DIAGCNTL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
