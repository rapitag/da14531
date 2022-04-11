#[doc = "Register `GP_CONTROL_REG` reader"]
pub struct R(crate::R<GP_CONTROL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_CONTROL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_CONTROL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_CONTROL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_CONTROL_REG` writer"]
pub struct W(crate::W<GP_CONTROL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_CONTROL_REG_SPEC>;
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
impl From<crate::W<GP_CONTROL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_CONTROL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLE_TIMER_DATA_CTRL` reader - Refer to BLE_TIMER_REG."]
pub struct BLE_TIMER_DATA_CTRL_R(crate::FieldReader<u8, u8>);
impl BLE_TIMER_DATA_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLE_TIMER_DATA_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_TIMER_DATA_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_TIMER_DATA_CTRL` writer - Refer to BLE_TIMER_REG."]
pub struct BLE_TIMER_DATA_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_TIMER_DATA_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 5)) | ((value as u16 & 3) << 5);
        self.w
    }
}
#[doc = "Field `CPU_DMA_BUS_PRIO` reader - Controls the CPU DMA system bus priority: If '0', the CPU has highest priority. If '1', the DMA has highest priority."]
pub struct CPU_DMA_BUS_PRIO_R(crate::FieldReader<bool, bool>);
impl CPU_DMA_BUS_PRIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_DMA_BUS_PRIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_DMA_BUS_PRIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_DMA_BUS_PRIO` writer - Controls the CPU DMA system bus priority: If '0', the CPU has highest priority. If '1', the DMA has highest priority."]
pub struct CPU_DMA_BUS_PRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_DMA_BUS_PRIO_W<'a> {
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
#[doc = "Field `BLE_WAKEUP_LP_IRQ` reader - The current value of the BLE_WAKEUP_LP_IRQ interrupt request."]
pub struct BLE_WAKEUP_LP_IRQ_R(crate::FieldReader<bool, bool>);
impl BLE_WAKEUP_LP_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_WAKEUP_LP_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_WAKEUP_LP_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_WAKEUP_REQ` reader - If '1', the BLE wakes up. Must be kept high at least for 1 low power clock period. If the BLE is in deep sleep state, then by setting this bit it will cause the wakeup LP IRQ to be asserted with a delay of 3 to 4 low power cycles."]
pub struct BLE_WAKEUP_REQ_R(crate::FieldReader<bool, bool>);
impl BLE_WAKEUP_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_WAKEUP_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_WAKEUP_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_WAKEUP_REQ` writer - If '1', the BLE wakes up. Must be kept high at least for 1 low power clock period. If the BLE is in deep sleep state, then by setting this bit it will cause the wakeup LP IRQ to be asserted with a delay of 3 to 4 low power cycles."]
pub struct BLE_WAKEUP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_WAKEUP_REQ_W<'a> {
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
    #[doc = "Bits 5:6 - Refer to BLE_TIMER_REG."]
    #[inline(always)]
    pub fn ble_timer_data_ctrl(&self) -> BLE_TIMER_DATA_CTRL_R {
        BLE_TIMER_DATA_CTRL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 4 - Controls the CPU DMA system bus priority: If '0', the CPU has highest priority. If '1', the DMA has highest priority."]
    #[inline(always)]
    pub fn cpu_dma_bus_prio(&self) -> CPU_DMA_BUS_PRIO_R {
        CPU_DMA_BUS_PRIO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - The current value of the BLE_WAKEUP_LP_IRQ interrupt request."]
    #[inline(always)]
    pub fn ble_wakeup_lp_irq(&self) -> BLE_WAKEUP_LP_IRQ_R {
        BLE_WAKEUP_LP_IRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - If '1', the BLE wakes up. Must be kept high at least for 1 low power clock period. If the BLE is in deep sleep state, then by setting this bit it will cause the wakeup LP IRQ to be asserted with a delay of 3 to 4 low power cycles."]
    #[inline(always)]
    pub fn ble_wakeup_req(&self) -> BLE_WAKEUP_REQ_R {
        BLE_WAKEUP_REQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6 - Refer to BLE_TIMER_REG."]
    #[inline(always)]
    pub fn ble_timer_data_ctrl(&mut self) -> BLE_TIMER_DATA_CTRL_W {
        BLE_TIMER_DATA_CTRL_W { w: self }
    }
    #[doc = "Bit 4 - Controls the CPU DMA system bus priority: If '0', the CPU has highest priority. If '1', the DMA has highest priority."]
    #[inline(always)]
    pub fn cpu_dma_bus_prio(&mut self) -> CPU_DMA_BUS_PRIO_W {
        CPU_DMA_BUS_PRIO_W { w: self }
    }
    #[doc = "Bit 0 - If '1', the BLE wakes up. Must be kept high at least for 1 low power clock period. If the BLE is in deep sleep state, then by setting this bit it will cause the wakeup LP IRQ to be asserted with a delay of 3 to 4 low power cycles."]
    #[inline(always)]
    pub fn ble_wakeup_req(&mut self) -> BLE_WAKEUP_REQ_W {
        BLE_WAKEUP_REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose system control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_control_reg](index.html) module"]
pub struct GP_CONTROL_REG_SPEC;
impl crate::RegisterSpec for GP_CONTROL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_control_reg::R](R) reader structure"]
impl crate::Readable for GP_CONTROL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_control_reg::W](W) writer structure"]
impl crate::Writable for GP_CONTROL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_CONTROL_REG to value 0"]
impl crate::Resettable for GP_CONTROL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
