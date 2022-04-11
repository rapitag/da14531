#[doc = "Register `BLE_ENBPRESET_REG` reader"]
pub struct R(crate::R<BLE_ENBPRESET_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_ENBPRESET_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_ENBPRESET_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_ENBPRESET_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_ENBPRESET_REG` writer"]
pub struct W(crate::W<BLE_ENBPRESET_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_ENBPRESET_REG_SPEC>;
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
impl From<crate::W<BLE_ENBPRESET_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_ENBPRESET_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWEXT` reader - Minimum and recommended value is \"TWIRQ_RESET + 1\". In the case of wake-up due to an external wake-up request, TWEXT specifies the time delay in low power oscillator cycles to deassert BLE_WAKEUP_LP_IRQ. Refer also to GP_CONTROL_REG\\[BLE_WAKEUP_REQ\\]. Range is \\[0...64 ms\\]
for 32kHz; \\[0...62.5 ms\\]
for 32.768kHz"]
pub struct TWEXT_R(crate::FieldReader<u16, u16>);
impl TWEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TWEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWEXT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWEXT` writer - Minimum and recommended value is \"TWIRQ_RESET + 1\". In the case of wake-up due to an external wake-up request, TWEXT specifies the time delay in low power oscillator cycles to deassert BLE_WAKEUP_LP_IRQ. Refer also to GP_CONTROL_REG\\[BLE_WAKEUP_REQ\\]. Range is \\[0...64 ms\\]
for 32kHz; \\[0...62.5 ms\\]
for 32.768kHz"]
pub struct TWEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TWEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | ((value as u32 & 0x07ff) << 21);
        self.w
    }
}
#[doc = "Field `TWIRQ_SET` reader - Minimum value is \"TWIRQ_RESET + 1\". Time in low power oscillator cycles to set BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration. Refer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\]. Range is \\[0...64 ms\\]
for 32kHz; \\[0...62.5 ms\\]
for 32.768kHz"]
pub struct TWIRQ_SET_R(crate::FieldReader<u16, u16>);
impl TWIRQ_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TWIRQ_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIRQ_SET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIRQ_SET` writer - Minimum value is \"TWIRQ_RESET + 1\". Time in low power oscillator cycles to set BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration. Refer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\]. Range is \\[0...64 ms\\]
for 32kHz; \\[0...62.5 ms\\]
for 32.768kHz"]
pub struct TWIRQ_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIRQ_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 10)) | ((value as u32 & 0x07ff) << 10);
        self.w
    }
}
#[doc = "Field `TWIRQ_RESET` reader - Recommended value is 1. Time in low power oscillator cycles to reset BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration. Refer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\]. Range is \\[0...32 ms\\]
for 32kHz; \\[0...31.25 ms\\]
for 32.768kHz."]
pub struct TWIRQ_RESET_R(crate::FieldReader<u16, u16>);
impl TWIRQ_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TWIRQ_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIRQ_RESET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIRQ_RESET` writer - Recommended value is 1. Time in low power oscillator cycles to reset BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration. Refer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\]. Range is \\[0...32 ms\\]
for 32kHz; \\[0...31.25 ms\\]
for 32.768kHz."]
pub struct TWIRQ_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIRQ_RESET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 21:31 - Minimum and recommended value is \"TWIRQ_RESET + 1\". In the case of wake-up due to an external wake-up request, TWEXT specifies the time delay in low power oscillator cycles to deassert BLE_WAKEUP_LP_IRQ. Refer also to GP_CONTROL_REG\\[BLE_WAKEUP_REQ\\]. Range is \\[0...64 ms\\]
for 32kHz; \\[0...62.5 ms\\]
for 32.768kHz"]
    #[inline(always)]
    pub fn twext(&self) -> TWEXT_R {
        TWEXT_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 10:20 - Minimum value is \"TWIRQ_RESET + 1\". Time in low power oscillator cycles to set BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration. Refer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\]. Range is \\[0...64 ms\\]
for 32kHz; \\[0...62.5 ms\\]
for 32.768kHz"]
    #[inline(always)]
    pub fn twirq_set(&self) -> TWIRQ_SET_R {
        TWIRQ_SET_R::new(((self.bits >> 10) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:9 - Recommended value is 1. Time in low power oscillator cycles to reset BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration. Refer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\]. Range is \\[0...32 ms\\]
for 32kHz; \\[0...31.25 ms\\]
for 32.768kHz."]
    #[inline(always)]
    pub fn twirq_reset(&self) -> TWIRQ_RESET_R {
        TWIRQ_RESET_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 21:31 - Minimum and recommended value is \"TWIRQ_RESET + 1\". In the case of wake-up due to an external wake-up request, TWEXT specifies the time delay in low power oscillator cycles to deassert BLE_WAKEUP_LP_IRQ. Refer also to GP_CONTROL_REG\\[BLE_WAKEUP_REQ\\]. Range is \\[0...64 ms\\]
for 32kHz; \\[0...62.5 ms\\]
for 32.768kHz"]
    #[inline(always)]
    pub fn twext(&mut self) -> TWEXT_W {
        TWEXT_W { w: self }
    }
    #[doc = "Bits 10:20 - Minimum value is \"TWIRQ_RESET + 1\". Time in low power oscillator cycles to set BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration. Refer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\]. Range is \\[0...64 ms\\]
for 32kHz; \\[0...62.5 ms\\]
for 32.768kHz"]
    #[inline(always)]
    pub fn twirq_set(&mut self) -> TWIRQ_SET_W {
        TWIRQ_SET_W { w: self }
    }
    #[doc = "Bits 0:9 - Recommended value is 1. Time in low power oscillator cycles to reset BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration. Refer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\]. Range is \\[0...32 ms\\]
for 32kHz; \\[0...31.25 ms\\]
for 32.768kHz."]
    #[inline(always)]
    pub fn twirq_reset(&mut self) -> TWIRQ_RESET_W {
        TWIRQ_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time in low power oscillator cycles register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_enbpreset_reg](index.html) module"]
pub struct BLE_ENBPRESET_REG_SPEC;
impl crate::RegisterSpec for BLE_ENBPRESET_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_enbpreset_reg::R](R) reader structure"]
impl crate::Readable for BLE_ENBPRESET_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_enbpreset_reg::W](W) writer structure"]
impl crate::Writable for BLE_ENBPRESET_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_ENBPRESET_REG to value 0"]
impl crate::Resettable for BLE_ENBPRESET_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
