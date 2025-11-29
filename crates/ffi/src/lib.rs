use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use mazerion_api::{ApiRequest, execute_calculation, list_calculators};
use std::collections::HashMap;

#[repr(C)]
pub struct FFICalculatorInfo {
    pub id: *mut c_char,
    pub name: *mut c_char,
    pub description: *mut c_char,
    pub category: *mut c_char,
}

#[repr(C)]
pub struct FFICalcResult {
    pub value: *mut c_char,
    pub unit: *mut c_char,
    pub error: *mut c_char,
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_calculate(
    calc_id: *const c_char,
    params_json: *const c_char,
) -> *mut FFICalcResult {
    if calc_id.is_null() || params_json.is_null() {
        return ptr::null_mut();
    }

    let calc_id = match unsafe { CStr::from_ptr(calc_id) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let params_str = match unsafe { CStr::from_ptr(params_json) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let params: HashMap<String, String> = match serde_json::from_str(params_str) {
        Ok(p) => p,
        Err(_) => return ptr::null_mut(),
    };

    let request = ApiRequest {
        calculator_id: calc_id.to_string(),
        params,
    };

    let result = Box::new(match execute_calculation(request) {
        Ok(response) => FFICalcResult {
            value: CString::new(response.value).unwrap().into_raw(),
            unit: CString::new(response.unit).unwrap().into_raw(),
            error: ptr::null_mut(),
        },
        Err(e) => FFICalcResult {
            value: ptr::null_mut(),
            unit: ptr::null_mut(),
            error: CString::new(e.to_string()).unwrap().into_raw(),
        },
    });

    Box::into_raw(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_list_calculators(out_len: *mut usize) -> *mut FFICalculatorInfo {
    let calculators = list_calculators();
    let len = calculators.len();

    if !out_len.is_null() {
        unsafe { *out_len = len };
    }

    let mut infos: Vec<FFICalculatorInfo> = calculators
        .into_iter()
        .map(|calc| FFICalculatorInfo {
            id: CString::new(calc.id).unwrap().into_raw(),
            name: CString::new(calc.name).unwrap().into_raw(),
            description: CString::new(calc.description).unwrap().into_raw(),
            category: CString::new(calc.category).unwrap().into_raw(),
        })
        .collect();

    let ptr = infos.as_mut_ptr();
    std::mem::forget(infos);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_free_result(result: *mut FFICalcResult) {
    if result.is_null() {
        return;
    }

    unsafe {
        let result = Box::from_raw(result);
        if !result.value.is_null() {
            let _ = CString::from_raw(result.value);
        }
        if !result.unit.is_null() {
            let _ = CString::from_raw(result.unit);
        }
        if !result.error.is_null() {
            let _ = CString::from_raw(result.error);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_free_calculator_infos(infos: *mut FFICalculatorInfo, len: usize) {
    if infos.is_null() {
        return;
    }

    unsafe {
        let infos = Vec::from_raw_parts(infos, len, len);
        for info in infos {
            if !info.id.is_null() {
                let _ = CString::from_raw(info.id);
            }
            if !info.name.is_null() {
                let _ = CString::from_raw(info.name);
            }
            if !info.description.is_null() {
                let _ = CString::from_raw(info.description);
            }
            if !info.category.is_null() {
                let _ = CString::from_raw(info.category);
            }
        }
    }
}