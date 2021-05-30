use libc::*;
use *;

extern "C" {
    pub fn ENGINE_load_builtin_engines();
    pub fn ENGINE_by_id(id: *const c_char) -> *mut ENGINE;

    pub fn ENGINE_init(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_finish(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_free(e: *mut ENGINE) -> c_int;

    pub fn ENGINE_ctrl_cmd(
        e: *mut ENGINE,
        cmd_name: *const c_char,
        i: c_long,
        p: *mut c_void,
        f: extern "C" fn() -> (),
        cmd_optional: c_int,
    ) -> c_int;
    pub fn ENGINE_ctrl_cmd_string(
        e: *mut ENGINE,
        cmd_name: *const c_char,
        arg: *const c_char,
        cmd_optional: c_int,
    ) -> c_int;

    pub fn ENGINE_load_private_key(
        e: *mut ENGINE,
        key_id: *const c_char,
        ui_method: *mut UI_METHOD,
        callback_data: *mut c_void,
    ) -> *mut EVP_PKEY;
    pub fn ENGINE_load_public_key(
        e: *mut ENGINE,
        key_id: *const c_char,
        ui_method: *mut UI_METHOD,
        callback_data: *mut c_void,
    ) -> *mut EVP_PKEY;
    pub fn ENGINE_load_ssl_client_cert(
        e: *mut ENGINE,
        ssl: *mut SSL,
        ca_dn: *mut stack_st_X509_NAME,
        pcert: *mut *mut X509,
        ppkey: *mut *mut EVP_PKEY,
        pother: *mut *mut c_void,
        ui_method: *mut UI_METHOD,
        callback_data: *mut c_void,
    ) -> c_int;
}

#[repr(C)]
pub struct UI_METHOD {
    name: *const c_char,
    ui_open_session: extern "C" fn(ui: *mut UI) -> c_int,
    ui_write_string: extern "C" fn(ui: *mut UI, uis: *mut UI_STRING) -> c_int,
    ui_flush: extern "C" fn(ui: *mut UI) -> c_int,
    ui_read_string: extern "C" fn(ui: *mut UI, uis: *mut UI_STRING) -> c_int,
    ui_close_session: extern "C" fn(ui: *mut UI) -> c_int,
    ui_construct_prompt: extern "C" fn(
        ui: *mut UI,
        object_desc: *const c_char,
        object_name: *const c_char,
    ) -> *mut c_char,
}

const UI_FLAG_REDOABLE: c_int = 0x0001;
const UI_FLAG_PRINT_ERRORS: c_int = 0x0100;

#[repr(C)]
pub struct UI {
    meth: *const UI_METHOD,
    strings: *mut c_void,
    user_data: *mut c_void,
    ex_data: CRYPTO_EX_DATA,
    flags: c_int,
}

#[repr(C)]
pub struct UI_STRING {
    string_type: UI_string_types,
    out_string: *const c_char,
    input_flags: c_int,
    result_buf: *mut c_char,
}

#[repr(C)]
pub enum UI_string_types {
    UIT_NONE = 0,
    UIT_PROMPT,
    UIT_VERIFY,
    UIT_BOOLEAN,
    UIT_INFO,
    UIT_ERROR,
}
