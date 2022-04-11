#[doc = "Register `I2C_INTR_MASK_REG` reader"]
pub struct R(crate::R<I2C_INTR_MASK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_INTR_MASK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_INTR_MASK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_INTR_MASK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_INTR_MASK_REG` writer"]
pub struct W(crate::W<I2C_INTR_MASK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_INTR_MASK_REG_SPEC>;
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
impl From<crate::W<I2C_INTR_MASK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_INTR_MASK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_GEN_CALL` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_GEN_CALL_R(crate::FieldReader<bool, bool>);
impl M_GEN_CALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_GEN_CALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_GEN_CALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_GEN_CALL` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_GEN_CALL_W<'a> {
    w: &'a mut W,
}
impl<'a> M_GEN_CALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u16 & 1) << 11);
        self.w
    }
}
#[doc = "Field `M_START_DET` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_START_DET_R(crate::FieldReader<bool, bool>);
impl M_START_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_START_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_START_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_START_DET` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_START_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> M_START_DET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u16 & 1) << 10);
        self.w
    }
}
#[doc = "Field `M_STOP_DET` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_STOP_DET_R(crate::FieldReader<bool, bool>);
impl M_STOP_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_STOP_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_STOP_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_STOP_DET` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_STOP_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> M_STOP_DET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
#[doc = "Field `M_ACTIVITY` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_ACTIVITY_R(crate::FieldReader<bool, bool>);
impl M_ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_ACTIVITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_ACTIVITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_ACTIVITY` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_ACTIVITY_W<'a> {
    w: &'a mut W,
}
impl<'a> M_ACTIVITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `M_RX_DONE` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RX_DONE_R(crate::FieldReader<bool, bool>);
impl M_RX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_RX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_RX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_RX_DONE` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> M_RX_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `M_TX_ABRT` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_TX_ABRT_R(crate::FieldReader<bool, bool>);
impl M_TX_ABRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_TX_ABRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_TX_ABRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_TX_ABRT` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_TX_ABRT_W<'a> {
    w: &'a mut W,
}
impl<'a> M_TX_ABRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `M_RD_REQ` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RD_REQ_R(crate::FieldReader<bool, bool>);
impl M_RD_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_RD_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_RD_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_RD_REQ` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RD_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> M_RD_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `M_TX_EMPTY` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_TX_EMPTY_R(crate::FieldReader<bool, bool>);
impl M_TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_TX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_TX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_TX_EMPTY` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> M_TX_EMPTY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `M_TX_OVER` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_TX_OVER_R(crate::FieldReader<bool, bool>);
impl M_TX_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_TX_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_TX_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_TX_OVER` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_TX_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> M_TX_OVER_W<'a> {
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
#[doc = "Field `M_RX_FULL` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RX_FULL_R(crate::FieldReader<bool, bool>);
impl M_RX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_RX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_RX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_RX_FULL` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> M_RX_FULL_W<'a> {
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
#[doc = "Field `M_RX_OVER` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RX_OVER_R(crate::FieldReader<bool, bool>);
impl M_RX_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_RX_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_RX_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_RX_OVER` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RX_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> M_RX_OVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `M_RX_UNDER` reader - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RX_UNDER_R(crate::FieldReader<bool, bool>);
impl M_RX_UNDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_RX_UNDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_RX_UNDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_RX_UNDER` writer - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
pub struct M_RX_UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> M_RX_UNDER_W<'a> {
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
    #[doc = "Bit 11 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_gen_call(&self) -> M_GEN_CALL_R {
        M_GEN_CALL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_start_det(&self) -> M_START_DET_R {
        M_START_DET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_stop_det(&self) -> M_STOP_DET_R {
        M_STOP_DET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_activity(&self) -> M_ACTIVITY_R {
        M_ACTIVITY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_done(&self) -> M_RX_DONE_R {
        M_RX_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_abrt(&self) -> M_TX_ABRT_R {
        M_TX_ABRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rd_req(&self) -> M_RD_REQ_R {
        M_RD_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_empty(&self) -> M_TX_EMPTY_R {
        M_TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_over(&self) -> M_TX_OVER_R {
        M_TX_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_full(&self) -> M_RX_FULL_R {
        M_RX_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_over(&self) -> M_RX_OVER_R {
        M_RX_OVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_under(&self) -> M_RX_UNDER_R {
        M_RX_UNDER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_gen_call(&mut self) -> M_GEN_CALL_W {
        M_GEN_CALL_W { w: self }
    }
    #[doc = "Bit 10 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_start_det(&mut self) -> M_START_DET_W {
        M_START_DET_W { w: self }
    }
    #[doc = "Bit 9 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_stop_det(&mut self) -> M_STOP_DET_W {
        M_STOP_DET_W { w: self }
    }
    #[doc = "Bit 8 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_activity(&mut self) -> M_ACTIVITY_W {
        M_ACTIVITY_W { w: self }
    }
    #[doc = "Bit 7 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_done(&mut self) -> M_RX_DONE_W {
        M_RX_DONE_W { w: self }
    }
    #[doc = "Bit 6 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_abrt(&mut self) -> M_TX_ABRT_W {
        M_TX_ABRT_W { w: self }
    }
    #[doc = "Bit 5 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rd_req(&mut self) -> M_RD_REQ_W {
        M_RD_REQ_W { w: self }
    }
    #[doc = "Bit 4 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_empty(&mut self) -> M_TX_EMPTY_W {
        M_TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 3 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_over(&mut self) -> M_TX_OVER_W {
        M_TX_OVER_W { w: self }
    }
    #[doc = "Bit 2 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_full(&mut self) -> M_RX_FULL_W {
        M_RX_FULL_W { w: self }
    }
    #[doc = "Bit 1 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_over(&mut self) -> M_RX_OVER_W {
        M_RX_OVER_W { w: self }
    }
    #[doc = "Bit 0 - These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_under(&mut self) -> M_RX_UNDER_W {
        M_RX_UNDER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_intr_mask_reg](index.html) module"]
pub struct I2C_INTR_MASK_REG_SPEC;
impl crate::RegisterSpec for I2C_INTR_MASK_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_intr_mask_reg::R](R) reader structure"]
impl crate::Readable for I2C_INTR_MASK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_intr_mask_reg::W](W) writer structure"]
impl crate::Writable for I2C_INTR_MASK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_INTR_MASK_REG to value 0x08ff"]
impl crate::Resettable for I2C_INTR_MASK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08ff
    }
}
