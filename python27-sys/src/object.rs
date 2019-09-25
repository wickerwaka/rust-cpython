use core::ptr;
use libc::{c_void, c_int, c_uint, c_long, c_char, c_double, FILE};
use pyport::Py_ssize_t;
use methodobject::PyMethodDef;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyObject {
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyVarObject {
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub ob_size: Py_ssize_t,
}

#[inline(always)]
pub unsafe fn Py_REFCNT(ob : *mut PyObject) -> Py_ssize_t {
    (*ob).ob_refcnt
}

#[inline(always)]
pub unsafe fn Py_TYPE(ob : *mut PyObject) -> *mut PyTypeObject {
    (*ob).ob_type
}

#[inline(always)]
pub unsafe fn Py_SIZE(ob : *mut PyObject) -> Py_ssize_t {
    (*(ob as *mut PyVarObject)).ob_size
}

pub type unaryfunc =
    unsafe extern "C" fn(arg1: *mut PyObject)
                              -> *mut PyObject;
pub type binaryfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject)
                              -> *mut PyObject;
pub type ternaryfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject,
                               arg3: *mut PyObject) -> *mut PyObject;
pub type inquiry =
    unsafe extern "C" fn(arg1: *mut PyObject)
                              -> c_int;
pub type lenfunc =
    unsafe extern "C" fn(arg1: *mut PyObject) -> Py_ssize_t;
pub type coercion =
    unsafe extern "C" fn
                              (arg1: *mut *mut PyObject,
                               arg2: *mut *mut PyObject) -> c_int;
pub type ssizeargfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t)
                              -> *mut PyObject;
pub type ssizessizeargfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: Py_ssize_t,
                               arg3: Py_ssize_t) -> *mut PyObject;
pub type intobjargproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: c_int,
                               arg3: *mut PyObject) -> c_int;
pub type intintobjargproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: c_int,
                               arg3: c_int, arg4: *mut PyObject)
                              -> c_int;
pub type ssizeobjargproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: Py_ssize_t,
                               arg3: *mut PyObject) -> c_int;
pub type ssizessizeobjargproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: Py_ssize_t,
                               arg3: Py_ssize_t, arg4: *mut PyObject)
                              -> c_int;
pub type objobjargproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject,
                               arg3: *mut PyObject) -> c_int;
pub type getreadbufferproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: c_int,
                               arg3: *mut *mut c_void)
                              -> c_int;
pub type getwritebufferproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: c_int,
                               arg3: *mut *mut c_void)
                              -> c_int;
pub type getsegcountproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut c_int)
                              -> c_int;
pub type getcharbufferproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: c_int,
                               arg3: *mut *mut c_char)
                              -> c_int;
pub type readbufferproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: Py_ssize_t,
                               arg3: *mut *mut c_void) -> Py_ssize_t;
pub type writebufferproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: Py_ssize_t,
                               arg3: *mut *mut c_void) -> Py_ssize_t;
pub type segcountproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut Py_ssize_t)
                              -> Py_ssize_t;
pub type charbufferproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: Py_ssize_t,
                               arg3: *mut *mut c_char) -> Py_ssize_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Py_buffer {
    pub buf: *mut c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: c_int,
    pub ndim: c_int,
    pub format: *mut c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub smalltable: [Py_ssize_t; 2],
    pub internal: *mut c_void,
}

pub type getbufferproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut Py_buffer,
                               arg3: c_int) -> c_int;
pub type releasebufferproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut Py_buffer);

// flags:
pub const PyBUF_SIMPLE : c_int = 0;
pub const PyBUF_WRITABLE : c_int = 0x0001;
pub const PyBUF_FORMAT : c_int = 0x0004;
pub const PyBUF_ND : c_int = 0x0008;
pub const PyBUF_STRIDES : c_int = (0x0010 | PyBUF_ND);
pub const PyBUF_C_CONTIGUOUS : c_int = (0x0020 | PyBUF_STRIDES);
pub const PyBUF_F_CONTIGUOUS : c_int = (0x0040 | PyBUF_STRIDES);
pub const PyBUF_ANY_CONTIGUOUS : c_int = (0x0080 | PyBUF_STRIDES);
pub const PyBUF_INDIRECT : c_int = (0x0100 | PyBUF_STRIDES);

pub const PyBUF_CONTIG : c_int = (PyBUF_ND | PyBUF_WRITABLE);
pub const PyBUF_CONTIG_RO : c_int = (PyBUF_ND);

