#[doc = "Register `BLE_BASETIMECNT_REG` reader"]
pub struct R(crate::R<BLE_BASETIMECNT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_BASETIMECNT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_BASETIMECNT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_BASETIMECNT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_BASETIMECNT_REG` writer"]
pub struct W(crate::W<BLE_BASETIMECNT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_BASETIMECNT_REG_SPEC>;
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
impl From<crate::W<BLE_BASETIMECNT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_BASETIMECNT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASETIMECNT` reader - Value of the 625us base time reference counter. Updated each time SAMPCLK is written. Used by the SW in order to synchronize with the HW"]
pub struct BASETIMECNT_R(crate::FieldReader<u32, u32>);
impl BASETIMECNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BASETIMECNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASETIMECNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:26 - Value of the 625us base time reference counter. Updated each time SAMPCLK is written. Used by the SW in order to synchronize with the HW"]
    #[inline(always)]
    pub fn basetimecnt(&self) -> BASETIMECNT_R {
        BASETIMECNT_R::new((self.bits & 0x07ff_ffff) as u32)
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
#[doc = "Base time reference counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_basetimecnt_reg](index.html) module"]
pub struct BLE_BASETIMECNT_REG_SPEC;
impl crate::RegisterSpec for BLE_BASETIMECNT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_basetimecnt_reg::R](R) reader structure"]
impl crate::Readable for BLE_BASETIMECNT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_basetimecnt_reg::W](W) writer structure"]
impl crate::Writable for BLE_BASETIMECNT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_BASETIMECNT_REG to value 0"]
impl crate::Resettable for BLE_BASETIMECNT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
