use std::{
    ffi::CString,
    fs::File,
    io::Read,
    os::raw::c_void,
    path::{Path, PathBuf},
    ptr::null,
};
use crate::{
    binding::binding_libretro::retro_game_info,
    erro_handle::{ErroHandle, RetroLogLevel},
};
use crate::core::CoreWrapper;
use super::ffi_tools::make_c_string;

fn get_full_path(path: &str) -> Result<PathBuf, ErroHandle> {
    match PathBuf::from(path).canonicalize() {
        Ok(full_path) => Ok(full_path),
        Err(e) => Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: e.to_string(),
        }),
    }
}

fn valid_rom_extension(ctx: &CoreWrapper, path: &Path) -> Result<(), ErroHandle> {
    let valid_extensions = ctx.system.info.valid_extensions.lock().unwrap();
    let path_str = path.extension().unwrap().to_str().unwrap();

    if !valid_extensions.contains(path_str) {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: "Extensão da rom invalida: valores esperados -> ".to_string()
                + &valid_extensions.to_string()
                + "; valor recebido -> "
                + path_str,
        });
    };

    Ok(())
}

pub fn create_game_info(
    ctx: &CoreWrapper,
    path: &str,
) -> Result<bool, ErroHandle> {
    let f_path = get_full_path(path)?;

    valid_rom_extension(ctx, &f_path)?;

    let mut buf = Vec::new();
    let meta = CString::new("").unwrap();
    let path = make_c_string(f_path.to_str().unwrap())?;
    let mut size = 0;

    let need_full_path = *ctx.system.info.need_full_path.lock().unwrap();

    if !need_full_path {
        let mut file = File::open(f_path).unwrap();

        size = file.metadata().unwrap().len() as usize;

        buf = Vec::with_capacity(size);

        file.read_to_end(&mut buf).unwrap();
    }

    let game_info = retro_game_info {
        data: if buf.is_empty() {
            null()
        } else {
            buf.as_ptr() as *const c_void
        },
        meta: meta.as_ptr(),
        path: path.as_ptr(),
        size,
    };

    let state = unsafe { ctx.raw.retro_load_game(&game_info) };

    Ok(state)
}