pub const PyBUF_STRIDED : c_int = (PyBUF_STRIDES | PyBUF_WRITABLE);
pub const PyBUF_STRIDED_RO : c_int = (PyBUF_STRIDES);

pub const PyBUF_RECORDS : c_int = (PyBUF_STRIDES | PyBUF_WRITABLE | PyBUF_FORMAT);
pub const PyBUF_RECORDS_RO : c_int = (PyBUF_STRIDES | PyBUF_FORMAT);

pub const PyBUF_FULL : c_int = (PyBUF_INDIRECT | PyBUF_WRITABLE | PyBUF_FORMAT);
pub const PyBUF_FULL_RO : c_int = (PyBUF_INDIRECT | PyBUF_FORMAT);


// buffertype:
pub const PyBUF_READ : c_int = 0x100;
pub const PyBUF_WRITE : c_int = 0x200;
pub const PyBUF_SHADOW : c_int = 0x400;



pub type objobjproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject)
                              -> c_int;
pub type visitproc =
    unsafe extern "C" fn
                              (object: *mut PyObject, arg: *mut c_void)
                              -> c_int;
pub type traverseproc =
    unsafe extern "C" fn
                              (slf: *mut PyObject, visit: visitproc,
                               arg: *mut c_void) -> c_int;


#[repr(C)]
#[derive(Copy)]
pub struct PyNumberMethods {
    pub nb_add: Option<binaryfunc>,
    pub nb_subtract: Option<binaryfunc>,
    pub nb_multiply: Option<binaryfunc>,
    pub nb_divide: Option<binaryfunc>,
    pub nb_remainder: Option<binaryfunc>,
    pub nb_divmod: Option<binaryfunc>,
    pub nb_power: Option<ternaryfunc>,
    pub nb_negative: Option<unaryfunc>,
    pub nb_positive: Option<unaryfunc>,
    pub nb_absolute: Option<unaryfunc>,
    pub nb_nonzero: Option<inquiry>,
    pub nb_invert: Option<unaryfunc>,
    pub nb_lshift: Option<binaryfunc>,
    pub nb_rshift: Option<binaryfunc>,
    pub nb_and: Option<binaryfunc>,
    pub nb_xor: Option<binaryfunc>,
    pub nb_or: Option<binaryfunc>,
    pub nb_coerce: Option<coercion>,
    pub nb_c_int: Option<unaryfunc>,
    pub nb_long: Option<unaryfunc>,
    pub nb_float: Option<unaryfunc>,
    pub nb_oct: Option<unaryfunc>,
    pub nb_hex: Option<unaryfunc>,
    pub nb_inplace_add: Option<binaryfunc>,
    pub nb_inplace_subtract: Option<binaryfunc>,
    pub nb_inplace_multiply: Option<binaryfunc>,
    pub nb_inplace_divide: Option<binaryfunc>,
    pub nb_inplace_remainder: Option<binaryfunc>,
    pub nb_inplace_power: Option<ternaryfunc>,
    pub nb_inplace_lshift: Option<binaryfunc>,
    pub nb_inplace_rshift: Option<binaryfunc>,
    pub nb_inplace_and: Option<binaryfunc>,
    pub nb_inplace_xor: Option<binaryfunc>,
    pub nb_inplace_or: Option<binaryfunc>,
    pub nb_floor_divide: Option<binaryfunc>,
    pub nb_true_divide: Option<binaryfunc>,
    pub nb_inplace_floor_divide: Option<binaryfunc>,
    pub nb_inplace_true_divide: Option<binaryfunc>,
    pub nb_index: Option<unaryfunc>,
}

impl Clone for PyNumberMethods {
    #[inline] fn clone(&self) -> PyNumberMethods { *self }
}

pub const PyNumberMethods_INIT : PyNumberMethods = PyNumberMethods {
    nb_add: None,
    nb_subtract: None,
    nb_multiply: None,
    nb_divide: None,
    nb_remainder: None,
    nb_divmod: None,
    nb_power: None,
    nb_negative: None,
    nb_positive: None,
    nb_absolute: None,
    nb_nonzero: None,
    nb_invert: None,
    nb_lshift: None,
    nb_rshift: None,
    nb_and: None,
    nb_xor: None,
    nb_or: None,
    nb_coerce: None,
    nb_c_int: None,
    nb_long: None,
    nb_float: None,
    nb_oct: None,
    nb_hex: None,
    nb_inplace_add: None,
    nb_inplace_subtract: None,
    nb_inplace_multiply: None,
    nb_inplace_divide: None,
    nb_inplace_remainder: None,
    nb_inplace_power: None,
    nb_inplace_lshift: None,
    nb_inplace_rshift: None,
    nb_inplace_and: None,
    nb_inplace_xor: None,
    nb_inplace_or: None,
    nb_floor_divide: None,
    nb_true_divide: None,
    nb_inplace_floor_divide: None,
    nb_inplace_true_divide: None,
    nb_index: None,
};

