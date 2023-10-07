use std::ffi::{c_char, c_void, CStr};
use std::ptr;

use crate::buffer::Buffer;
use crate::{
    acir_create_proof, acir_delete_acir_composer, acir_get_circuit_sizes,
    acir_get_solidity_verifier, acir_get_verification_key, acir_init_proving_key,
    acir_init_verification_key, acir_load_verification_key, acir_new_acir_composer,
    acir_serialize_proof_into_fields, acir_serialize_verification_key_into_fields,
    acir_verify_proof,
};

/// A safe wrapper around the ACIR composer from the C library.
pub struct AcirComposer {
    ptr: *mut c_void,
}

impl AcirComposer {
    /// Creates a new ACIR composer.
    pub fn new(size_hint: u32) -> Result<Self, &'static str> {
        let mut out_ptr = ptr::null_mut();
        unsafe { acir_new_acir_composer(&size_hint, &mut out_ptr) }
        if out_ptr.is_null() {
            Err("Failed to create a new ACIR composer.")
        } else {
            Ok(AcirComposer { ptr: out_ptr })
        }
    }

    /// Initializes the proving key for this composer.
    pub fn init_proving_key(&self, constraint_system_buf: &[u8]) {
        unsafe { acir_init_proving_key(&self.ptr, constraint_system_buf.as_ptr()) }
    }

    /// Creates a proof using the provided constraint system buffer and witness.
    pub fn create_proof(
        &self,
        constraint_system_buf: &[u8],
        witness: &[u8],
        is_recursive: bool,
    ) -> Result<Vec<u8>, &'static str> {
        let mut out_ptr: *mut u8 = ptr::null_mut();
        unsafe {
            acir_create_proof(
                &self.ptr,
                constraint_system_buf.as_ptr(),
                witness.as_ptr(),
                &is_recursive,
                &mut out_ptr,
            );
        }
        if out_ptr.is_null() {
            Err("Failed to create proof.")
        } else {
            let result = unsafe { Buffer::from_ptr(out_ptr)?.to_vec() };
            Ok(result)
        }
    }

    pub fn load_verification_key(&self, verification_key: &[u8]) {
        unsafe {
            acir_load_verification_key(&self.ptr, verification_key.as_ptr());
        }
    }

    pub fn init_verification_key(&self) {
        unsafe {
            acir_init_verification_key(&self.ptr);
        }
    }

    pub fn get_verification_key(&self) -> Result<Vec<u8>, &'static str> {
        let mut out_ptr: *mut u8 = ptr::null_mut();
        unsafe {
            acir_get_verification_key(&self.ptr, &mut out_ptr);
        }
        if out_ptr.is_null() {
            Err("Failed to get verification key.")
        } else {
            let result = unsafe { Buffer::from_ptr(out_ptr)?.to_vec() };
            Ok(result)
        }
    }

    pub fn verify_proof(&self, proof: &[u8], is_recursive: bool) -> bool {
        let mut result = false;
        unsafe {
            acir_verify_proof(&self.ptr, proof.as_ptr(), &is_recursive, &mut result);
        }
        result
    }

    pub fn get_solidity_verifier(&self) -> Result<String, &'static str> {
        let mut out_ptr: *mut u8 = ptr::null_mut();
        unsafe {
            acir_get_solidity_verifier(&self.ptr, &mut out_ptr);
        }
        if out_ptr.is_null() {
            Err("Failed to get solidity verifier.")
        } else {
            let verifier_string = unsafe {
                CStr::from_ptr(out_ptr as *const c_char)
                    .to_str()
                    .unwrap()
                    .to_string()
            };
            Ok(verifier_string)
        }
    }

    pub fn serialize_proof_into_fields(
        &self,
        proof: &[u8],
        num_inner_public_inputs: u32,
    ) -> Result<Vec<u8>, &'static str> {
        let mut out_ptr: *mut u8 = ptr::null_mut();
        unsafe {
            acir_serialize_proof_into_fields(
                &self.ptr,
                proof.as_ptr(),
                &num_inner_public_inputs,
                &mut out_ptr,
            );
        }
        if out_ptr.is_null() {
            Err("Failed to serialize proof into fields.")
        } else {
            let result = unsafe { Buffer::from_ptr(out_ptr)?.to_vec() };
            Ok(result)
        }
    }

    pub fn serialize_verification_key_into_fields(
        &self,
    ) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
        let mut out_vkey_ptr: *mut u8 = ptr::null_mut();
        let out_key_hash_ptr: *mut u8 = ptr::null_mut();
        unsafe {
            acir_serialize_verification_key_into_fields(
                &self.ptr,
                &mut out_vkey_ptr,
                out_key_hash_ptr,
            );
        }
        if out_vkey_ptr.is_null() || out_key_hash_ptr.is_null() {
            Err("Failed to serialize verification key into fields.")
        } else {
            let vkey = unsafe { Buffer::from_ptr(out_vkey_ptr)?.to_vec() };
            let key_hash = unsafe { Buffer::from_ptr(out_key_hash_ptr)?.to_vec() };
            Ok((vkey, key_hash))
        }
    }

    /// Internally frees the underlying ACIR composer.
    fn delete(&self) {
        unsafe { acir_delete_acir_composer(&self.ptr) }
    }
}

impl Drop for AcirComposer {
    fn drop(&mut self) {
        self.delete();
    }
}

/// Represents the sizes of various circuit components.
#[derive(Default, Debug)]
pub struct CircuitSizes {
    pub exact: u32,
    pub total: u32,
    pub subgroup: u32,
}

/// Fetches the sizes for various circuit components using the provided constraint system buffer.
pub fn get_circuit_sizes(constraint_system_buf: &[u8]) -> CircuitSizes {
    let mut ret = CircuitSizes::default();
    unsafe {
        acir_get_circuit_sizes(
            constraint_system_buf.as_ptr(),
            &mut ret.exact,
            &mut ret.total,
            &mut ret.subgroup,
        );
    }
    ret
}