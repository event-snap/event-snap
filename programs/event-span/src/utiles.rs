use std::{cell::RefMut, mem, ops::DerefMut};

use anchor_lang::{prelude::*, ZeroCopy, __private::bytemuck};

pub fn deserialize_account<'info, T: ZeroCopy + Owner>(
    acc_info: &'info AccountInfo,
) -> Result<RefMut<'info, T>> {
    if !acc_info.is_writable {
        return Err(ErrorCode::AccountNotMutable.into());
    }
    let data = acc_info.try_borrow_mut_data()?;

    let mut disc_bytes = [0u8; 8];
    disc_bytes.copy_from_slice(&data[..8]);

    Ok(RefMut::map(data, |data| {
        bytemuck::from_bytes_mut(&mut data.deref_mut()[8..mem::size_of::<T>() + 8])
    }))
}
