#[doc = "Register `BLE_DIAGCNTL3_REG` reader"]
pub struct R(crate::R<BLE_DIAGCNTL3_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_DIAGCNTL3_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_DIAGCNTL3_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_DIAGCNTL3_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_DIAGCNTL3_REG` writer"]
pub struct W(crate::W<BLE_DIAGCNTL3_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_DIAGCNTL3_REG_SPEC>;
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
impl From<crate::W<BLE_DIAGCNTL3_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_DIAGCNTL3_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAG7_INV` reader - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG7_INV_R(crate::FieldReader<bool, bool>);
impl DIAG7_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG7_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG7_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG7_INV` writer - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG7_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG7_INV_W<'a> {
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
#[doc = "Field `DIAG7_BIT` reader - Selects which bit from the DIAG7 word will be forwarded to bit 7 of the BLE DIagnostic Port."]
pub struct DIAG7_BIT_R(crate::FieldReader<u8, u8>);
impl DIAG7_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG7_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG7_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG7_BIT` writer - Selects which bit from the DIAG7 word will be forwarded to bit 7 of the BLE DIagnostic Port."]
pub struct DIAG7_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG7_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 28)) | ((value as u32 & 7) << 28);
        self.w
    }
}
#[doc = "Field `DIAG6_INV` reader - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG6_INV_R(crate::FieldReader<bool, bool>);
impl DIAG6_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG6_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG6_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG6_INV` writer - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG6_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG6_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `DIAG6_BIT` reader - Selects which bit from the DIAG6 word will be forwarded to bit 6 of the BLE DIagnostic Port."]
pub struct DIAG6_BIT_R(crate::FieldReader<u8, u8>);
impl DIAG6_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG6_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG6_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG6_BIT` writer - Selects which bit from the DIAG6 word will be forwarded to bit 6 of the BLE DIagnostic Port."]
pub struct DIAG6_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG6_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
#[doc = "Field `DIAG5_INV` reader - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG5_INV_R(crate::FieldReader<bool, bool>);
impl DIAG5_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG5_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG5_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG5_INV` writer - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG5_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG5_INV_W<'a> {
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
#[doc = "Field `DIAG5_BIT` reader - Selects which bit from the DIAG5 word will be forwarded to bit 5 of the BLE DIagnostic Port."]
pub struct DIAG5_BIT_R(crate::FieldReader<u8, u8>);
impl DIAG5_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG5_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG5_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG5_BIT` writer - Selects which bit from the DIAG5 word will be forwarded to bit 5 of the BLE DIagnostic Port."]
pub struct DIAG5_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG5_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 20)) | ((value as u32 & 7) << 20);
        self.w
    }
}
#[doc = "Field `DIAG4_INV` reader - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG4_INV_R(crate::FieldReader<bool, bool>);
impl DIAG4_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG4_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG4_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG4_INV` writer - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG4_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG4_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `DIAG4_BIT` reader - Selects which bit from the DIAG4 word will be forwarded to bit 4 of the BLE DIagnostic Port."]
pub struct DIAG4_BIT_R(crate::FieldReader<u8, u8>);
impl DIAG4_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG4_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG4_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG4_BIT` writer - Selects which bit from the DIAG4 word will be forwarded to bit 4 of the BLE DIagnostic Port."]
pub struct DIAG4_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG4_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 16)) | ((value as u32 & 7) << 16);
        self.w
    }
}
#[doc = "Field `DIAG3_INV` reader - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG3_INV_R(crate::FieldReader<bool, bool>);
impl DIAG3_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG3_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG3_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG3_INV` writer - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG3_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG3_INV_W<'a> {
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
#[doc = "Field `DIAG3_BIT` reader - Selects which bit from the DIAG3 word will be forwarded to bit 3 of the BLE DIagnostic Port."]
pub struct DIAG3_BIT_R(crate::FieldReader<u8, u8>);
impl DIAG3_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG3_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG3_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG3_BIT` writer - Selects which bit from the DIAG3 word will be forwarded to bit 3 of the BLE DIagnostic Port."]
pub struct DIAG3_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG3_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 12)) | ((value as u32 & 7) << 12);
        self.w
    }
}
#[doc = "Field `DIAG2_INV` reader - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG2_INV_R(crate::FieldReader<bool, bool>);
impl DIAG2_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG2_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG2_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG2_INV` writer - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG2_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG2_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `DIAG2_BIT` reader - Selects which bit from the DIAG2 word will be forwarded to bit 2 of the BLE DIagnostic Port."]
pub struct DIAG2_BIT_R(crate::FieldReader<u8, u8>);
impl DIAG2_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG2_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG2_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG2_BIT` writer - Selects which bit from the DIAG2 word will be forwarded to bit 2 of the BLE DIagnostic Port."]
pub struct DIAG2_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG2_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `DIAG1_INV` reader - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG1_INV_R(crate::FieldReader<bool, bool>);
impl DIAG1_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG1_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG1_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG1_INV` writer - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG1_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG1_INV_W<'a> {
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
#[doc = "Field `DIAG1_BIT` reader - Selects which bit from the DIAG1 word will be forwarded to bit 1 of the BLE DIagnostic Port."]
pub struct DIAG1_BIT_R(crate::FieldReader<u8, u8>);
impl DIAG1_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG1_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG1_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG1_BIT` writer - Selects which bit from the DIAG1 word will be forwarded to bit 1 of the BLE DIagnostic Port."]
pub struct DIAG1_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG1_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `DIAG0_INV` reader - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG0_INV_R(crate::FieldReader<bool, bool>);
impl DIAG0_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG0_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG0_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG0_INV` writer - If set, then the specific diagnostic bit will be inverted."]
pub struct DIAG0_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG0_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `DIAG0_BIT` reader - Selects which bit from the DIAG0 word will be forwarded to bit 0 of the BLE DIagnostic Port."]
pub struct DIAG0_BIT_R(crate::FieldReader<u8, u8>);
impl DIAG0_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG0_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG0_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG0_BIT` writer - Selects which bit from the DIAG0 word will be forwarded to bit 0 of the BLE DIagnostic Port."]
pub struct DIAG0_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG0_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag7_inv(&self) -> DIAG7_INV_R {
        DIAG7_INV_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Selects which bit from the DIAG7 word will be forwarded to bit 7 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag7_bit(&self) -> DIAG7_BIT_R {
        DIAG7_BIT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 27 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag6_inv(&self) -> DIAG6_INV_R {
        DIAG6_INV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Selects which bit from the DIAG6 word will be forwarded to bit 6 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag6_bit(&self) -> DIAG6_BIT_R {
        DIAG6_BIT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 23 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag5_inv(&self) -> DIAG5_INV_R {
        DIAG5_INV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Selects which bit from the DIAG5 word will be forwarded to bit 5 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag5_bit(&self) -> DIAG5_BIT_R {
        DIAG5_BIT_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 19 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag4_inv(&self) -> DIAG4_INV_R {
        DIAG4_INV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Selects which bit from the DIAG4 word will be forwarded to bit 4 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag4_bit(&self) -> DIAG4_BIT_R {
        DIAG4_BIT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 15 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag3_inv(&self) -> DIAG3_INV_R {
        DIAG3_INV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Selects which bit from the DIAG3 word will be forwarded to bit 3 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag3_bit(&self) -> DIAG3_BIT_R {
        DIAG3_BIT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag2_inv(&self) -> DIAG2_INV_R {
        DIAG2_INV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Selects which bit from the DIAG2 word will be forwarded to bit 2 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag2_bit(&self) -> DIAG2_BIT_R {
        DIAG2_BIT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 7 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag1_inv(&self) -> DIAG1_INV_R {
        DIAG1_INV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Selects which bit from the DIAG1 word will be forwarded to bit 1 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag1_bit(&self) -> DIAG1_BIT_R {
        DIAG1_BIT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag0_inv(&self) -> DIAG0_INV_R {
        DIAG0_INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:2 - Selects which bit from the DIAG0 word will be forwarded to bit 0 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag0_bit(&self) -> DIAG0_BIT_R {
        DIAG0_BIT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag7_inv(&mut self) -> DIAG7_INV_W {
        DIAG7_INV_W { w: self }
    }
    #[doc = "Bits 28:30 - Selects which bit from the DIAG7 word will be forwarded to bit 7 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag7_bit(&mut self) -> DIAG7_BIT_W {
        DIAG7_BIT_W { w: self }
    }
    #[doc = "Bit 27 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag6_inv(&mut self) -> DIAG6_INV_W {
        DIAG6_INV_W { w: self }
    }
    #[doc = "Bits 24:26 - Selects which bit from the DIAG6 word will be forwarded to bit 6 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag6_bit(&mut self) -> DIAG6_BIT_W {
        DIAG6_BIT_W { w: self }
    }
    #[doc = "Bit 23 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag5_inv(&mut self) -> DIAG5_INV_W {
        DIAG5_INV_W { w: self }
    }
    #[doc = "Bits 20:22 - Selects which bit from the DIAG5 word will be forwarded to bit 5 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag5_bit(&mut self) -> DIAG5_BIT_W {
        DIAG5_BIT_W { w: self }
    }
    #[doc = "Bit 19 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag4_inv(&mut self) -> DIAG4_INV_W {
        DIAG4_INV_W { w: self }
    }
    #[doc = "Bits 16:18 - Selects which bit from the DIAG4 word will be forwarded to bit 4 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag4_bit(&mut self) -> DIAG4_BIT_W {
        DIAG4_BIT_W { w: self }
    }
    #[doc = "Bit 15 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag3_inv(&mut self) -> DIAG3_INV_W {
        DIAG3_INV_W { w: self }
    }
    #[doc = "Bits 12:14 - Selects which bit from the DIAG3 word will be forwarded to bit 3 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag3_bit(&mut self) -> DIAG3_BIT_W {
        DIAG3_BIT_W { w: self }
    }
    #[doc = "Bit 11 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag2_inv(&mut self) -> DIAG2_INV_W {
        DIAG2_INV_W { w: self }
    }
    #[doc = "Bits 8:10 - Selects which bit from the DIAG2 word will be forwarded to bit 2 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag2_bit(&mut self) -> DIAG2_BIT_W {
        DIAG2_BIT_W { w: self }
    }
    #[doc = "Bit 7 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag1_inv(&mut self) -> DIAG1_INV_W {
        DIAG1_INV_W { w: self }
    }
    #[doc = "Bits 4:6 - Selects which bit from the DIAG1 word will be forwarded to bit 1 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag1_bit(&mut self) -> DIAG1_BIT_W {
        DIAG1_BIT_W { w: self }
    }
    #[doc = "Bit 3 - If set, then the specific diagnostic bit will be inverted."]
    #[inline(always)]
    pub fn diag0_inv(&mut self) -> DIAG0_INV_W {
        DIAG0_INV_W { w: self }
    }
    #[doc = "Bits 0:2 - Selects which bit from the DIAG0 word will be forwarded to bit 0 of the BLE DIagnostic Port."]
    #[inline(always)]
    pub fn diag0_bit(&mut self) -> DIAG0_BIT_W {
        DIAG0_BIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug use only\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_diagcntl3_reg](index.html) module"]
pub struct BLE_DIAGCNTL3_REG_SPEC;
impl crate::RegisterSpec for BLE_DIAGCNTL3_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_diagcntl3_reg::R](R) reader structure"]
impl crate::Readable for BLE_DIAGCNTL3_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_diagcntl3_reg::W](W) writer structure"]
impl crate::Writable for BLE_DIAGCNTL3_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_DIAGCNTL3_REG to value 0"]
impl crate::Resettable for BLE_DIAGCNTL3_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
