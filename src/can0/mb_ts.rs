#[doc = "Register `MB%s_TS` reader"]
pub type R = crate::R<MbTsSpec>;
#[doc = "Register `MB%s_TS` writer"]
pub type W = crate::W<MbTsSpec>;
#[doc = "Field `TSL` reader - Time Stamp Higher Byte Bits TSL\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TslR = crate::FieldReader;
#[doc = "Field `TSL` writer - Time Stamp Higher Byte Bits TSL\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TslW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TSH` reader - Time Stamp Lower Byte Bits TSH\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TshR = crate::FieldReader;
#[doc = "Field `TSH` writer - Time Stamp Lower Byte Bits TSH\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TshW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Higher Byte Bits TSL\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsl(&self) -> TslR {
        TslR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Lower Byte Bits TSH\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsh(&self) -> TshR {
        TshR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_TS")
            .field("tsh", &self.tsh())
            .field("tsl", &self.tsl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Stamp Higher Byte Bits TSL\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsl(&mut self) -> TslW<MbTsSpec> {
        TslW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Time Stamp Lower Byte Bits TSH\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsh(&mut self) -> TshW<MbTsSpec> {
        TshW::new(self, 8)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_ts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_ts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbTsSpec;
impl crate::RegisterSpec for MbTsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mb_ts::R`](R) reader structure"]
impl crate::Readable for MbTsSpec {}
#[doc = "`write(|w| ..)` method takes [`mb_ts::W`](W) writer structure"]
impl crate::Writable for MbTsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_TS to value 0"]
impl crate::Resettable for MbTsSpec {}
