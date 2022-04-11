#[doc = "Register `RESET_STAT_REG` reader"]
pub struct R(crate::R<RESET_STAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_STAT_REG` writer"]
pub struct W(crate::W<RESET_STAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STAT_REG_SPEC>;
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
impl From<crate::W<RESET_STAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOGRESET_STAT` reader - Indicates that a Watchdog has happened. This bit is also set with a PowerOn Reset"]
pub struct WDOGRESET_STAT_R(crate::FieldReader<bool, bool>);
impl WDOGRESET_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOGRESET_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGRESET_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGRESET_STAT` writer - Indicates that a Watchdog has happened. This bit is also set with a PowerOn Reset"]
pub struct WDOGRESET_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGRESET_STAT_W<'a> {
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
#[doc = "Field `SWRESET_STAT` reader - Indicates that a SW Reset has been requested. The SW reset is requested by SYS_CTRL_REG\\[SW_RESET\\]
or SCB->AIRCR inside the ARM. This bit is also set with a PowerOn Reset"]
pub struct SWRESET_STAT_R(crate::FieldReader<bool, bool>);
impl SWRESET_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRESET_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRESET_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRESET_STAT` writer - Indicates that a SW Reset has been requested. The SW reset is requested by SYS_CTRL_REG\\[SW_RESET\\]
or SCB->AIRCR inside the ARM. This bit is also set with a PowerOn Reset"]
pub struct SWRESET_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRESET_STAT_W<'a> {
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
#[doc = "Field `HWRESET_STAT` reader - Indicates that a HW Reset has happened This bit is also set with a PowerOn Reset"]
pub struct HWRESET_STAT_R(crate::FieldReader<bool, bool>);
impl HWRESET_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HWRESET_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWRESET_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWRESET_STAT` writer - Indicates that a HW Reset has happened This bit is also set with a PowerOn Reset"]
pub struct HWRESET_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> HWRESET_STAT_W<'a> {
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
#[doc = "Field `PORESET_STAT` reader - Indicates that a PowerOn Reset has happened"]
pub struct PORESET_STAT_R(crate::FieldReader<bool, bool>);
impl PORESET_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORESET_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORESET_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORESET_STAT` writer - Indicates that a PowerOn Reset has happened"]
pub struct PORESET_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORESET_STAT_W<'a> {
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
    #[doc = "Bit 3 - Indicates that a Watchdog has happened. This bit is also set with a PowerOn Reset"]
    #[inline(always)]
    pub fn wdogreset_stat(&self) -> WDOGRESET_STAT_R {
        WDOGRESET_STAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that a SW Reset has been requested. The SW reset is requested by SYS_CTRL_REG\\[SW_RESET\\]
or SCB->AIRCR inside the ARM. This bit is also set with a PowerOn Reset"]
    #[inline(always)]
    pub fn swreset_stat(&self) -> SWRESET_STAT_R {
        SWRESET_STAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that a HW Reset has happened This bit is also set with a PowerOn Reset"]
    #[inline(always)]
    pub fn hwreset_stat(&self) -> HWRESET_STAT_R {
        HWRESET_STAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Indicates that a PowerOn Reset has happened"]
    #[inline(always)]
    pub fn poreset_stat(&self) -> PORESET_STAT_R {
        PORESET_STAT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Indicates that a Watchdog has happened. This bit is also set with a PowerOn Reset"]
    #[inline(always)]
    pub fn wdogreset_stat(&mut self) -> WDOGRESET_STAT_W {
        WDOGRESET_STAT_W { w: self }
    }
    #[doc = "Bit 2 - Indicates that a SW Reset has been requested. The SW reset is requested by SYS_CTRL_REG\\[SW_RESET\\]
or SCB->AIRCR inside the ARM. This bit is also set with a PowerOn Reset"]
    #[inline(always)]
    pub fn swreset_stat(&mut self) -> SWRESET_STAT_W {
        SWRESET_STAT_W { w: self }
    }
    #[doc = "Bit 1 - Indicates that a HW Reset has happened This bit is also set with a PowerOn Reset"]
    #[inline(always)]
    pub fn hwreset_stat(&mut self) -> HWRESET_STAT_W {
        HWRESET_STAT_W { w: self }
    }
    #[doc = "Bit 0 - Indicates that a PowerOn Reset has happened"]
    #[inline(always)]
    pub fn poreset_stat(&mut self) -> PORESET_STAT_W {
        PORESET_STAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_stat_reg](index.html) module"]
pub struct RESET_STAT_REG_SPEC;
impl crate::RegisterSpec for RESET_STAT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [reset_stat_reg::R](R) reader structure"]
impl crate::Readable for RESET_STAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_stat_reg::W](W) writer structure"]
impl crate::Writable for RESET_STAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_STAT_REG to value 0x0f"]
impl crate::Resettable for RESET_STAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
