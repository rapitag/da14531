#[doc = "Register `BLE_TIMER_REG` reader"]
pub struct R(crate::R<BLE_TIMER_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_TIMER_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_TIMER_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_TIMER_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_TIMER_REG` writer"]
pub struct W(crate::W<BLE_TIMER_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_TIMER_REG_SPEC>;
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
impl From<crate::W<BLE_TIMER_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_TIMER_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLE_TIMER_DATA` reader - Operation depends on GP_CONTROL_REG->BLE_TIMER_DATA_CTRL. If BLE_TIMER_DATA_CTRL = 0 then: This register is located at the Always On Power Domain and it holds the automatically sampled value of the BLE FINECNT timer The HW automatically samples the value into this register during the sequence of \"BLE Sleep On\" and restores automatically the value during the BLE Wake up sequence. The Software may read and modify the value while the BLE is in Sleep state. While the BLE is awake, the value of the register has no meaning, while changing the value by writing another one will have no effect in the operation of the BLE core. There is a constraint when the SW performs an write-read sequence where it has to inject a one cycle delay in between (e.g. write-NOP-read) in order to read back the correct value. If BLE_TIMER_DATA_CTRL is non 0 then write operations have the same effect as when BLE_TIMER_DATA_CTRL=0, while for read operations: BLE_TIMER_DATA_CTRL= 1: then reading BLE_TIMER_REG returns \"deepsldur\\[9:0\\]\". BLE_TIMER_DATA_CTRL= 2: then reading BLE_TIMER_REG returns \"deepsltime_samp\\[9:0\\]\". BLE_TIMER_DATA_CTRL= 3: then reading BLE_TIMER_REG returns \"{deep_sleep_stat_monitor, deepsltime_samp\\[18:10\\]}. ."]
pub struct BLE_TIMER_DATA_R(crate::FieldReader<u16, u16>);
impl BLE_TIMER_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BLE_TIMER_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_TIMER_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_TIMER_DATA` writer - Operation depends on GP_CONTROL_REG->BLE_TIMER_DATA_CTRL. If BLE_TIMER_DATA_CTRL = 0 then: This register is located at the Always On Power Domain and it holds the automatically sampled value of the BLE FINECNT timer The HW automatically samples the value into this register during the sequence of \"BLE Sleep On\" and restores automatically the value during the BLE Wake up sequence. The Software may read and modify the value while the BLE is in Sleep state. While the BLE is awake, the value of the register has no meaning, while changing the value by writing another one will have no effect in the operation of the BLE core. There is a constraint when the SW performs an write-read sequence where it has to inject a one cycle delay in between (e.g. write-NOP-read) in order to read back the correct value. If BLE_TIMER_DATA_CTRL is non 0 then write operations have the same effect as when BLE_TIMER_DATA_CTRL=0, while for read operations: BLE_TIMER_DATA_CTRL= 1: then reading BLE_TIMER_REG returns \"deepsldur\\[9:0\\]\". BLE_TIMER_DATA_CTRL= 2: then reading BLE_TIMER_REG returns \"deepsltime_samp\\[9:0\\]\". BLE_TIMER_DATA_CTRL= 3: then reading BLE_TIMER_REG returns \"{deep_sleep_stat_monitor, deepsltime_samp\\[18:10\\]}. ."]
pub struct BLE_TIMER_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_TIMER_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Operation depends on GP_CONTROL_REG->BLE_TIMER_DATA_CTRL. If BLE_TIMER_DATA_CTRL = 0 then: This register is located at the Always On Power Domain and it holds the automatically sampled value of the BLE FINECNT timer The HW automatically samples the value into this register during the sequence of \"BLE Sleep On\" and restores automatically the value during the BLE Wake up sequence. The Software may read and modify the value while the BLE is in Sleep state. While the BLE is awake, the value of the register has no meaning, while changing the value by writing another one will have no effect in the operation of the BLE core. There is a constraint when the SW performs an write-read sequence where it has to inject a one cycle delay in between (e.g. write-NOP-read) in order to read back the correct value. If BLE_TIMER_DATA_CTRL is non 0 then write operations have the same effect as when BLE_TIMER_DATA_CTRL=0, while for read operations: BLE_TIMER_DATA_CTRL= 1: then reading BLE_TIMER_REG returns \"deepsldur\\[9:0\\]\". BLE_TIMER_DATA_CTRL= 2: then reading BLE_TIMER_REG returns \"deepsltime_samp\\[9:0\\]\". BLE_TIMER_DATA_CTRL= 3: then reading BLE_TIMER_REG returns \"{deep_sleep_stat_monitor, deepsltime_samp\\[18:10\\]}. ."]
    #[inline(always)]
    pub fn ble_timer_data(&self) -> BLE_TIMER_DATA_R {
        BLE_TIMER_DATA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Operation depends on GP_CONTROL_REG->BLE_TIMER_DATA_CTRL. If BLE_TIMER_DATA_CTRL = 0 then: This register is located at the Always On Power Domain and it holds the automatically sampled value of the BLE FINECNT timer The HW automatically samples the value into this register during the sequence of \"BLE Sleep On\" and restores automatically the value during the BLE Wake up sequence. The Software may read and modify the value while the BLE is in Sleep state. While the BLE is awake, the value of the register has no meaning, while changing the value by writing another one will have no effect in the operation of the BLE core. There is a constraint when the SW performs an write-read sequence where it has to inject a one cycle delay in between (e.g. write-NOP-read) in order to read back the correct value. If BLE_TIMER_DATA_CTRL is non 0 then write operations have the same effect as when BLE_TIMER_DATA_CTRL=0, while for read operations: BLE_TIMER_DATA_CTRL= 1: then reading BLE_TIMER_REG returns \"deepsldur\\[9:0\\]\". BLE_TIMER_DATA_CTRL= 2: then reading BLE_TIMER_REG returns \"deepsltime_samp\\[9:0\\]\". BLE_TIMER_DATA_CTRL= 3: then reading BLE_TIMER_REG returns \"{deep_sleep_stat_monitor, deepsltime_samp\\[18:10\\]}. ."]
    #[inline(always)]
    pub fn ble_timer_data(&mut self) -> BLE_TIMER_DATA_W {
        BLE_TIMER_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE FINECNT sampled value while in deep sleep state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_timer_reg](index.html) module"]
pub struct BLE_TIMER_REG_SPEC;
impl crate::RegisterSpec for BLE_TIMER_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ble_timer_reg::R](R) reader structure"]
impl crate::Readable for BLE_TIMER_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_timer_reg::W](W) writer structure"]
impl crate::Writable for BLE_TIMER_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_TIMER_REG to value 0"]
impl crate::Resettable for BLE_TIMER_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
