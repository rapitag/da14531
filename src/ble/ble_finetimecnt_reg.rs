#[doc = "Register `BLE_FINETIMECNT_REG` reader"]
pub struct R(crate::R<BLE_FINETIMECNT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_FINETIMECNT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_FINETIMECNT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_FINETIMECNT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_FINETIMECNT_REG` writer"]
pub struct W(crate::W<BLE_FINETIMECNT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_FINETIMECNT_REG_SPEC>;
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
impl From<crate::W<BLE_FINETIMECNT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_FINETIMECNT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINECNT` reader - Value of the current s fine time reference counter. Updated each time SAMPCLK is written. Used by the SW in order to synchronize with the HW, and obtain a more precise sleep duration"]
pub struct FINECNT_R(crate::FieldReader<u16, u16>);
impl FINECNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FINECNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINECNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Value of the current s fine time reference counter. Updated each time SAMPCLK is written. Used by the SW in order to synchronize with the HW, and obtain a more precise sleep duration"]
    #[inline(always)]
    pub fn finecnt(&self) -> FINECNT_R {
        FINECNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fine time reference counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_finetimecnt_reg](index.html) module"]
pub struct BLE_FINETIMECNT_REG_SPEC;
impl crate::RegisterSpec for BLE_FINETIMECNT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_finetimecnt_reg::R](R) reader structure"]
impl crate::Readable for BLE_FINETIMECNT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_finetimecnt_reg::W](W) writer structure"]
impl crate::Writable for BLE_FINETIMECNT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_FINETIMECNT_REG to value 0"]
impl crate::Resettable for BLE_FINETIMECNT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
