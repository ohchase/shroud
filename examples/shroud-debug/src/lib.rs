use windows::Win32::{
    Foundation::{BOOL, HMODULE},
    System::{Console::AllocConsole, SystemServices::DLL_PROCESS_ATTACH},
};

unsafe extern "system" fn start_routine(_parameter: *mut std::ffi::c_void) -> u32 {
    // match shroud::directx9::methods() {
    //     Ok(m) => {
    //         println!("{m:#?}");
    //     }
    //     Err(e) => {
    //         println!("{e:?}");
    //     }
    // }

    // match shroud::directx11::methods() {
    //     Ok(m) => {
    //         println!("{m:#?}");
    //     }
    //     Err(e) => {
    //         println!("{e:?}");
    //     }
    // }

    match shroud::directx12::methods() {
        Ok(m) => {
            println!("{m:#?}");
        }
        Err(e) => {
            println!("{e:?}");
        }
    }

    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, _reserved: usize) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        unsafe { AllocConsole() };
        println!("Attached.");

        let thread = unsafe {
            windows::Win32::System::Threading::CreateThread(
                None,
                0,
                Some(start_routine),
                Some(dll_module.0 as *const std::ffi::c_void),
                windows::Win32::System::Threading::THREAD_CREATION_FLAGS(0),
                None,
            )
        };

        match thread {
            Ok(_handle) => {
                println!("Created thread")
            }
            Err(e) => {
                panic!("Unable to create thread {e:?}")
            }
        }
    }

    true.into()
}
