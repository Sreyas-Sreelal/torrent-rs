#[repr(C)]
pub enum ProxyType{
	NONE,
	Socks4,
	Socks5,
	Socks5Pw,
	Http,
	HttpPw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proxy_setting {
    pub hostname: [::std::os::raw::c_char; 256usize],
    pub port: ::std::os::raw::c_int,
    pub username: [::std::os::raw::c_char; 256usize],
    pub password: [::std::os::raw::c_char; 256usize],
    pub type_: ::std::os::raw::c_int,
}

#[test]
fn bindgen_test_layout_proxy_setting() {
    assert_eq!(
        ::std::mem::size_of::<proxy_setting>(),
        776usize,
        concat!("Size of: ", stringify!(proxy_setting))
    );
    assert_eq!(
        ::std::mem::align_of::<proxy_setting>(),
        4usize,
        concat!("Alignment of ", stringify!(proxy_setting))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).hostname as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(hostname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).port as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).username as *const _ as usize },
        260usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(username)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).password as *const _ as usize },
        516usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(password)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).type_ as *const _ as usize },
        772usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(type_)
        )
    );
}