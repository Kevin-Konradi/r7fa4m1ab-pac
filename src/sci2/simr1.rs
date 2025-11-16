#[doc = "Register `SIMR1` reader"]
pub type R = crate::R<Simr1Spec>;
#[doc = "Register `SIMR1` writer"]
pub type W = crate::W<Simr1Spec>;
#[doc = "Simple I2C Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicm {
    #[doc = "0: Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)"]
    _0 = 0,
    #[doc = "1: Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)"]
    _1 = 1,
}
impl From<Iicm> for bool {
    #[inline(always)]
    fn from(variant: Iicm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICM` reader - Simple I2C Mode Select"]
pub type IicmR = crate::BitReader<Iicm>;
impl IicmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicm {
        match self.bits {
            false => Iicm::_0,
            true => Iicm::_1,
        }
    }
    #[doc = "Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicm::_0
    }
    #[doc = "Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicm::_1
    }
}
#[doc = "Field `IICM` writer - Simple I2C Mode Select"]
pub type IicmW<'a, REG> = crate::BitWriter<'a, REG, Iicm>;
impl<'a, REG> IicmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicm::_0)
    }
    #[doc = "Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicm::_1)
    }
}
#[doc = "SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iicdl {
    #[doc = "0: No output delay"]
    _00000 = 0,
    #[doc = "1: (IICDL - 1 ) to IIDCDL cycles. The delay is in the clock cycles from the on-chip baud rate generator."]
    Others = 1,
}
impl From<Iicdl> for u8 {
    #[inline(always)]
    fn from(variant: Iicdl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iicdl {
    type Ux = u8;
}
impl crate::IsEnum for Iicdl {}
#[doc = "Field `IICDL` reader - SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
pub type IicdlR = crate::FieldReader<Iicdl>;
impl IicdlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicdl {
        match self.bits {
            0 => Iicdl::_00000,
            _ => Iicdl::Others,
        }
    }
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == Iicdl::_00000
    }
    #[doc = "(IICDL - 1 ) to IIDCDL cycles. The delay is in the clock cycles from the on-chip baud rate generator."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Iicdl::Others)
    }
}
#[doc = "Field `IICDL` writer - SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
pub type IicdlW<'a, REG> = crate::FieldWriter<'a, REG, 5, Iicdl, crate::Safe>;
impl<'a, REG> IicdlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(Iicdl::_00000)
    }
    #[doc = "(IICDL - 1 ) to IIDCDL cycles. The delay is in the clock cycles from the on-chip baud rate generator."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Iicdl::Others)
    }
}
impl R {
    #[doc = "Bit 0 - Simple I2C Mode Select"]
    #[inline(always)]
    pub fn iicm(&self) -> IicmR {
        IicmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:7 - SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
    #[inline(always)]
    pub fn iicdl(&self) -> IicdlR {
        IicdlR::new((self.bits >> 3) & 0x1f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIMR1")
            .field("iicdl", &self.iicdl())
            .field("iicm", &self.iicm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Simple I2C Mode Select"]
    #[inline(always)]
    pub fn iicm(&mut self) -> IicmW<Simr1Spec> {
        IicmW::new(self, 0)
    }
    #[doc = "Bits 3:7 - SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
    #[inline(always)]
    pub fn iicdl(&mut self) -> IicdlW<Simr1Spec> {
        IicdlW::new(self, 3)
    }
}
#[doc = "I2C Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`simr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simr1Spec;
impl crate::RegisterSpec for Simr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`simr1::R`](R) reader structure"]
impl crate::Readable for Simr1Spec {}
#[doc = "`write(|w| ..)` method takes [`simr1::W`](W) writer structure"]
impl crate::Writable for Simr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIMR1 to value 0"]
impl crate::Resettable for Simr1Spec {}
