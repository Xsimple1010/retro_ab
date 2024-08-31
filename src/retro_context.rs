use crate::core::CoreWrapperIns;
use crate::erro_handle::ErroHandle;
use crate::erro_handle::RetroLogLevel;
use crate::{core::CoreWrapper, environment::RetroEnvCallbacks, paths::Paths};
use std::ptr::addr_of;
use std::sync::Arc;
use uuid::Uuid;

static mut CONTEXTS: Vec<Arc<RetroContext>> = Vec::new();

pub type RetroCtxIns = Arc<RetroContext>;

pub struct RetroContext {
    pub id: Uuid,
    pub core: CoreWrapperIns,
}

impl RetroContext {
    pub fn new(
        core_path: &str,
        paths: Paths,
        callbacks: RetroEnvCallbacks,
    ) -> Result<RetroCtxIns, ErroHandle> {
        let id = Uuid::new_v4();

        let context = Arc::new(RetroContext {
            id,
            core: CoreWrapper::new(id, core_path, paths.clone(), callbacks)?,
        });

        context.core.init()?;

        unsafe {
            CONTEXTS.push(Arc::clone(&context));
        }

        Ok(context)
    }

    pub fn is_valid(&self) -> bool {
        let mut is_valide = false;

        unsafe {
            for ctx in &*addr_of!(CONTEXTS) {
                if ctx.id.eq(&self.id) {
                    is_valide = true;
                    break;
                }
            }
        }

        is_valide
    }

    pub fn delete(&self) -> Result<(), ErroHandle> {
        unsafe {
            let position = CONTEXTS.partition_point(|ctx| ctx.id == self.id);

            if !CONTEXTS.is_empty() {
                CONTEXTS.remove(position - 1);
            }
        };

        self.core.de_init()?;

        Ok(())
    }

    pub fn get_num_contexts() -> usize {
        unsafe { CONTEXTS.len() }
    }

    #[doc = "
        # Pegar uma instância pelo seu id

        Use isso com moderação, pois pode quasar muita confusão no código.

        ```
        // inicia pelo menos uma instância
        let ctx = RetroContext::new(core_path, paths, callbacks);

        let same_ctx = RetroContext::get_from_id(&ctx.id)

        if some_ctx.id == ctx.id {
            println!('same id: {:?}', some_ctx.id);
        }
        ```
    "]
    pub fn get_from_id(id: &Uuid) -> Result<RetroCtxIns, ErroHandle> {
        unsafe {
            for ctx in CONTEXTS.iter() {
                if ctx.id.eq(id) {
                    return Ok(ctx.clone());
                }
            }
        }

        Err(ErroHandle {
            message: "O contexto voce esta tentando acessar não existe".to_string(),
            level: RetroLogLevel::RETRO_LOG_ERROR,
        })
    }
}

#[cfg(test)]
mod retro_context {
    use crate::erro_handle::ErroHandle;
    use crate::test_tools::context::get_context;

    #[test]
    fn test_create_and_delete() -> Result<(), ErroHandle> {
        let ctx = get_context()?;

        assert_eq!(
            ctx.is_valid(),
            true,
            "O contexto id -> {:?} nao foi inicializado!",
            ctx.id
        );

        let current_id = ctx.id.clone();

        ctx.delete()?;

        assert_eq!(
            ctx.is_valid(),
            false,
            "O contexto id -> {:?} nao foi removido!",
            current_id
        );

        Ok(())
    }
}
