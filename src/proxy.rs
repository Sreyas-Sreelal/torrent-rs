#[repr(C)]
pub enum ProxyType {
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