#[repr(C)]
#[derive(Copy)]
pub struct PySequenceMethods {
    pub sq_length: Option<lenfunc>,
    pub sq_concat: Option<binaryfunc>,
    pub sq_repeat: Option<ssizeargfunc>,
    pub sq_item: Option<ssizeargfunc>,
    pub sq_slice: Option<ssizessizeargfunc>,
    pub sq_ass_item: Option<ssizeobjargproc>,
    pub sq_ass_slice: Option<ssizessizeobjargproc>,
    pub sq_contains: Option<objobjproc>,
    pub sq_inplace_concat: Option<binaryfunc>,
    pub sq_inplace_repeat: Option<ssizeargfunc>,
}

impl Clone for PySequenceMethods {
    #[inline] fn clone(&self) -> PySequenceMethods { *self }
}

pub const PySequenceMethods_INIT : PySequenceMethods = PySequenceMethods {
    sq_length: None,
    sq_concat: None,
    sq_repeat: None,
    sq_item: None,
    sq_slice: None,
    sq_ass_item: None,
    sq_ass_slice: None,
    sq_contains: None,
    sq_inplace_concat: None,
    sq_inplace_repeat: None,
};

#[repr(C)]
#[derive(Copy)]
pub struct PyMappingMethods {
    pub mp_length: Option<lenfunc>,
    pub mp_subscript: Option<binaryfunc>,
    pub mp_ass_subscript: Option<objobjargproc>,
}

impl Clone for PyMappingMethods {
    #[inline] fn clone(&self) -> PyMappingMethods { *self }
}

pub const PyMappingMethods_INIT : PyMappingMethods = PyMappingMethods {
    mp_length: None,
    mp_subscript: None,
    mp_ass_subscript: None,
};

#[repr(C)]
#[derive(Copy)]
pub struct PyBufferProcs {
    pub bf_getreadbuffer: Option<readbufferproc>,
    pub bf_getwritebuffer: Option<writebufferproc>,
    pub bf_getsegcount: Option<segcountproc>,
    pub bf_getcharbuffer: Option<charbufferproc>,
    pub bf_getbuffer: Option<getbufferproc>,
    pub bf_releasebuffer: Option<releasebufferproc>,
}

impl Clone for PyBufferProcs {
    #[inline] fn clone(&self) -> PyBufferProcs { *self }
}

pub const PyBufferProcs_INIT : PyBufferProcs = PyBufferProcs {
    bf_getreadbuffer: None,
    bf_getwritebuffer: None,
    bf_getsegcount: None,
    bf_getcharbuffer: None,
    bf_getbuffer: None,
    bf_releasebuffer: None,
};

pub type freefunc =
    unsafe extern "C" fn(arg1: *mut c_void);
pub type destructor =
    unsafe extern "C" fn(arg1: *mut PyObject);
pub type printfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut FILE,
                               arg3: c_int) -> c_int;
pub type getattrfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut c_char)
                              -> *mut PyObject;
pub type getattrofunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject)
                              -> *mut PyObject;
pub type setattrfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut c_char,
                               arg3: *mut PyObject) -> c_int;
pub type setattrofunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject,
                               arg3: *mut PyObject) -> c_int;
pub type cmpfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject)
                              -> c_int;
pub type reprfunc =
    unsafe extern "C" fn(arg1: *mut PyObject)
                              -> *mut PyObject;
pub type hashfunc =
    unsafe extern "C" fn(arg1: *mut PyObject)
                              -> c_long;
pub type richcmpfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject,
                               arg3: c_int) -> *mut PyObject;
pub type getiterfunc =
    unsafe extern "C" fn(arg1: *mut PyObject)
                              -> *mut PyObject;
pub type iternextfunc =
    unsafe extern "C" fn(arg1: *mut PyObject)
                              -> *mut PyObject;
