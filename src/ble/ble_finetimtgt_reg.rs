#[doc = "Register `BLE_FINETIMTGT_REG` reader"]
pub struct R(crate::R<BLE_FINETIMTGT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_FINETIMTGT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_FINETIMTGT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_FINETIMTGT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_FINETIMTGT_REG` writer"]
pub struct W(crate::W<BLE_FINETIMTGT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_FINETIMTGT_REG_SPEC>;
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
impl From<crate::W<BLE_FINETIMTGT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_FINETIMTGT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINETARGET` reader - Fine Timer Target value on which a ble_finetgtim_irq must be generated. This timer has a precision of 625 usec: interrupt is generated only when FINETARGET = BASETIMECNT"]
pub struct FINETARGET_R(crate::FieldReader<u32, u32>);
impl FINETARGET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FINETARGET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINETARGET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINETARGET` writer - Fine Timer Target value on which a ble_finetgtim_irq must be generated. This timer has a precision of 625 usec: interrupt is generated only when FINETARGET = BASETIMECNT"]
pub struct FINETARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff_ffff) | (value as u32 & 0x07ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:26 - Fine Timer Target value on which a ble_finetgtim_irq must be generated. This timer has a precision of 625 usec: interrupt is generated only when FINETARGET = BASETIMECNT"]
    #[inline(always)]
    pub fn finetarget(&self) -> FINETARGET_R {
        FINETARGET_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:26 - Fine Timer Target value on which a ble_finetgtim_irq must be generated. This timer has a precision of 625 usec: interrupt is generated only when FINETARGET = BASETIMECNT"]
    #[inline(always)]
    pub fn finetarget(&mut self) -> FINETARGET_W {
        FINETARGET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fine Timer Target value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_finetimtgt_reg](index.html) module"]
pub struct BLE_FINETIMTGT_REG_SPEC;
impl crate::RegisterSpec for BLE_FINETIMTGT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_finetimtgt_reg::R](R) reader structure"]
impl crate::Readable for BLE_FINETIMTGT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_finetimtgt_reg::W](W) writer structure"]
impl crate::Writable for BLE_FINETIMTGT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_FINETIMTGT_REG to value 0"]
impl crate::Resettable for BLE_FINETIMTGT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
