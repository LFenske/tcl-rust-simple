
use ::libc;
use std::ptr;

pub type TclCmdProc = extern "C" fn (cd:*mut libc::c_void, interp:*mut libc::c_void,
                                      argc: ::libc::c_int, argv: *mut *const ::libc::c_char) -> ::libc::c_int;

#[link(name = "tcl8.6")]
extern {
    pub fn Tcl_CreateCommand(interp:*mut libc::c_void, cmdName: *const ::libc::c_char,
                             _proc: TclCmdProc, cd:*mut libc::c_void, dp:*mut libc::c_void) -> *mut libc::c_void;
}

/*
If my dynamic library is called foo.dll then Tcl expects the entry function name to be: Foo_Init
This function takes care of registering my extension with the Tcl interpreter
*/
#[allow(non_snake_case)]
#[no_mangle]
pub extern fn Foo_Init(interp: *mut libc::c_void) -> ::libc::c_int {
    /*
    This creates a new Tcl command, which can be invoked
    mycmd str-0 str-1 ... str-n
    When this command is invoked from a Tcl script, it will call the my_command function defined below
    It will pass all the trailing whitespace-separated strings as an array
    */
    unsafe { Tcl_CreateCommand(interp, "mycmd\0".as_ptr() as *const i8,
                               my_command, ptr::null_mut() , ptr::null_mut()); }
    
    0 // This tells Tcl everything ended OK
}

/*
Function that will get called by the Tcl interpreter whenever mycmd is invoked
*/
extern "C" fn my_command(_:*mut libc::c_void, _interp: *mut libc::c_void,
                        _argc: ::libc::c_int, _argv: *mut *const ::libc::c_char) -> ::libc::c_int {
    println!("rust from tcl");
    
    0 // This tells Tcl everything ended OK
}
