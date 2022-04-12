//! These are useful utility functions.
//! 
//! Many of these functions are copied directly from ash as they are not declared public.

use ash::vk;
use ash::prelude::VkResult;

/// This is a direct copy from ash::prelude (because it is not public).
///
/// Repeatedly calls `f` until it does not return [`vk::Result::INCOMPLETE`] anymore,
/// ensuring all available data has been read into the vector.
///
/// See for example [`vkEnumerateInstanceExtensionProperties`]: the number of available
/// items may change between calls; [`vk::Result::INCOMPLETE`] is returned when the count
/// increased (and the vector is not large enough after querying the initial size),
/// requiring Ash to try again.
///
/// [`vkEnumerateInstanceExtensionProperties`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
pub(crate) unsafe fn read_into_uninitialized_vector<N: Copy + Default + TryInto<usize>, T>(
    f: impl Fn(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: std::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, std::ptr::null_mut()).result()?;
        let mut data =
            Vec::with_capacity(count.try_into().expect("`N` failed to convert to `usize`"));

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            data.set_len(count.try_into().expect("`N` failed to convert to `usize`"));
            break err_code.result_with_success(data);
        }
    }
}

/// This is a direct copy from ash::prelude (because it is not public).
#[cfg(feature = "debug")]
pub(crate) fn debug_flags<Value: Into<u64> + Copy>(
    f: &mut std::fmt::Formatter,
    known: &[(Value, &'static str)],
    value: Value,
) -> std::fmt::Result {
    let mut first = true;
    let mut accum = value.into();
    for &(bit, name) in known {
        let bit = bit.into();
        if bit != 0 && accum & bit == bit {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            accum &= !bit;
        }
    }
    if accum != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{:b}", accum)?;
    }
    Ok(())
}


/// Creates a fixed size c_char array from a CStr.
/// 
/// If the size of the string is too large for the array None is returned.
pub(crate) fn c_char_array_from_cstr<const N: usize>(data: &::std::ffi::CStr) -> Option<[::std::os::raw::c_char; N]> {
    let mut result: [::std::os::raw::c_char; N] = unsafe { ::std::mem::zeroed() }; // Default not implemented for arbitrary length

    // Yes this is stupid but rust FFI is absolutely useless
    let data = data.to_bytes_with_nul();
    if data.len() > N {
        return None;
    }

    for (i, c) in data.iter().enumerate() {
        result[i] = *c as ::std::os::raw::c_char;
    }

    Some(result)
}