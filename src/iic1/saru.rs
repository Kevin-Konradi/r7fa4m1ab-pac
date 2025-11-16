#[doc = "Register `SARU%s` reader"]
pub type R = crate::R<SaruSpec>;
#[doc = "Register `SARU%s` writer"]
pub type W = crate::W<SaruSpec>;
#[doc = "7-Bit/10-Bit Address Format Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fs {
    #[doc = "0: The 7-bit address format is selected."]
    _0 = 0,
    #[doc = "1: The 10-bit address format is selected."]
    _1 = 1,
}
impl From<Fs> for bool {
    #[inline(always)]
    fn from(variant: Fs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FS` reader - 7-Bit/10-Bit Address Format Selection"]
pub type FsR = crate::BitReader<Fs>;
impl FsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fs {
        match self.bits {
            false => Fs::_0,
            true => Fs::_1,
        }
    }
    #[doc = "The 7-bit address format is selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fs::_0
    }
    #[doc = "The 10-bit address format is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fs::_1
    }
}
#[doc = "Field `FS` writer - 7-Bit/10-Bit Address Format Selection"]
pub type FsW<'a, REG> = crate::BitWriter<'a, REG, Fs>;
impl<'a, REG> FsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 7-bit address format is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fs::_0)
    }
    #[doc = "The 10-bit address format is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fs::_1)
    }
}
#[doc = "Field `SVA8` reader - 10-Bit Address(bit8)"]
pub type Sva8R = crate::BitReader;
#[doc = "Field `SVA8` writer - 10-Bit Address(bit8)"]
pub type Sva8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVA9` reader - 10-Bit Address(bit9)"]
pub type Sva9R = crate::BitReader;
#[doc = "Field `SVA9` writer - 10-Bit Address(bit9)"]
pub type Sva9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 7-Bit/10-Bit Address Format Selection"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 10-Bit Address(bit8)"]
    #[inline(always)]
    pub fn sva8(&self) -> Sva8R {
        Sva8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 10-Bit Address(bit9)"]
    #[inline(always)]
    pub fn sva9(&self) -> Sva9R {
        Sva9R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SARU")
            .field("sva9", &self.sva9())
            .field("sva8", &self.sva8())
            .field("fs", &self.fs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 7-Bit/10-Bit Address Format Selection"]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<SaruSpec> {
        FsW::new(self, 0)
    }
    #[doc = "Bit 1 - 10-Bit Address(bit8)"]
    #[inline(always)]
    pub fn sva8(&mut self) -> Sva8W<SaruSpec> {
        Sva8W::new(self, 1)
    }
    #[doc = "Bit 2 - 10-Bit Address(bit9)"]
    #[inline(always)]
    pub fn sva9(&mut self) -> Sva9W<SaruSpec> {
        Sva9W::new(self, 2)
    }
}
#[doc = "Slave Address Register U%s\n\nYou can [`read`](crate::Reg::read) this register and get [`saru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaruSpec;
impl crate::RegisterSpec for SaruSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`saru::R`](R) reader structure"]
impl crate::Readable for SaruSpec {}
#[doc = "`write(|w| ..)` method takes [`saru::W`](W) writer structure"]
impl crate::Writable for SaruSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SARU%s to value 0"]
impl crate::Resettable for SaruSpec {}
