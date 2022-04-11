#[doc = "Register `RF_DIAGIRQ_CTRL_REG` reader"]
pub struct R(crate::R<RF_DIAGIRQ_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_DIAGIRQ_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_DIAGIRQ_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_DIAGIRQ_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_DIAGIRQ_CTRL_REG` writer"]
pub struct W(crate::W<RF_DIAGIRQ_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_DIAGIRQ_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_DIAGIRQ_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_DIAGIRQ_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAG_BUS3_EDGE_SEL` reader - "]
pub struct DIAG_BUS3_EDGE_SEL_R(crate::FieldReader<bool, bool>);
impl DIAG_BUS3_EDGE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG_BUS3_EDGE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS3_EDGE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS3_EDGE_SEL` writer - "]
pub struct DIAG_BUS3_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS3_EDGE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `DIAG_BUS3_BIT_SEL` reader - "]
pub struct DIAG_BUS3_BIT_SEL_R(crate::FieldReader<u8, u8>);
impl DIAG_BUS3_BIT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG_BUS3_BIT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS3_BIT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS3_BIT_SEL` writer - "]
pub struct DIAG_BUS3_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS3_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 27)) | ((value as u32 & 7) << 27);
        self.w
    }
}
#[doc = "Field `DIAG_BUS3_SEL` reader - "]
pub struct DIAG_BUS3_SEL_R(crate::FieldReader<u8, u8>);
impl DIAG_BUS3_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG_BUS3_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS3_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS3_SEL` writer - "]
pub struct DIAG_BUS3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 25)) | ((value as u32 & 3) << 25);
        self.w
    }
}
#[doc = "Field `DIAG_BUS3_IRQ_MASK` reader - "]
pub struct DIAG_BUS3_IRQ_MASK_R(crate::FieldReader<bool, bool>);
impl DIAG_BUS3_IRQ_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG_BUS3_IRQ_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS3_IRQ_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS3_IRQ_MASK` writer - "]
pub struct DIAG_BUS3_IRQ_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS3_IRQ_MASK_W<'a> {
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
#[doc = "Field `DIAG_BUS2_EDGE_SEL` reader - "]
pub struct DIAG_BUS2_EDGE_SEL_R(crate::FieldReader<bool, bool>);
impl DIAG_BUS2_EDGE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG_BUS2_EDGE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS2_EDGE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS2_EDGE_SEL` writer - "]
pub struct DIAG_BUS2_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS2_EDGE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `DIAG_BUS2_BIT_SEL` reader - "]
pub struct DIAG_BUS2_BIT_SEL_R(crate::FieldReader<u8, u8>);
impl DIAG_BUS2_BIT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG_BUS2_BIT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS2_BIT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS2_BIT_SEL` writer - "]
pub struct DIAG_BUS2_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS2_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 19)) | ((value as u32 & 7) << 19);
        self.w
    }
}
#[doc = "Field `DIAG_BUS2_SEL` reader - "]
pub struct DIAG_BUS2_SEL_R(crate::FieldReader<u8, u8>);
impl DIAG_BUS2_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG_BUS2_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS2_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS2_SEL` writer - "]
pub struct DIAG_BUS2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 17)) | ((value as u32 & 3) << 17);
        self.w
    }
}
#[doc = "Field `DIAG_BUS2_IRQ_MASK` reader - "]
pub struct DIAG_BUS2_IRQ_MASK_R(crate::FieldReader<bool, bool>);
impl DIAG_BUS2_IRQ_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG_BUS2_IRQ_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS2_IRQ_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS2_IRQ_MASK` writer - "]
pub struct DIAG_BUS2_IRQ_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS2_IRQ_MASK_W<'a> {
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
#[doc = "Field `DIAG_BUS1_EDGE_SEL` reader - "]
pub struct DIAG_BUS1_EDGE_SEL_R(crate::FieldReader<bool, bool>);
impl DIAG_BUS1_EDGE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG_BUS1_EDGE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS1_EDGE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS1_EDGE_SEL` writer - "]
pub struct DIAG_BUS1_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS1_EDGE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `DIAG_BUS1_BIT_SEL` reader - "]
pub struct DIAG_BUS1_BIT_SEL_R(crate::FieldReader<u8, u8>);
impl DIAG_BUS1_BIT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG_BUS1_BIT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS1_BIT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS1_BIT_SEL` writer - "]
pub struct DIAG_BUS1_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS1_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 11)) | ((value as u32 & 7) << 11);
        self.w
    }
}
#[doc = "Field `DIAG_BUS1_SEL` reader - "]
pub struct DIAG_BUS1_SEL_R(crate::FieldReader<u8, u8>);
impl DIAG_BUS1_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG_BUS1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS1_SEL` writer - "]
pub struct DIAG_BUS1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 9)) | ((value as u32 & 3) << 9);
        self.w
    }
}
#[doc = "Field `DIAG_BUS1_IRQ_MASK` reader - "]
pub struct DIAG_BUS1_IRQ_MASK_R(crate::FieldReader<bool, bool>);
impl DIAG_BUS1_IRQ_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG_BUS1_IRQ_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS1_IRQ_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS1_IRQ_MASK` writer - "]
pub struct DIAG_BUS1_IRQ_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS1_IRQ_MASK_W<'a> {
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
#[doc = "Field `DIAG_BUS0_EDGE_SEL` reader - "]
pub struct DIAG_BUS0_EDGE_SEL_R(crate::FieldReader<bool, bool>);
impl DIAG_BUS0_EDGE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG_BUS0_EDGE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS0_EDGE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS0_EDGE_SEL` writer - "]
pub struct DIAG_BUS0_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS0_EDGE_SEL_W<'a> {
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
#[doc = "Field `DIAG_BUS0_BIT_SEL` reader - "]
pub struct DIAG_BUS0_BIT_SEL_R(crate::FieldReader<u8, u8>);
impl DIAG_BUS0_BIT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG_BUS0_BIT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS0_BIT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS0_BIT_SEL` writer - "]
pub struct DIAG_BUS0_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS0_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u32 & 7) << 3);
        self.w
    }
}
#[doc = "Field `DIAG_BUS0_SEL` reader - "]
pub struct DIAG_BUS0_SEL_R(crate::FieldReader<u8, u8>);
impl DIAG_BUS0_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG_BUS0_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS0_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS0_SEL` writer - "]
pub struct DIAG_BUS0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 1)) | ((value as u32 & 3) << 1);
        self.w
    }
}
#[doc = "Field `DIAG_BUS0_IRQ_MASK` reader - "]
pub struct DIAG_BUS0_IRQ_MASK_R(crate::FieldReader<bool, bool>);
impl DIAG_BUS0_IRQ_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAG_BUS0_IRQ_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG_BUS0_IRQ_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG_BUS0_IRQ_MASK` writer - "]
pub struct DIAG_BUS0_IRQ_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG_BUS0_IRQ_MASK_W<'a> {
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
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn diag_bus3_edge_sel(&self) -> DIAG_BUS3_EDGE_SEL_R {
        DIAG_BUS3_EDGE_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn diag_bus3_bit_sel(&self) -> DIAG_BUS3_BIT_SEL_R {
        DIAG_BUS3_BIT_SEL_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn diag_bus3_sel(&self) -> DIAG_BUS3_SEL_R {
        DIAG_BUS3_SEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn diag_bus3_irq_mask(&self) -> DIAG_BUS3_IRQ_MASK_R {
        DIAG_BUS3_IRQ_MASK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn diag_bus2_edge_sel(&self) -> DIAG_BUS2_EDGE_SEL_R {
        DIAG_BUS2_EDGE_SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn diag_bus2_bit_sel(&self) -> DIAG_BUS2_BIT_SEL_R {
        DIAG_BUS2_BIT_SEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn diag_bus2_sel(&self) -> DIAG_BUS2_SEL_R {
        DIAG_BUS2_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn diag_bus2_irq_mask(&self) -> DIAG_BUS2_IRQ_MASK_R {
        DIAG_BUS2_IRQ_MASK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn diag_bus1_edge_sel(&self) -> DIAG_BUS1_EDGE_SEL_R {
        DIAG_BUS1_EDGE_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn diag_bus1_bit_sel(&self) -> DIAG_BUS1_BIT_SEL_R {
        DIAG_BUS1_BIT_SEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn diag_bus1_sel(&self) -> DIAG_BUS1_SEL_R {
        DIAG_BUS1_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn diag_bus1_irq_mask(&self) -> DIAG_BUS1_IRQ_MASK_R {
        DIAG_BUS1_IRQ_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn diag_bus0_edge_sel(&self) -> DIAG_BUS0_EDGE_SEL_R {
        DIAG_BUS0_EDGE_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn diag_bus0_bit_sel(&self) -> DIAG_BUS0_BIT_SEL_R {
        DIAG_BUS0_BIT_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn diag_bus0_sel(&self) -> DIAG_BUS0_SEL_R {
        DIAG_BUS0_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn diag_bus0_irq_mask(&self) -> DIAG_BUS0_IRQ_MASK_R {
        DIAG_BUS0_IRQ_MASK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn diag_bus3_edge_sel(&mut self) -> DIAG_BUS3_EDGE_SEL_W {
        DIAG_BUS3_EDGE_SEL_W { w: self }
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn diag_bus3_bit_sel(&mut self) -> DIAG_BUS3_BIT_SEL_W {
        DIAG_BUS3_BIT_SEL_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn diag_bus3_sel(&mut self) -> DIAG_BUS3_SEL_W {
        DIAG_BUS3_SEL_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn diag_bus3_irq_mask(&mut self) -> DIAG_BUS3_IRQ_MASK_W {
        DIAG_BUS3_IRQ_MASK_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn diag_bus2_edge_sel(&mut self) -> DIAG_BUS2_EDGE_SEL_W {
        DIAG_BUS2_EDGE_SEL_W { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn diag_bus2_bit_sel(&mut self) -> DIAG_BUS2_BIT_SEL_W {
        DIAG_BUS2_BIT_SEL_W { w: self }
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn diag_bus2_sel(&mut self) -> DIAG_BUS2_SEL_W {
        DIAG_BUS2_SEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn diag_bus2_irq_mask(&mut self) -> DIAG_BUS2_IRQ_MASK_W {
        DIAG_BUS2_IRQ_MASK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn diag_bus1_edge_sel(&mut self) -> DIAG_BUS1_EDGE_SEL_W {
        DIAG_BUS1_EDGE_SEL_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn diag_bus1_bit_sel(&mut self) -> DIAG_BUS1_BIT_SEL_W {
        DIAG_BUS1_BIT_SEL_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn diag_bus1_sel(&mut self) -> DIAG_BUS1_SEL_W {
        DIAG_BUS1_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn diag_bus1_irq_mask(&mut self) -> DIAG_BUS1_IRQ_MASK_W {
        DIAG_BUS1_IRQ_MASK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn diag_bus0_edge_sel(&mut self) -> DIAG_BUS0_EDGE_SEL_W {
        DIAG_BUS0_EDGE_SEL_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn diag_bus0_bit_sel(&mut self) -> DIAG_BUS0_BIT_SEL_W {
        DIAG_BUS0_BIT_SEL_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn diag_bus0_sel(&mut self) -> DIAG_BUS0_SEL_W {
        DIAG_BUS0_SEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn diag_bus0_irq_mask(&mut self) -> DIAG_BUS0_IRQ_MASK_W {
        DIAG_BUS0_IRQ_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_diagirq_ctrl_reg](index.html) module"]
pub struct RF_DIAGIRQ_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_DIAGIRQ_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_diagirq_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_DIAGIRQ_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_diagirq_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_DIAGIRQ_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_DIAGIRQ_CTRL_REG to value 0"]
impl crate::Resettable for RF_DIAGIRQ_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
