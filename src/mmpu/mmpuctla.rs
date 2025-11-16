#[doc = "Register `MMPUCTLA` reader"]
pub type R = crate::R<MmpuctlaSpec>;
#[doc = "Register `MMPUCTLA` writer"]
pub type W = crate::W<MmpuctlaSpec>;
#[doc = "Master Group enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Master Group A disabled"]
    _0 = 0,
    #[doc = "1: Master Group A enabled."]
    _1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Master Group enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::_0,
            true => Enable::_1,
        }
    }
    #[doc = "Master Group A disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enable::_0
    }
    #[doc = "Master Group A enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enable::_1
    }
}
#[doc = "Field `ENABLE` writer - Master Group enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master Group A disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_0)
    }
    #[doc = "Master Group A enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_1)
    }
}
#[doc = "Operation after detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oad {
    #[doc = "0: Non-maskable interrupt."]
    _0 = 0,
    #[doc = "1: Internal reset."]
    _1 = 1,
}
impl From<Oad> for bool {
    #[inline(always)]
    fn from(variant: Oad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAD` reader - Operation after detection"]
pub type OadR = crate::BitReader<Oad>;
impl OadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oad {
        match self.bits {
            false => Oad::_0,
            true => Oad::_1,
        }
    }
    #[doc = "Non-maskable interrupt."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oad::_0
    }
    #[doc = "Internal reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oad::_1
    }
}
#[doc = "Field `OAD` writer - Operation after detection"]
pub type OadW<'a, REG> = crate::BitWriter<'a, REG, Oad>;
impl<'a, REG> OadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_0)
    }
    #[doc = "Internal reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code These bits are used to enable or disable writing of the OAD and ENABLE bit."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation after detection"]
    #[inline(always)]
    pub fn oad(&self) -> OadR {
        OadR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUCTLA")
            .field("oad", &self.oad())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<MmpuctlaSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Operation after detection"]
    #[inline(always)]
    pub fn oad(&mut self) -> OadW<MmpuctlaSpec> {
        OadW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Key Code These bits are used to enable or disable writing of the OAD and ENABLE bit."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MmpuctlaSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Bus Master MPU Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpuctla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuctla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmpuctlaSpec;
impl crate::RegisterSpec for MmpuctlaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mmpuctla::R`](R) reader structure"]
impl crate::Readable for MmpuctlaSpec {}
#[doc = "`write(|w| ..)` method takes [`mmpuctla::W`](W) writer structure"]
impl crate::Writable for MmpuctlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMPUCTLA to value 0"]
impl crate::Resettable for MmpuctlaSpec {}
