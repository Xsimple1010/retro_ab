use crate::{
    core::{CoreWrapperIns, RetroEnvCallbacks},
    erro_handle::ErroHandle,
    paths::Paths,
    retro_context::{RetroContext, RetroCtxIns},
};

pub struct RetroAB {
    retro_ctx: RetroCtxIns,
}

impl Drop for RetroAB {
    fn drop(&mut self) {
        self.retro_ctx.delete().unwrap();
    }
}

impl RetroAB {
    pub fn new(
        core_path: &str,
        paths: Paths,
        callbacks: RetroEnvCallbacks,
    ) -> Result<Self, ErroHandle> {
        Ok(RetroAB {
            retro_ctx: RetroContext::new(core_path, paths, callbacks)?,
        })
    }

    pub fn core(&self) -> CoreWrapperIns {
        self.retro_ctx.core.clone()
    }
}
