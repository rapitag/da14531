#[doc = "Register `GP_ADC_CTRL2_REG` reader"]
pub struct R(crate::R<GP_ADC_CTRL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_ADC_CTRL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_ADC_CTRL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_ADC_CTRL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_ADC_CTRL2_REG` writer"]
pub struct W(crate::W<GP_ADC_CTRL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_ADC_CTRL2_REG_SPEC>;
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
impl From<crate::W<GP_ADC_CTRL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_ADC_CTRL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP_ADC_STORE_DEL` reader - 0: Data is stored after handshake synchronisation 1: Data is stored 2 ADC_CLK cycles after internal start trigger 7: Data is stored 8 ADC_CLK cycles after internal start trigger"]
pub struct GP_ADC_STORE_DEL_R(crate::FieldReader<u8, u8>);
impl GP_ADC_STORE_DEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_STORE_DEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_STORE_DEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_STORE_DEL` writer - 0: Data is stored after handshake synchronisation 1: Data is stored 2 ADC_CLK cycles after internal start trigger 7: Data is stored 8 ADC_CLK cycles after internal start trigger"]
pub struct GP_ADC_STORE_DEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_STORE_DEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 13)) | ((value as u16 & 7) << 13);
        self.w
    }
}
#[doc = "Field `GP_ADC_SMPL_TIME` reader - 0: The sample time (switch is closed) is two ADC_CLK cycles 1: The sample time is 1*8 ADC_CLK cycles 2: The sample time is 2*8 ADC_CLK cycles 15: The sample time is 15*8 ADC_CLK cycles"]
pub struct GP_ADC_SMPL_TIME_R(crate::FieldReader<u8, u8>);
impl GP_ADC_SMPL_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_SMPL_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_SMPL_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_SMPL_TIME` writer - 0: The sample time (switch is closed) is two ADC_CLK cycles 1: The sample time is 1*8 ADC_CLK cycles 2: The sample time is 2*8 ADC_CLK cycles 15: The sample time is 15*8 ADC_CLK cycles"]
pub struct GP_ADC_SMPL_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_SMPL_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u16 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `GP_ADC_CONV_NRS` reader - 0: 1 sample is taken or 2 in case ADC_CHOP is active. 1: 2 samples are taken. 2: 4 samples are taken. 7: 128 samples are taken."]
pub struct GP_ADC_CONV_NRS_R(crate::FieldReader<u8, u8>);
impl GP_ADC_CONV_NRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_CONV_NRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_CONV_NRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_CONV_NRS` writer - 0: 1 sample is taken or 2 in case ADC_CHOP is active. 1: 2 samples are taken. 2: 4 samples are taken. 7: 128 samples are taken."]
pub struct GP_ADC_CONV_NRS_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_CONV_NRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 6)) | ((value as u16 & 7) << 6);
        self.w
    }
}
#[doc = "Field `GP_ADC_OFFS_SH_CM` reader - Common mode adjust for offset shifter. Input range is CM +/- 450mV. 0: CM = 1.25V (Input range 0.80 - 1.70) 1: CM = 1.30V (Input range 0.85 - 1.75) (default) 2: CM = 1.35V (Input range 0.90 - 1.80) 3: CM = 1.40V (input range 0.95 - 1.85)"]
pub struct GP_ADC_OFFS_SH_CM_R(crate::FieldReader<u8, u8>);
impl GP_ADC_OFFS_SH_CM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_OFFS_SH_CM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_OFFS_SH_CM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_OFFS_SH_CM` writer - Common mode adjust for offset shifter. Input range is CM +/- 450mV. 0: CM = 1.25V (Input range 0.80 - 1.70) 1: CM = 1.30V (Input range 0.85 - 1.75) (default) 2: CM = 1.35V (Input range 0.90 - 1.80) 3: CM = 1.40V (input range 0.95 - 1.85)"]
pub struct GP_ADC_OFFS_SH_CM_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_OFFS_SH_CM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u16 & 3) << 4);
        self.w
    }
}
#[doc = "Field `GP_ADC_OFFS_SH_EN` reader - 0: Disable input shifter 1: Enable input shifter (900mV - 1800mV shifted to 0mV - 900mV)"]
pub struct GP_ADC_OFFS_SH_EN_R(crate::FieldReader<bool, bool>);
impl GP_ADC_OFFS_SH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_OFFS_SH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_OFFS_SH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_OFFS_SH_EN` writer - 0: Disable input shifter 1: Enable input shifter (900mV - 1800mV shifted to 0mV - 900mV)"]
pub struct GP_ADC_OFFS_SH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_OFFS_SH_EN_W<'a> {
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
#[doc = "Field `GP_ADC_I20U` reader - 1: Adds 20uA constant load current at the ADC LDO to minimize ripple on the reference voltage of the ADC."]
pub struct GP_ADC_I20U_R(crate::FieldReader<bool, bool>);
impl GP_ADC_I20U_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_I20U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_I20U_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_I20U` writer - 1: Adds 20uA constant load current at the ADC LDO to minimize ripple on the reference voltage of the ADC."]
pub struct GP_ADC_I20U_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_I20U_W<'a> {
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
#[doc = "Field `GP_ADC_ATTN` reader - 0: No attenuator (input voltages up to 0.9V allowed) 1: Enabling 2x attenuator (input voltages up to 1.8V allowed) 2: Enabling 3x attenuator (input voltages up to 2.7V allowed) 3: Enabling 4x attenuator (input voltages up to 3.6V allowed) Enabling the attenuator requires a longer sampling time."]
pub struct GP_ADC_ATTN_R(crate::FieldReader<u8, u8>);
impl GP_ADC_ATTN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_ATTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_ATTN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_ATTN` writer - 0: No attenuator (input voltages up to 0.9V allowed) 1: Enabling 2x attenuator (input voltages up to 1.8V allowed) 2: Enabling 3x attenuator (input voltages up to 2.7V allowed) 3: Enabling 4x attenuator (input voltages up to 3.6V allowed) Enabling the attenuator requires a longer sampling time."]
pub struct GP_ADC_ATTN_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_ATTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:15 - 0: Data is stored after handshake synchronisation 1: Data is stored 2 ADC_CLK cycles after internal start trigger 7: Data is stored 8 ADC_CLK cycles after internal start trigger"]
    #[inline(always)]
    pub fn gp_adc_store_del(&self) -> GP_ADC_STORE_DEL_R {
        GP_ADC_STORE_DEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 9:12 - 0: The sample time (switch is closed) is two ADC_CLK cycles 1: The sample time is 1*8 ADC_CLK cycles 2: The sample time is 2*8 ADC_CLK cycles 15: The sample time is 15*8 ADC_CLK cycles"]
    #[inline(always)]
    pub fn gp_adc_smpl_time(&self) -> GP_ADC_SMPL_TIME_R {
        GP_ADC_SMPL_TIME_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 6:8 - 0: 1 sample is taken or 2 in case ADC_CHOP is active. 1: 2 samples are taken. 2: 4 samples are taken. 7: 128 samples are taken."]
    #[inline(always)]
    pub fn gp_adc_conv_nrs(&self) -> GP_ADC_CONV_NRS_R {
        GP_ADC_CONV_NRS_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Common mode adjust for offset shifter. Input range is CM +/- 450mV. 0: CM = 1.25V (Input range 0.80 - 1.70) 1: CM = 1.30V (Input range 0.85 - 1.75) (default) 2: CM = 1.35V (Input range 0.90 - 1.80) 3: CM = 1.40V (input range 0.95 - 1.85)"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_cm(&self) -> GP_ADC_OFFS_SH_CM_R {
        GP_ADC_OFFS_SH_CM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - 0: Disable input shifter 1: Enable input shifter (900mV - 1800mV shifted to 0mV - 900mV)"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_en(&self) -> GP_ADC_OFFS_SH_EN_R {
        GP_ADC_OFFS_SH_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Adds 20uA constant load current at the ADC LDO to minimize ripple on the reference voltage of the ADC."]
    #[inline(always)]
    pub fn gp_adc_i20u(&self) -> GP_ADC_I20U_R {
        GP_ADC_I20U_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - 0: No attenuator (input voltages up to 0.9V allowed) 1: Enabling 2x attenuator (input voltages up to 1.8V allowed) 2: Enabling 3x attenuator (input voltages up to 2.7V allowed) 3: Enabling 4x attenuator (input voltages up to 3.6V allowed) Enabling the attenuator requires a longer sampling time."]
    #[inline(always)]
    pub fn gp_adc_attn(&self) -> GP_ADC_ATTN_R {
        GP_ADC_ATTN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 13:15 - 0: Data is stored after handshake synchronisation 1: Data is stored 2 ADC_CLK cycles after internal start trigger 7: Data is stored 8 ADC_CLK cycles after internal start trigger"]
    #[inline(always)]
    pub fn gp_adc_store_del(&mut self) -> GP_ADC_STORE_DEL_W {
        GP_ADC_STORE_DEL_W { w: self }
    }
    #[doc = "Bits 9:12 - 0: The sample time (switch is closed) is two ADC_CLK cycles 1: The sample time is 1*8 ADC_CLK cycles 2: The sample time is 2*8 ADC_CLK cycles 15: The sample time is 15*8 ADC_CLK cycles"]
    #[inline(always)]
    pub fn gp_adc_smpl_time(&mut self) -> GP_ADC_SMPL_TIME_W {
        GP_ADC_SMPL_TIME_W { w: self }
    }
    #[doc = "Bits 6:8 - 0: 1 sample is taken or 2 in case ADC_CHOP is active. 1: 2 samples are taken. 2: 4 samples are taken. 7: 128 samples are taken."]
    #[inline(always)]
    pub fn gp_adc_conv_nrs(&mut self) -> GP_ADC_CONV_NRS_W {
        GP_ADC_CONV_NRS_W { w: self }
    }
    #[doc = "Bits 4:5 - Common mode adjust for offset shifter. Input range is CM +/- 450mV. 0: CM = 1.25V (Input range 0.80 - 1.70) 1: CM = 1.30V (Input range 0.85 - 1.75) (default) 2: CM = 1.35V (Input range 0.90 - 1.80) 3: CM = 1.40V (input range 0.95 - 1.85)"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_cm(&mut self) -> GP_ADC_OFFS_SH_CM_W {
        GP_ADC_OFFS_SH_CM_W { w: self }
    }
    #[doc = "Bit 3 - 0: Disable input shifter 1: Enable input shifter (900mV - 1800mV shifted to 0mV - 900mV)"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_en(&mut self) -> GP_ADC_OFFS_SH_EN_W {
        GP_ADC_OFFS_SH_EN_W { w: self }
    }
    #[doc = "Bit 2 - 1: Adds 20uA constant load current at the ADC LDO to minimize ripple on the reference voltage of the ADC."]
    #[inline(always)]
    pub fn gp_adc_i20u(&mut self) -> GP_ADC_I20U_W {
        GP_ADC_I20U_W { w: self }
    }
    #[doc = "Bits 0:1 - 0: No attenuator (input voltages up to 0.9V allowed) 1: Enabling 2x attenuator (input voltages up to 1.8V allowed) 2: Enabling 3x attenuator (input voltages up to 2.7V allowed) 3: Enabling 4x attenuator (input voltages up to 3.6V allowed) Enabling the attenuator requires a longer sampling time."]
    #[inline(always)]
    pub fn gp_adc_attn(&mut self) -> GP_ADC_ATTN_W {
        GP_ADC_ATTN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose ADC Second Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_adc_ctrl2_reg](index.html) module"]
pub struct GP_ADC_CTRL2_REG_SPEC;
impl crate::RegisterSpec for GP_ADC_CTRL2_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_adc_ctrl2_reg::R](R) reader structure"]
impl crate::Readable for GP_ADC_CTRL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_adc_ctrl2_reg::W](W) writer structure"]
impl crate::Writable for GP_ADC_CTRL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_ADC_CTRL2_REG to value 0x0210"]
impl crate::Resettable for GP_ADC_CTRL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210
    }
}
