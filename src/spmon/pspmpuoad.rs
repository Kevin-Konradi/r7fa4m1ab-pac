#[doc = "Register `PSPMPUOAD` reader"]
pub type R = crate::R<PspmpuoadSpec>;
#[doc = "Register `PSPMPUOAD` writer"]
pub type W = crate::W<PspmpuoadSpec>;
#[doc = "Operation after detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oad {
    #[doc = "0: Non-maskable interrupt"]
    _0 = 0,
    #[doc = "1: Reset."]
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
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oad::_0
    }
    #[doc = "Reset."]
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
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_0)
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_1)
    }
}
#[doc = "Key Code The data written to these bits are not stored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "165: Writing to the OAD bit is valid, when the KEY bits are written 0xA5."]
    _0xA5 = 165,
    #[doc = "0: Writing to the OAD bit is invalid."]
    Others = 0,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Key Code The data written to these bits are not stored."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing to the OAD bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(Key::_0xA5)
    }
    #[doc = "Writing to the OAD bit is invalid."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Others)
    }
}
impl R {
    #[doc = "Bit 0 - Operation after detection"]
    #[inline(always)]
    pub fn oad(&self) -> OadR {
        OadR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSPMPUOAD")
            .field("oad", &self.oad())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Operation after detection"]
    #[inline(always)]
    pub fn oad(&mut self) -> OadW<PspmpuoadSpec> {
        OadW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Key Code The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<PspmpuoadSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Stack Pointer Monitor Operation After Detection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpuoad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuoad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PspmpuoadSpec;
impl crate::RegisterSpec for PspmpuoadSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pspmpuoad::R`](R) reader structure"]
impl crate::Readable for PspmpuoadSpec {}
#[doc = "`write(|w| ..)` method takes [`pspmpuoad::W`](W) writer structure"]
impl crate::Writable for PspmpuoadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSPMPUOAD to value 0"]
impl crate::Resettable for PspmpuoadSpec {}
