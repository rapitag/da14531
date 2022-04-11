#[doc = "Register `RAM_PWR_CTRL_REG` reader"]
pub struct R(crate::R<RAM_PWR_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_PWR_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_PWR_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_PWR_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_PWR_CTRL_REG` writer"]
pub struct W(crate::W<RAM_PWR_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_PWR_CTRL_REG_SPEC>;
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
impl From<crate::W<RAM_PWR_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_PWR_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM3_PWR_CTRL` reader - See description of RAM1_PWR_CTRL."]
pub struct RAM3_PWR_CTRL_R(crate::FieldReader<u8, u8>);
impl RAM3_PWR_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM3_PWR_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM3_PWR_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM3_PWR_CTRL` writer - See description of RAM1_PWR_CTRL."]
pub struct RAM3_PWR_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM3_PWR_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u16 & 3) << 4);
        self.w
    }
}
#[doc = "Field `RAM2_PWR_CTRL` reader - See description of RAM1_PWR_CTRL."]
pub struct RAM2_PWR_CTRL_R(crate::FieldReader<u8, u8>);
impl RAM2_PWR_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM2_PWR_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM2_PWR_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM2_PWR_CTRL` writer - See description of RAM1_PWR_CTRL."]
pub struct RAM2_PWR_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2_PWR_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u16 & 3) << 2);
        self.w
    }
}
#[doc = "Field `RAM1_PWR_CTRL` reader - Power state control of the individual RAMs. May only change when the memory isn't accessed. When in Active or Sleep mode: 0x0: Normal operation 0x1: Normal operation 0x2: Retained (no access possible) 0x3: Off (memory content corrupted) When in Extended Sleep, Deep Sleep or Hibernation mode 0x0: Retained 0x1: Off (memory content corrupted) 0x2: Retained 0x3: Off (memory content corrupted)"]
pub struct RAM1_PWR_CTRL_R(crate::FieldReader<u8, u8>);
impl RAM1_PWR_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM1_PWR_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM1_PWR_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM1_PWR_CTRL` writer - Power state control of the individual RAMs. May only change when the memory isn't accessed. When in Active or Sleep mode: 0x0: Normal operation 0x1: Normal operation 0x2: Retained (no access possible) 0x3: Off (memory content corrupted) When in Extended Sleep, Deep Sleep or Hibernation mode 0x0: Retained 0x1: Off (memory content corrupted) 0x2: Retained 0x3: Off (memory content corrupted)"]
pub struct RAM1_PWR_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1_PWR_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram3_pwr_ctrl(&self) -> RAM3_PWR_CTRL_R {
        RAM3_PWR_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram2_pwr_ctrl(&self) -> RAM2_PWR_CTRL_R {
        RAM2_PWR_CTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Power state control of the individual RAMs. May only change when the memory isn't accessed. When in Active or Sleep mode: 0x0: Normal operation 0x1: Normal operation 0x2: Retained (no access possible) 0x3: Off (memory content corrupted) When in Extended Sleep, Deep Sleep or Hibernation mode 0x0: Retained 0x1: Off (memory content corrupted) 0x2: Retained 0x3: Off (memory content corrupted)"]
    #[inline(always)]
    pub fn ram1_pwr_ctrl(&self) -> RAM1_PWR_CTRL_R {
        RAM1_PWR_CTRL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram3_pwr_ctrl(&mut self) -> RAM3_PWR_CTRL_W {
        RAM3_PWR_CTRL_W { w: self }
    }
    #[doc = "Bits 2:3 - See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram2_pwr_ctrl(&mut self) -> RAM2_PWR_CTRL_W {
        RAM2_PWR_CTRL_W { w: self }
    }
    #[doc = "Bits 0:1 - Power state control of the individual RAMs. May only change when the memory isn't accessed. When in Active or Sleep mode: 0x0: Normal operation 0x1: Normal operation 0x2: Retained (no access possible) 0x3: Off (memory content corrupted) When in Extended Sleep, Deep Sleep or Hibernation mode 0x0: Retained 0x1: Off (memory content corrupted) 0x2: Retained 0x3: Off (memory content corrupted)"]
    #[inline(always)]
    pub fn ram1_pwr_ctrl(&mut self) -> RAM1_PWR_CTRL_W {
        RAM1_PWR_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control power state of System RAMS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_pwr_ctrl_reg](index.html) module"]
pub struct RAM_PWR_CTRL_REG_SPEC;
impl crate::RegisterSpec for RAM_PWR_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ram_pwr_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RAM_PWR_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_pwr_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RAM_PWR_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_PWR_CTRL_REG to value 0"]
impl crate::Resettable for RAM_PWR_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