pub type descrgetfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject,
                               arg3: *mut PyObject) -> *mut PyObject;
pub type descrsetfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject,
                               arg3: *mut PyObject) -> c_int;
pub type initproc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyObject,
                               arg3: *mut PyObject) -> c_int;
pub type newfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyTypeObject,
                               arg2: *mut PyObject, arg3: *mut PyObject)
                              -> *mut PyObject;
pub type allocfunc =
    unsafe extern "C" fn
                              (arg1: *mut PyTypeObject,
                               arg2: Py_ssize_t) -> *mut PyObject;

#[repr(C)]
#[derive(Copy)]
pub struct PyTypeObject {
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub ob_size: Py_ssize_t,
    pub tp_name: *const c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: Option<destructor>,
    pub tp_print: Option<printfunc>,
    pub tp_getattr: Option<getattrfunc>,
    pub tp_setattr: Option<setattrfunc>,
    pub tp_compare: Option<cmpfunc>,
    pub tp_repr: Option<reprfunc>,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: Option<hashfunc>,
    pub tp_call: Option<ternaryfunc>,
    pub tp_str: Option<reprfunc>,
    pub tp_getattro: Option<getattrofunc>,
    pub tp_setattro: Option<setattrofunc>,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: c_long,
    pub tp_doc: *const c_char,
    pub tp_traverse: Option<traverseproc>,
    pub tp_clear: Option<inquiry>,
    pub tp_richcompare: Option<richcmpfunc>,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: Option<getiterfunc>,
    pub tp_iternext: Option<iternextfunc>,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut ::structmember::PyMemberDef,
    pub tp_getset: *mut ::descrobject::PyGetSetDef,
    pub tp_base: *mut PyTypeObject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: Option<descrgetfunc>,
    pub tp_descr_set: Option<descrsetfunc>,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: Option<initproc>,
    pub tp_alloc: Option<allocfunc>,
    pub tp_new: Option<newfunc>,
    pub tp_free: Option<freefunc>,
    pub tp_is_gc: Option<inquiry>,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut PyObject,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: Option<destructor>,
    pub tp_version_tag: c_uint,
    #[cfg(py_sys_config="COUNT_ALLOCS")]
    pub tp_allocs: Py_ssize_t,
    #[cfg(py_sys_config="COUNT_ALLOCS")]
    pub tp_frees: Py_ssize_t,
    #[cfg(py_sys_config="COUNT_ALLOCS")]
    pub tp_maxalloc: Py_ssize_t,
    #[cfg(py_sys_config="COUNT_ALLOCS")]
    pub tp_prev: *mut PyTypeObject,
    #[cfg(py_sys_config="COUNT_ALLOCS")]
    pub tp_next: *mut PyTypeObject,
}

impl Clone for PyTypeObject {
    #[inline] fn clone(&self) -> PyTypeObject { *self }
}

#[cfg(py_sys_config="Py_TRACE_REFS")]
pub const PyTypeObject_INIT : PyTypeObject = PyTypeObject {
    _ob_next: 0 as *mut PyObject,
    _ob_prev: 0 as *mut PyObject,
    ob_refcnt: 1,
    ob_type: 0 as *mut PyTypeObject,
    ob_size: 0,
    tp_name: 0 as *const c_char,
    tp_basicsize: 0,
    tp_itemsize: 0,
    tp_dealloc: None,
    tp_print: None,
    tp_getattr: None,
    tp_setattr: None,
    tp_compare: None,
    tp_repr: None,
    tp_as_number: 0 as *mut PyNumberMethods,
    tp_as_sequence: 0 as *mut PySequenceMethods,
    tp_as_mapping: 0 as *mut PyMappingMethods,
    tp_hash: None,
    tp_call: None,
    tp_str: None,
    tp_getattro: None,
    tp_setattro: None,
    tp_as_buffer: 0 as *mut PyBufferProcs,
    tp_flags: Py_TPFLAGS_DEFAULT,
    tp_doc: 0 as *const c_char,
    tp_traverse: None,
    tp_clear: None,
    tp_richcompare: None,
    tp_weaklistoffset: 0,
    tp_iter: None,
    tp_iternext: None,
    tp_methods: 0 as *mut PyMethodDef,
    tp_members: 0 as *mut ::structmember::PyMemberDef,
    tp_getset: 0 as *mut ::descrobject::PyGetSetDef,
    tp_base: 0 as *mut PyTypeObject,
    tp_dict: 0 as *mut PyObject,
    tp_descr_get: None,
    tp_descr_set: None,
    tp_dictoffset: 0,
    tp_init: None,
    tp_alloc: None,
    tp_new: None,
    tp_free: None,
    tp_is_gc: None,
    tp_bases: 0 as *mut PyObject,
    tp_mro: 0 as *mut PyObject,
    tp_cache: 0 as *mut PyObject,
    tp_subclasses: 0 as *mut PyObject,
    tp_weaklist: 0 as *mut PyObject,
    tp_del: None,
    tp_version_tag: 0,
};

