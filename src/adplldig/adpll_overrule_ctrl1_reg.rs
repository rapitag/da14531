#[doc = "Register `ADPLL_OVERRULE_CTRL1_REG` reader"]
pub struct R(crate::R<ADPLL_OVERRULE_CTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_OVERRULE_CTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_OVERRULE_CTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_OVERRULE_CTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_OVERRULE_CTRL1_REG` writer"]
pub struct W(crate::W<ADPLL_OVERRULE_CTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_OVERRULE_CTRL1_REG_SPEC>;
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
impl From<crate::W<ADPLL_OVERRULE_CTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_OVERRULE_CTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVR_DTC_OH_WR` reader - "]
pub struct OVR_DTC_OH_WR_R(crate::FieldReader<u8, u8>);
impl OVR_DTC_OH_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVR_DTC_OH_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DTC_OH_WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DTC_OH_WR` writer - "]
pub struct OVR_DTC_OH_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DTC_OH_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
#[doc = "Field `OVR_DTC_OH_SEL` reader - "]
pub struct OVR_DTC_OH_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_DTC_OH_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_DTC_OH_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DTC_OH_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DTC_OH_SEL` writer - "]
pub struct OVR_DTC_OH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DTC_OH_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `OVR_DCOAMP_WR` reader - "]
pub struct OVR_DCOAMP_WR_R(crate::FieldReader<u8, u8>);
impl OVR_DCOAMP_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVR_DCOAMP_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOAMP_WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOAMP_WR` writer - "]
pub struct OVR_DCOAMP_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOAMP_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 17)) | ((value as u32 & 0x0f) << 17);
        self.w
    }
}
#[doc = "Field `OVR_DCOAMP_SEL` reader - "]
pub struct OVR_DCOAMP_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_DCOAMP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_DCOAMP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOAMP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOAMP_SEL` writer - "]
pub struct OVR_DCOAMP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOAMP_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `OVR_DCOAMPHOLD_WR` reader - "]
pub struct OVR_DCOAMPHOLD_WR_R(crate::FieldReader<bool, bool>);
impl OVR_DCOAMPHOLD_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_DCOAMPHOLD_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOAMPHOLD_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOAMPHOLD_WR` writer - "]
pub struct OVR_DCOAMPHOLD_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOAMPHOLD_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `OVR_DCOAMPHOLD_SEL` reader - "]
pub struct OVR_DCOAMPHOLD_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_DCOAMPHOLD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_DCOAMPHOLD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOAMPHOLD_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOAMPHOLD_SEL` writer - "]
pub struct OVR_DCOAMPHOLD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOAMPHOLD_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `OVR_RDYFORDIV_WR` reader - "]
pub struct OVR_RDYFORDIV_WR_R(crate::FieldReader<bool, bool>);
impl OVR_RDYFORDIV_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RDYFORDIV_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RDYFORDIV_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RDYFORDIV_WR` writer - "]
pub struct OVR_RDYFORDIV_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_RDYFORDIV_WR_W<'a> {
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
#[doc = "Field `OVR_RDYFORDIV_SEL` reader - "]
pub struct OVR_RDYFORDIV_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_RDYFORDIV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RDYFORDIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RDYFORDIV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RDYFORDIV_SEL` writer - "]
pub struct OVR_RDYFORDIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_RDYFORDIV_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `OVR_VPAEN_WR` reader - "]
pub struct OVR_VPAEN_WR_R(crate::FieldReader<bool, bool>);
impl OVR_VPAEN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_VPAEN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_VPAEN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_VPAEN_WR` writer - "]
pub struct OVR_VPAEN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_VPAEN_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `OVR_VPAEN_SEL` reader - "]
pub struct OVR_VPAEN_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_VPAEN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_VPAEN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_VPAEN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_VPAEN_SEL` writer - "]
pub struct OVR_VPAEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_VPAEN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `OVR_SRESETN_WR` reader - "]
pub struct OVR_SRESETN_WR_R(crate::FieldReader<bool, bool>);
impl OVR_SRESETN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_SRESETN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_SRESETN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_SRESETN_WR` writer - "]
pub struct OVR_SRESETN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_SRESETN_WR_W<'a> {
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
#[doc = "Field `OVR_SRESETN_SEL` reader - "]
pub struct OVR_SRESETN_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_SRESETN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_SRESETN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_SRESETN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_SRESETN_SEL` writer - "]
pub struct OVR_SRESETN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_SRESETN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `OVR_ENPAIN_WR` reader - "]
pub struct OVR_ENPAIN_WR_R(crate::FieldReader<bool, bool>);
impl OVR_ENPAIN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_ENPAIN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_ENPAIN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_ENPAIN_WR` writer - "]
pub struct OVR_ENPAIN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_ENPAIN_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `OVR_ENPAIN_SEL` reader - "]
pub struct OVR_ENPAIN_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_ENPAIN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_ENPAIN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_ENPAIN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_ENPAIN_SEL` writer - "]
pub struct OVR_ENPAIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_ENPAIN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `OVR_RXBIT_WR` reader - "]
pub struct OVR_RXBIT_WR_R(crate::FieldReader<bool, bool>);
impl OVR_RXBIT_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RXBIT_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RXBIT_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RXBIT_WR` writer - "]
pub struct OVR_RXBIT_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_RXBIT_WR_W<'a> {
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
#[doc = "Field `OVR_RXBIT_SEL` reader - "]
pub struct OVR_RXBIT_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_RXBIT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RXBIT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RXBIT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RXBIT_SEL` writer - "]
pub struct OVR_RXBIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_RXBIT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `OVR_ACTIVE_WR` reader - "]
pub struct OVR_ACTIVE_WR_R(crate::FieldReader<bool, bool>);
impl OVR_ACTIVE_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_ACTIVE_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_ACTIVE_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_ACTIVE_WR` writer - "]
pub struct OVR_ACTIVE_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_ACTIVE_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `OVR_ACTIVE_SEL` reader - "]
pub struct OVR_ACTIVE_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_ACTIVE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_ACTIVE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_ACTIVE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_ACTIVE_SEL` writer - "]
pub struct OVR_ACTIVE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_ACTIVE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn ovr_dtc_oh_wr(&self) -> OVR_DTC_OH_WR_R {
        OVR_DTC_OH_WR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ovr_dtc_oh_sel(&self) -> OVR_DTC_OH_SEL_R {
        OVR_DTC_OH_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 17:20"]
    #[inline(always)]
    pub fn ovr_dcoamp_wr(&self) -> OVR_DCOAMP_WR_R {
        OVR_DCOAMP_WR_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ovr_dcoamp_sel(&self) -> OVR_DCOAMP_SEL_R {
        OVR_DCOAMP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ovr_dcoamphold_wr(&self) -> OVR_DCOAMPHOLD_WR_R {
        OVR_DCOAMPHOLD_WR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ovr_dcoamphold_sel(&self) -> OVR_DCOAMPHOLD_SEL_R {
        OVR_DCOAMPHOLD_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovr_rdyfordiv_wr(&self) -> OVR_RDYFORDIV_WR_R {
        OVR_RDYFORDIV_WR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ovr_rdyfordiv_sel(&self) -> OVR_RDYFORDIV_SEL_R {
        OVR_RDYFORDIV_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ovr_vpaen_wr(&self) -> OVR_VPAEN_WR_R {
        OVR_VPAEN_WR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ovr_vpaen_sel(&self) -> OVR_VPAEN_SEL_R {
        OVR_VPAEN_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ovr_sresetn_wr(&self) -> OVR_SRESETN_WR_R {
        OVR_SRESETN_WR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ovr_sresetn_sel(&self) -> OVR_SRESETN_SEL_R {
        OVR_SRESETN_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ovr_enpain_wr(&self) -> OVR_ENPAIN_WR_R {
        OVR_ENPAIN_WR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ovr_enpain_sel(&self) -> OVR_ENPAIN_SEL_R {
        OVR_ENPAIN_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ovr_rxbit_wr(&self) -> OVR_RXBIT_WR_R {
        OVR_RXBIT_WR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ovr_rxbit_sel(&self) -> OVR_RXBIT_SEL_R {
        OVR_RXBIT_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ovr_active_wr(&self) -> OVR_ACTIVE_WR_R {
        OVR_ACTIVE_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ovr_active_sel(&self) -> OVR_ACTIVE_SEL_R {
        OVR_ACTIVE_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn ovr_dtc_oh_wr(&mut self) -> OVR_DTC_OH_WR_W {
        OVR_DTC_OH_WR_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ovr_dtc_oh_sel(&mut self) -> OVR_DTC_OH_SEL_W {
        OVR_DTC_OH_SEL_W { w: self }
    }
    #[doc = "Bits 17:20"]
    #[inline(always)]
    pub fn ovr_dcoamp_wr(&mut self) -> OVR_DCOAMP_WR_W {
        OVR_DCOAMP_WR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ovr_dcoamp_sel(&mut self) -> OVR_DCOAMP_SEL_W {
        OVR_DCOAMP_SEL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ovr_dcoamphold_wr(&mut self) -> OVR_DCOAMPHOLD_WR_W {
        OVR_DCOAMPHOLD_WR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ovr_dcoamphold_sel(&mut self) -> OVR_DCOAMPHOLD_SEL_W {
        OVR_DCOAMPHOLD_SEL_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovr_rdyfordiv_wr(&mut self) -> OVR_RDYFORDIV_WR_W {
        OVR_RDYFORDIV_WR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ovr_rdyfordiv_sel(&mut self) -> OVR_RDYFORDIV_SEL_W {
        OVR_RDYFORDIV_SEL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ovr_vpaen_wr(&mut self) -> OVR_VPAEN_WR_W {
        OVR_VPAEN_WR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ovr_vpaen_sel(&mut self) -> OVR_VPAEN_SEL_W {
        OVR_VPAEN_SEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ovr_sresetn_wr(&mut self) -> OVR_SRESETN_WR_W {
        OVR_SRESETN_WR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ovr_sresetn_sel(&mut self) -> OVR_SRESETN_SEL_W {
        OVR_SRESETN_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ovr_enpain_wr(&mut self) -> OVR_ENPAIN_WR_W {
        OVR_ENPAIN_WR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ovr_enpain_sel(&mut self) -> OVR_ENPAIN_SEL_W {
        OVR_ENPAIN_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ovr_rxbit_wr(&mut self) -> OVR_RXBIT_WR_W {
        OVR_RXBIT_WR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ovr_rxbit_sel(&mut self) -> OVR_RXBIT_SEL_W {
        OVR_RXBIT_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ovr_active_wr(&mut self) -> OVR_ACTIVE_WR_W {
        OVR_ACTIVE_WR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ovr_active_sel(&mut self) -> OVR_ACTIVE_SEL_W {
        OVR_ACTIVE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_overrule_ctrl1_reg](index.html) module"]
pub struct ADPLL_OVERRULE_CTRL1_REG_SPEC;
impl crate::RegisterSpec for ADPLL_OVERRULE_CTRL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_overrule_ctrl1_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_OVERRULE_CTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_overrule_ctrl1_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_OVERRULE_CTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_OVERRULE_CTRL1_REG to value 0"]
impl crate::Resettable for ADPLL_OVERRULE_CTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
