#[doc = "Register `FIDCR%s` reader"]
pub type R = crate::R<FidcrSpec>;
#[doc = "Register `FIDCR%s` writer"]
pub type W = crate::W<FidcrSpec>;
#[doc = "Field `EID` reader - Extended ID"]
pub type EidR = crate::FieldReader<u32>;
#[doc = "Field `EID` writer - Extended ID"]
pub type EidW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SID` reader - Standard ID"]
pub type SidR = crate::FieldReader<u16>;
#[doc = "Field `SID` writer - Standard ID"]
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Remote Transmission Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtr {
    #[doc = "0: Data frame"]
    _0 = 0,
    #[doc = "1: Remote frame"]
    _1 = 1,
}
impl From<Rtr> for bool {
    #[inline(always)]
    fn from(variant: Rtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR` reader - Remote Transmission Request"]
pub type RtrR = crate::BitReader<Rtr>;
impl RtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtr {
        match self.bits {
            false => Rtr::_0,
            true => Rtr::_1,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtr::_0
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtr::_1
    }
}
#[doc = "Field `RTR` writer - Remote Transmission Request"]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG, Rtr>;
impl<'a, REG> RtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtr::_0)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtr::_1)
    }
}
#[doc = "ID Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ide {
    #[doc = "0: Standard ID"]
    _0 = 0,
    #[doc = "1: Extended ID"]
    _1 = 1,
}
impl From<Ide> for bool {
    #[inline(always)]
    fn from(variant: Ide) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE` reader - ID Extension"]
pub type IdeR = crate::BitReader<Ide>;
impl IdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ide {
        match self.bits {
            false => Ide::_0,
            true => Ide::_1,
        }
    }
    #[doc = "Standard ID"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ide::_0
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ide::_1
    }
}
#[doc = "Field `IDE` writer - ID Extension"]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG, Ide>;
impl<'a, REG> IdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard ID"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ide::_0)
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ide::_1)
    }
}
impl R {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    pub fn eid(&self) -> EidR {
        EidR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Remote Transmission Request"]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ID Extension"]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIDCR")
            .field("ide", &self.ide())
            .field("rtr", &self.rtr())
            .field("sid", &self.sid())
            .field("eid", &self.eid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    pub fn eid(&mut self) -> EidW<FidcrSpec> {
        EidW::new(self, 0)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    pub fn sid(&mut self) -> SidW<FidcrSpec> {
        SidW::new(self, 18)
    }
    #[doc = "Bit 30 - Remote Transmission Request"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RtrW<FidcrSpec> {
        RtrW::new(self, 30)
    }
    #[doc = "Bit 31 - ID Extension"]
    #[inline(always)]
    pub fn ide(&mut self) -> IdeW<FidcrSpec> {
        IdeW::new(self, 31)
    }
}
#[doc = "FIFO Received ID Compare Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`fidcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fidcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FidcrSpec;
impl crate::RegisterSpec for FidcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fidcr::R`](R) reader structure"]
impl crate::Readable for FidcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fidcr::W`](W) writer structure"]
impl crate::Writable for FidcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIDCR%s to value 0"]
impl crate::Resettable for FidcrSpec {}
