use crate::torrent::session_status;

use crate::torrent::torrent_status;

extern "C" {
    pub fn session_create(first_tag: ::std::os::raw::c_int, ...) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn session_close(ses: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn session_add_torrent(
        ses: *mut ::std::os::raw::c_void,
        first_tag: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn session_remove_torrent(
        ses: *mut ::std::os::raw::c_void,
        tor: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn session_pop_alert(
        ses: *mut ::std::os::raw::c_void,
        dest: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        category: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn session_get_status(
        ses: *mut ::std::os::raw::c_void,
        s: *mut session_status,
        struct_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn session_set_settings(
        ses: *mut ::std::os::raw::c_void,
        first_tag: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn session_get_setting(
        ses: *mut ::std::os::raw::c_void,
        tag: ::std::os::raw::c_int,
        value: *mut ::std::os::raw::c_void,
        value_size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn torrent_get_status(
        tor: ::std::os::raw::c_int,
        s: *mut torrent_status,
        struct_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn torrent_set_settings(
        tor: ::std::os::raw::c_int,
        first_tag: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn torrent_get_setting(
        tor: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        value: *mut ::std::os::raw::c_void,
        value_size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