#[cfg(not(py_sys_config="Py_TRACE_REFS"))]
pub const PyTypeObject_INIT : PyTypeObject = PyTypeObject {
    ob_refcnt: 1,
    ob_type: 0 as *mut PyTypeObject,
    ob_size: 0,
    tp_name: 0 as *const c_char,
    tp_basicsize: 0,
    tp_itemsize: 0,
    tp_dealloc: None,
    tp_print: None,
    tp_getattr: None,
    tp_setattr: None,
    tp_compare: None,
    tp_repr: None,
    tp_as_number: 0 as *mut PyNumberMethods,
    tp_as_sequence: 0 as *mut PySequenceMethods,
    tp_as_mapping: 0 as *mut PyMappingMethods,
    tp_hash: None,
    tp_call: None,
    tp_str: None,
    tp_getattro: None,
    tp_setattro: None,
    tp_as_buffer: 0 as *mut PyBufferProcs,
    tp_flags: Py_TPFLAGS_DEFAULT,
    tp_doc: 0 as *const c_char,
    tp_traverse: None,
    tp_clear: None,
    tp_richcompare: None,
    tp_weaklistoffset: 0,
    tp_iter: None,
    tp_iternext: None,
    tp_methods: 0 as *mut PyMethodDef,
    tp_members: 0 as *mut ::structmember::PyMemberDef,
    tp_getset: 0 as *mut ::descrobject::PyGetSetDef,
    tp_base: 0 as *mut PyTypeObject,
    tp_dict: 0 as *mut PyObject,
    tp_descr_get: None,
    tp_descr_set: None,
    tp_dictoffset: 0,
    tp_init: None,
    tp_alloc: None,
    tp_new: None,
    tp_free: None,
    tp_is_gc: None,
    tp_bases: 0 as *mut PyObject,
    tp_mro: 0 as *mut PyObject,
    tp_cache: 0 as *mut PyObject,
    tp_subclasses: 0 as *mut PyObject,
    tp_weaklist: 0 as *mut PyObject,
    tp_del: None,
    tp_version_tag: 0,
};

impl PyTypeObject {
    #[inline]
    pub fn init_ob_type(&mut self, type_object: *mut PyTypeObject) {
        self.ob_type = type_object;
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct PyHeapTypeObject {
    pub ht_type: PyTypeObject,
    pub as_number: PyNumberMethods,
    pub as_mapping: PyMappingMethods,
    pub as_sequence: PySequenceMethods,
    pub as_buffer: PyBufferProcs,
    pub ht_name: *mut PyObject,
    pub ht_slots: *mut PyObject,
}

impl Clone for PyHeapTypeObject {
    #[inline] fn clone(&self) -> PyHeapTypeObject { *self }
}

// access macro to the members which are floating "behind" the object
#[inline]
pub unsafe fn PyHeapType_GET_MEMBERS(etype: *mut PyHeapTypeObject) -> *mut ::structmember::PyMemberDef {
    let basicsize = (*Py_TYPE(etype as *mut PyObject)).tp_basicsize;
    (etype as *mut u8).offset(basicsize as isize) as *mut ::structmember::PyMemberDef
}

#[cfg_attr(windows, link(name="pythonXY"))] extern "C" {
    pub fn PyType_IsSubtype(a: *mut PyTypeObject, b: *mut PyTypeObject) -> c_int;
}

#[inline(always)]
pub unsafe fn PyObject_TypeCheck(ob: *mut PyObject, tp: *mut PyTypeObject) -> c_int {
    (Py_TYPE(ob) == tp || PyType_IsSubtype(Py_TYPE(ob), tp) != 0) as c_int
}

#[cfg_attr(windows, link(name="pythonXY"))] extern "C" {
    pub static mut PyType_Type: PyTypeObject;
    pub static mut PyBaseObject_Type: PyTypeObject;
    pub static mut PySuper_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyType_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_TYPE_SUBCLASS)
}

