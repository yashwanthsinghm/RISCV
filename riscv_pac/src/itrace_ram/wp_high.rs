#[doc = "Register `WP_HIGH` reader"]
pub type R = crate::R<WpHighSpec>;
#[doc = "Register `WP_HIGH` writer"]
pub type W = crate::W<WpHighSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Write pointer of trace packet into RAM. High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`wp_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpHighSpec;
impl crate::RegisterSpec for WpHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wp_high::R`](R) reader structure"]
impl crate::Readable for WpHighSpec {}
#[doc = "`write(|w| ..)` method takes [`wp_high::W`](W) writer structure"]
impl crate::Writable for WpHighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WP_HIGH to value 0"]
impl crate::Resettable for WpHighSpec {
    const RESET_VALUE: u32 = 0;
}
