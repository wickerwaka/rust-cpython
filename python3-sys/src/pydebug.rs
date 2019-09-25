use libc::c_int;

#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name="pythonXY"))] extern "C" {
    pub static mut Py_DebugFlag: c_int;
    pub static mut Py_VerboseFlag: c_int;
    pub static mut Py_QuietFlag: c_int;
    pub static mut Py_InteractiveFlag: c_int;
    pub static mut Py_InspectFlag: c_int;
    pub static mut Py_OptimizeFlag: c_int;
    pub static mut Py_NoSiteFlag: c_int;
    pub static mut Py_BytesWarningFlag: c_int;
    #[cfg(not(Py_3_7))]
    pub static mut Py_UseClassExceptionsFlag: c_int;
    pub static mut Py_FrozenFlag: c_int;
    pub static mut Py_IgnoreEnvironmentFlag: c_int;
    pub static mut Py_DontWriteBytecodeFlag: c_int;
    pub static mut Py_NoUserSiteDirectory: c_int;
    pub static mut Py_UnbufferedStdioFlag: c_int;
    pub static mut Py_HashRandomizationFlag: c_int;
    #[cfg(Py_3_4)]
    pub static mut Py_IsolatedFlag: c_int;
    #[cfg(all(Py_3_7, windows))]
    pub static mut Py_LegacyWindowsFSEncodingFlag: c_int;
    #[cfg(all(Py_3_6, windows))]
    pub static mut Py_LegacyWindowsStdioFlag: c_int;
}

