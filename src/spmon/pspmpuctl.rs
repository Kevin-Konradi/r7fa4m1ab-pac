#[doc = "Register `PSPMPUCTL` reader"]
pub type R = crate::R<PspmpuctlSpec>;
#[doc = "Register `PSPMPUCTL` writer"]
pub type W = crate::W<PspmpuctlSpec>;
#[doc = "Stack Pointer Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Stack pointer monitor is disabled"]
    _0 = 0,
    #[doc = "1: Stack pointer monitor is enabled"]
    _1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Stack Pointer Monitor Enable"]
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
    #[doc = "Stack pointer monitor is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enable::_0
    }
    #[doc = "Stack pointer monitor is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enable::_1
    }
}
#[doc = "Field `ENABLE` writer - Stack Pointer Monitor Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stack pointer monitor is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_0)
    }
    #[doc = "Stack pointer monitor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_1)
    }
}
#[doc = "Stack Pointer Monitor Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Stack pointer has not overflowed or underflowed"]
    _0 = 0,
    #[doc = "1: Stack pointer has overflowed or underflowed"]
    _1 = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Stack Pointer Monitor Error Flag"]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::_0,
            true => Error::_1,
        }
    }
    #[doc = "Stack pointer has not overflowed or underflowed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Error::_0
    }
    #[doc = "Stack pointer has overflowed or underflowed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Error::_1
    }
}
#[doc = "Field `ERROR` writer - Stack Pointer Monitor Error Flag"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, Error>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stack pointer has not overflowed or underflowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Error::_0)
    }
    #[doc = "Stack pointer has overflowed or underflowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Error::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Stack Pointer Monitor Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSPMPUCTL")
            .field("error", &self.error())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stack Pointer Monitor Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<PspmpuctlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 8 - Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<PspmpuctlSpec> {
        ErrorW::new(self, 8)
    }
}
#[doc = "Stack Pointer Monitor Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpuctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PspmpuctlSpec;
impl crate::RegisterSpec for PspmpuctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pspmpuctl::R`](R) reader structure"]
impl crate::Readable for PspmpuctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pspmpuctl::W`](W) writer structure"]
impl crate::Writable for PspmpuctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSPMPUCTL to value 0"]
impl crate::Resettable for PspmpuctlSpec {}