#[inline(always)]
pub unsafe fn PyType_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == (&mut PyType_Type as *mut _)) as c_int
}

#[cfg_attr(windows, link(name="pythonXY"))] extern "C" {
    pub fn PyType_Ready(t: *mut PyTypeObject) -> c_int;
    pub fn PyType_GenericAlloc(t: *mut PyTypeObject, nitems: Py_ssize_t)
     -> *mut PyObject;
    pub fn PyType_GenericNew(t: *mut PyTypeObject, args: *mut PyObject,
                             kwds: *mut PyObject) -> *mut PyObject;
    fn _PyType_Lookup(arg1: *mut PyTypeObject, arg2: *mut PyObject)
     -> *mut PyObject;
    fn _PyObject_LookupSpecial(arg1: *mut PyObject,
                                   arg2: *mut c_char,
                                   arg3: *mut *mut PyObject) -> *mut PyObject;
    pub fn PyType_ClearCache() -> c_uint;
    pub fn PyType_Modified(t: *mut PyTypeObject);
    
    pub fn PyObject_Print(o: *mut PyObject, fp: *mut FILE,
                          flags: c_int) -> c_int;
    fn _PyObject_Dump(o: *mut PyObject);
    pub fn PyObject_Repr(o: *mut PyObject) -> *mut PyObject;
    fn _PyObject_Str(o: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_Str(o: *mut PyObject) -> *mut PyObject;
}

#[inline(always)]
pub unsafe fn PyObject_Bytes(o: *mut PyObject) -> *mut PyObject {
    PyObject_Str(o)
}

#[cfg_attr(windows, link(name="pythonXY"))] extern "C" {
    #[cfg(py_sys_config="Py_USING_UNICODE")]
    pub fn PyObject_Unicode(o: *mut PyObject) -> *mut PyObject;
    
    pub fn PyObject_Compare(arg1: *mut PyObject, arg2: *mut PyObject)
     -> c_int;
    pub fn PyObject_RichCompare(arg1: *mut PyObject, arg2: *mut PyObject,
                                arg3: c_int) -> *mut PyObject;
    pub fn PyObject_RichCompareBool(arg1: *mut PyObject, arg2: *mut PyObject,
                                    arg3: c_int) -> c_int;
    pub fn PyObject_GetAttrString(arg1: *mut PyObject,
                                  arg2: *const c_char)
     -> *mut PyObject;
    pub fn PyObject_SetAttrString(arg1: *mut PyObject,
                                  arg2: *const c_char,
                                  arg3: *mut PyObject) -> c_int;
    pub fn PyObject_HasAttrString(arg1: *mut PyObject,
                                  arg2: *const c_char)
     -> c_int;
    pub fn PyObject_GetAttr(arg1: *mut PyObject, arg2: *mut PyObject)
     -> *mut PyObject;
    pub fn PyObject_SetAttr(arg1: *mut PyObject, arg2: *mut PyObject,
                            arg3: *mut PyObject) -> c_int;
    pub fn PyObject_HasAttr(arg1: *mut PyObject, arg2: *mut PyObject)
     -> c_int;
    fn _PyObject_GetDictPtr(arg1: *mut PyObject) -> *mut *mut PyObject;
    pub fn PyObject_SelfIter(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_GenericGetAttr(arg1: *mut PyObject, arg2: *mut PyObject)
     -> *mut PyObject;
    pub fn PyObject_GenericSetAttr(arg1: *mut PyObject, arg2: *mut PyObject,
                                   arg3: *mut PyObject) -> c_int;
    pub fn PyObject_Hash(arg1: *mut PyObject) -> c_long;
    pub fn PyObject_HashNotImplemented(arg1: *mut PyObject) -> c_long;
    pub fn PyObject_IsTrue(arg1: *mut PyObject) -> c_int;
    pub fn PyObject_Not(arg1: *mut PyObject) -> c_int;
    pub fn PyCallable_Check(arg1: *mut PyObject) -> c_int;
    pub fn PyNumber_Coerce(arg1: *mut *mut PyObject, arg2: *mut *mut PyObject)
     -> c_int;
    pub fn PyNumber_CoerceEx(arg1: *mut *mut PyObject,
                             arg2: *mut *mut PyObject) -> c_int;
    pub fn PyObject_ClearWeakRefs(arg1: *mut PyObject);
    fn _PyObject_SlotCompare(arg1: *mut PyObject, arg2: *mut PyObject)
     -> c_int;
    fn _PyObject_GenericGetAttrWithDict(arg1: *mut PyObject,
                                            arg2: *mut PyObject,
                                            arg3: *mut PyObject)
     -> *mut PyObject;
    fn _PyObject_GenericSetAttrWithDict(arg1: *mut PyObject,
                                            arg2: *mut PyObject,
                                            arg3: *mut PyObject,
                                            arg4: *mut PyObject)
     -> c_int;
    pub fn PyObject_Dir(arg1: *mut PyObject) -> *mut PyObject;
    pub fn Py_ReprEnter(arg1: *mut PyObject) -> c_int;
    pub fn Py_ReprLeave(arg1: *mut PyObject);
    fn _Py_HashDouble(arg1: c_double) -> c_long;
    fn _Py_HashPointer(arg1: *mut c_void) -> c_long;
}

