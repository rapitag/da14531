#[doc = "Register `BLE_BDADDRL_REG` reader"]
pub struct R(crate::R<BLE_BDADDRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_BDADDRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_BDADDRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_BDADDRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_BDADDRL_REG` writer"]
pub struct W(crate::W<BLE_BDADDRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_BDADDRL_REG_SPEC>;
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
impl From<crate::W<BLE_BDADDRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_BDADDRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDADDRL` reader - Bluetooth Low Energy Device Address. LSB part."]
pub struct BDADDRL_R(crate::FieldReader<u32, u32>);
impl BDADDRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BDADDRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDADDRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDADDRL` writer - Bluetooth Low Energy Device Address. LSB part."]
pub struct BDADDRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BDADDRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bluetooth Low Energy Device Address. LSB part."]
    #[inline(always)]
    pub fn bdaddrl(&self) -> BDADDRL_R {
        BDADDRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bluetooth Low Energy Device Address. LSB part."]
    #[inline(always)]
    pub fn bdaddrl(&mut self) -> BDADDRL_W {
        BDADDRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE device address LSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_bdaddrl_reg](index.html) module"]
pub struct BLE_BDADDRL_REG_SPEC;
impl crate::RegisterSpec for BLE_BDADDRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_bdaddrl_reg::R](R) reader structure"]
impl crate::Readable for BLE_BDADDRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_bdaddrl_reg::W](W) writer structure"]
impl crate::Writable for BLE_BDADDRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_BDADDRL_REG to value 0"]
impl crate::Resettable for BLE_BDADDRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
