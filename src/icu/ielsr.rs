#[doc = "Register `IELSR%s` reader"]
pub type R = crate::R<IelsrSpec>;
#[doc = "Register `IELSR%s` writer"]
pub type W = crate::W<IelsrSpec>;
#[doc = "ICU Event selection to NVIC Set the number for the event signal to be linked .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iels {
    #[doc = "0: Nothing is selected"]
    _0x000 = 0,
    #[doc = "1: See Event Table"]
    Others = 1,
}
impl From<Iels> for u8 {
    #[inline(always)]
    fn from(variant: Iels) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iels {
    type Ux = u8;
}
impl crate::IsEnum for Iels {}
#[doc = "Field `IELS` reader - ICU Event selection to NVIC Set the number for the event signal to be linked ."]
pub type IelsR = crate::FieldReader<Iels>;
impl IelsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iels {
        match self.bits {
            0 => Iels::_0x000,
            _ => Iels::Others,
        }
    }
    #[doc = "Nothing is selected"]
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == Iels::_0x000
    }
    #[doc = "See Event Table"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Iels::Others)
    }
}
#[doc = "Field `IELS` writer - ICU Event selection to NVIC Set the number for the event signal to be linked ."]
pub type IelsW<'a, REG> = crate::FieldWriter<'a, REG, 8, Iels, crate::Safe>;
impl<'a, REG> IelsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing is selected"]
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut crate::W<REG> {
        self.variant(Iels::_0x000)
    }
    #[doc = "See Event Table"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Iels::Others)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ir {
    #[doc = "0: No interrupt request is generated"]
    _0 = 0,
    #[doc = "1: An interrupt request is generated ( 1 write to the IR bit is prohibited. )"]
    _1 = 1,
}
impl From<Ir> for bool {
    #[inline(always)]
    fn from(variant: Ir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IR` reader - Interrupt Status Flag"]
pub type IrR = crate::BitReader<Ir>;
impl IrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ir {
        match self.bits {
            false => Ir::_0,
            true => Ir::_1,
        }
    }
    #[doc = "No interrupt request is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ir::_0
    }
    #[doc = "An interrupt request is generated ( 1 write to the IR bit is prohibited. )"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ir::_1
    }
}
#[doc = "Field `IR` writer - Interrupt Status Flag"]
pub type IrW<'a, REG> = crate::BitWriter<'a, REG, Ir>;
impl<'a, REG> IrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt request is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ir::_0)
    }
    #[doc = "An interrupt request is generated ( 1 write to the IR bit is prohibited. )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ir::_1)
    }
}
#[doc = "DTC Activation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtce {
    #[doc = "0: DTC activation is disabled"]
    _0 = 0,
    #[doc = "1: DTC activation is enabled"]
    _1 = 1,
}
impl From<Dtce> for bool {
    #[inline(always)]
    fn from(variant: Dtce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCE` reader - DTC Activation Enable"]
pub type DtceR = crate::BitReader<Dtce>;
impl DtceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtce {
        match self.bits {
            false => Dtce::_0,
            true => Dtce::_1,
        }
    }
    #[doc = "DTC activation is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtce::_0
    }
    #[doc = "DTC activation is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtce::_1
    }
}
#[doc = "Field `DTCE` writer - DTC Activation Enable"]
pub type DtceW<'a, REG> = crate::BitWriter<'a, REG, Dtce>;
impl<'a, REG> DtceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTC activation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtce::_0)
    }
    #[doc = "DTC activation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtce::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - ICU Event selection to NVIC Set the number for the event signal to be linked ."]
    #[inline(always)]
    pub fn iels(&self) -> IelsR {
        IelsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn ir(&self) -> IrR {
        IrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - DTC Activation Enable"]
    #[inline(always)]
    pub fn dtce(&self) -> DtceR {
        DtceR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IELSR")
            .field("dtce", &self.dtce())
            .field("ir", &self.ir())
            .field("iels", &self.iels())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - ICU Event selection to NVIC Set the number for the event signal to be linked ."]
    #[inline(always)]
    pub fn iels(&mut self) -> IelsW<IelsrSpec> {
        IelsW::new(self, 0)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn ir(&mut self) -> IrW<IelsrSpec> {
        IrW::new(self, 16)
    }
    #[doc = "Bit 24 - DTC Activation Enable"]
    #[inline(always)]
    pub fn dtce(&mut self) -> DtceW<IelsrSpec> {
        DtceW::new(self, 24)
    }
}
#[doc = "ICU Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ielsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IelsrSpec;
impl crate::RegisterSpec for IelsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ielsr::R`](R) reader structure"]
impl crate::Readable for IelsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ielsr::W`](W) writer structure"]
impl crate::Writable for IelsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IELSR%s to value 0"]
impl crate::Resettable for IelsrSpec {}