// Flag bits for printing:
pub const Py_PRINT_RAW : c_int = 1;       // No string quotes etc.


/// PyBufferProcs contains bf_getcharbuffer
pub const Py_TPFLAGS_HAVE_GETCHARBUFFER : c_long = (1<<0);

/// PySequenceMethods contains sq_contains
pub const Py_TPFLAGS_HAVE_SEQUENCE_IN : c_long = (1<<1);

/// PySequenceMethods and PyNumberMethods contain in-place operators
pub const Py_TPFLAGS_HAVE_INPLACEOPS : c_long = (1<<3);

/// PyNumberMethods do their own coercion
pub const Py_TPFLAGS_CHECKTYPES : c_long = (1<<4);

/// tp_richcompare is defined
pub const Py_TPFLAGS_HAVE_RICHCOMPARE : c_long = (1<<5);

/// Objects which are weakly referencable if their tp_weaklistoffset is >0
pub const Py_TPFLAGS_HAVE_WEAKREFS : c_long = (1<<6);

/// tp_iter is defined
pub const Py_TPFLAGS_HAVE_ITER : c_long = (1<<7);

/// New members introduced by Python 2.2 exist
pub const Py_TPFLAGS_HAVE_CLASS : c_long = (1<<8);

/// Set if the type object is dynamically allocated
pub const Py_TPFLAGS_HEAPTYPE : c_long = (1<<9);

/// Set if the type allows subclassing
pub const Py_TPFLAGS_BASETYPE : c_long = (1<<10);

/// Set if the type is 'ready' -- fully initialized
pub const Py_TPFLAGS_READY : c_long = (1<<12);

/// Set while the type is being 'readied', to prevent recursive ready calls
pub const Py_TPFLAGS_READYING : c_long = (1<<13);

/// Objects support garbage collection (see objimp.h)
pub const Py_TPFLAGS_HAVE_GC : c_long = (1<<14);

// Two bits are preserved for Stackless Python, next after this is 17.
const Py_TPFLAGS_HAVE_STACKLESS_EXTENSION : c_long = 0;

/// Objects support nb_index in PyNumberMethods
pub const Py_TPFLAGS_HAVE_INDEX : c_long = (1<<17);

/// Objects support type attribute cache
pub const Py_TPFLAGS_HAVE_VERSION_TAG  : c_long = (1<<18);
pub const Py_TPFLAGS_VALID_VERSION_TAG : c_long = (1<<19);

/* Type is abstract and cannot be instantiated */
pub const Py_TPFLAGS_IS_ABSTRACT : c_long = (1<<20);

/* Has the new buffer protocol */
pub const Py_TPFLAGS_HAVE_NEWBUFFER : c_long = (1<<21);

/* These flags are used to determine if a type is a subclass. */
pub const Py_TPFLAGS_INT_SUBCLASS         : c_long = (1<<23);
pub const Py_TPFLAGS_LONG_SUBCLASS        : c_long = (1<<24);
pub const Py_TPFLAGS_LIST_SUBCLASS        : c_long = (1<<25);
pub const Py_TPFLAGS_TUPLE_SUBCLASS       : c_long = (1<<26);
pub const Py_TPFLAGS_STRING_SUBCLASS      : c_long = (1<<27);
pub const Py_TPFLAGS_UNICODE_SUBCLASS     : c_long = (1<<28);
pub const Py_TPFLAGS_DICT_SUBCLASS        : c_long = (1<<29);
pub const Py_TPFLAGS_BASE_EXC_SUBCLASS    : c_long = (1<<30);
pub const Py_TPFLAGS_TYPE_SUBCLASS        : c_long = (1<<31);

