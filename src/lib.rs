#![allow(non_snake_case)]



use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*};
use csv::*;

//write the trampoline address here to return to the original function.
#[no_mangle]
static mut INFLATE_RETURN: usize = 0;
#[no_mangle]
static mut DEFLATE_RETURN: usize = 0;


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => detach(),
        _ => ()
    }

    true
}


#[no_mangle]
fn attach() {
    let _ = std::fs::File::create(std::env::var("USERPROFILE").unwrap() + "/Documents/GShovel/tmp/out.csv");
    //let _ = std::fs::File::create(std::env::var("USERPROFILE").unwrap() + "/Documents/GShovel/tmp/in.csv");
}
fn detach() {
    unsafe {
        // Create a message box
        //MessageBoxA(HWND(0),
        //    s!("Detaching"),
        //    s!("GameControl.dll"),
        //    Default::default()
        //);
    }
}

/*
zlib_stream {
    next_in: *[u8],
    available: u32,
    ...
}
*/
type ZStreamp = *mut (*mut u8, u32, usize, *mut u8, u32, usize);


#[no_mangle]
pub unsafe extern "C" fn deflate(stream: ZStreamp, flush: u32) {
    let f = std::fs::File::options().create(true).append(true).open(std::env::var("USERPROFILE").unwrap() + "/Documents/GShovel/tmp/out.csv").unwrap();
    let mut writer = Writer::from_writer(f);
    
    let (buf_addr, amt, _, _, _, _) = *stream;
    let buf = std::slice::from_raw_parts(buf_addr, amt as usize);
    
    writer.write_record(&[chrono::Utc::now().to_rfc3339().as_bytes(), buf]).expect("Error writing to data file.");
    let _ = writer.flush();

    if DEFLATE_RETURN != 0 {
        std::mem::transmute::<usize, fn(ZStreamp, u32)>(DEFLATE_RETURN)(stream, flush);
    }
}




/// DOES NOT WORK
/// Libmem is unable to hook the inflateEnd symbol for some reason.
/// Instead try 
#[no_mangle]
pub unsafe extern "C" fn inflateEnd(stream: ZStreamp, flush: u32) {
    //let f = std::fs::File::options().create(true).append(true).open(std::env::var("USERPROFILE").unwrap() + "/Documents/GShovel/tmp/in.csv").unwrap();
    //let mut writer = Writer::from_writer(f);

    //if INFLATE_RETURN != 0 {
    //    std::mem::transmute::<usize, fn(ZStreamp, u32)>(INFLATE_RETURN)(stream, flush);        
    //}
}


