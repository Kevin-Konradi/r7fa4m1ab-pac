#[doc = "Register `DCPCFG` reader"]
pub type R = crate::R<DcpcfgSpec>;
#[doc = "Register `DCPCFG` writer"]
pub type W = crate::W<DcpcfgSpec>;
#[doc = "Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Data receiving direction"]
    _0 = 0,
    #[doc = "1: Data transmitting direction"]
    _1 = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Transfer Direction"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::_0,
            true => Dir::_1,
        }
    }
    #[doc = "Data receiving direction"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dir::_0
    }
    #[doc = "Data transmitting direction"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dir::_1
    }
}
#[doc = "Field `DIR` writer - Transfer Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data receiving direction"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_0)
    }
    #[doc = "Data transmitting direction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_1)
    }
}
#[doc = "Pipe Disabled at End of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtnak {
    #[doc = "0: Pipe continued at the end of transfer"]
    _0 = 0,
    #[doc = "1: Pipe disabled at the end of transfer"]
    _1 = 1,
}
impl From<Shtnak> for bool {
    #[inline(always)]
    fn from(variant: Shtnak) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTNAK` reader - Pipe Disabled at End of Transfer"]
pub type ShtnakR = crate::BitReader<Shtnak>;
impl ShtnakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shtnak {
        match self.bits {
            false => Shtnak::_0,
            true => Shtnak::_1,
        }
    }
    #[doc = "Pipe continued at the end of transfer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Shtnak::_0
    }
    #[doc = "Pipe disabled at the end of transfer"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Shtnak::_1
    }
}
#[doc = "Field `SHTNAK` writer - Pipe Disabled at End of Transfer"]
pub type ShtnakW<'a, REG> = crate::BitWriter<'a, REG, Shtnak>;
impl<'a, REG> ShtnakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pipe continued at the end of transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Shtnak::_0)
    }
    #[doc = "Pipe disabled at the end of transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Shtnak::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub fn shtnak(&self) -> ShtnakR {
        ShtnakR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCPCFG")
            .field("shtnak", &self.shtnak())
            .field("dir", &self.dir())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<DcpcfgSpec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 7 - Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub fn shtnak(&mut self) -> ShtnakW<DcpcfgSpec> {
        ShtnakW::new(self, 7)
    }
}
#[doc = "DCP Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcpcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcpcfgSpec;
impl crate::RegisterSpec for DcpcfgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dcpcfg::R`](R) reader structure"]
impl crate::Readable for DcpcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcpcfg::W`](W) writer structure"]
impl crate::Writable for DcpcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCPCFG to value 0"]
impl crate::Resettable for DcpcfgSpec {}