pub const Py_TPFLAGS_DEFAULT : c_long = (
                 Py_TPFLAGS_HAVE_GETCHARBUFFER |
                 Py_TPFLAGS_HAVE_SEQUENCE_IN |
                 Py_TPFLAGS_HAVE_INPLACEOPS |
                 Py_TPFLAGS_HAVE_RICHCOMPARE |
                 Py_TPFLAGS_HAVE_WEAKREFS |
                 Py_TPFLAGS_HAVE_ITER |
                 Py_TPFLAGS_HAVE_CLASS |
                 Py_TPFLAGS_HAVE_STACKLESS_EXTENSION |
                 Py_TPFLAGS_HAVE_INDEX |
                 0);

#[inline(always)]
pub unsafe fn PyType_HasFeature(t : *mut PyTypeObject, f : c_long) -> c_int {
    (((*t).tp_flags & f) != 0) as c_int
}

#[inline(always)]
pub unsafe fn PyType_FastSubclass(t : *mut PyTypeObject, f : c_long) -> c_int {
    PyType_HasFeature(t, f)
}

// Reference counting macros.
#[inline(always)]
pub unsafe fn Py_INCREF(op : *mut PyObject) {
    if cfg!(py_sys_config="Py_REF_DEBUG") {
        Py_IncRef(op)
    } else {
        (*op).ob_refcnt += 1
    }
}

#[inline(always)]
pub unsafe fn Py_DECREF(op: *mut PyObject) {
    if cfg!(py_sys_config="Py_REF_DEBUG") || cfg!(py_sys_config="COUNT_ALLOCS") {
        Py_DecRef(op)
    } else {
        (*op).ob_refcnt -= 1;
        if (*op).ob_refcnt == 0 {
            (*Py_TYPE(op)).tp_dealloc.unwrap()(op)
        }
    }
}

#[inline(always)]
pub unsafe fn Py_CLEAR(op: &mut *mut PyObject) {
    let tmp = *op;
    if !tmp.is_null() {
        *op = ptr::null_mut();
        Py_DECREF(tmp);
    }
}

#[inline(always)]
pub unsafe fn Py_XINCREF(op : *mut PyObject) {
    if !op.is_null() {
        Py_INCREF(op)
    }
}

#[inline(always)]
pub unsafe fn Py_XDECREF(op : *mut PyObject) {
    if !op.is_null() {
        Py_DECREF(op)
    }
}

#[cfg_attr(windows, link(name="pythonXY"))] extern "C" {
    pub fn Py_IncRef(o: *mut PyObject);
    pub fn Py_DecRef(o: *mut PyObject);

    static mut _Py_NoneStruct: PyObject;
    static mut _Py_NotImplementedStruct: PyObject;
}

#[inline(always)]
pub unsafe fn Py_None() -> *mut PyObject {
    &mut _Py_NoneStruct
}

#[inline(always)]
pub unsafe fn Py_NotImplemented() -> *mut PyObject {
    &mut _Py_NotImplementedStruct
}

/* Rich comparison opcodes */
pub const Py_LT : c_int = 0;
pub const Py_LE : c_int = 1;
pub const Py_EQ : c_int = 2;
pub const Py_NE : c_int = 3;
pub const Py_GT : c_int = 4;
pub const Py_GE : c_int = 5;

#[cfg_attr(windows, link(name="pythonXY"))] extern "C" {
    fn _PyTrash_thread_deposit_object(o: *mut PyObject);
    fn _PyTrash_thread_destroy_chain();
}

pub const PyTrash_UNWIND_LEVEL : c_int = 50;

#[inline(always)]
pub unsafe fn Py_TRASHCAN<F : FnOnce() -> ()>(op: *mut PyObject, body: F) {
    let tstate = ::pystate::PyThreadState_GET();
    if tstate.is_null() || (*tstate).trash_delete_nesting < PyTrash_UNWIND_LEVEL {
        if !tstate.is_null() {
            (*tstate).trash_delete_nesting += 1;
        }
        body();
        if !tstate.is_null() {
            (*tstate).trash_delete_nesting -= 1;
            if !(*tstate).trash_delete_later.is_null() && (*tstate).trash_delete_nesting <= 0 {
                _PyTrash_thread_destroy_chain();
            }
        }
    } else {
        _PyTrash_thread_deposit_object(op)
    }
}

