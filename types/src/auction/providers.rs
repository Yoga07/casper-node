use crate::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    system_contract_errors::auction::Error,
    CLTyped, Key, URef, U512,
};

/// Provider of runtime host functionality.
pub trait RuntimeProvider {
    /// This method should return the caller of the current context.
    fn get_caller(&self) -> AccountHash;
}

/// Provides functionality of a contract storage.
pub trait StorageProvider {
    /// Compatible error type.
    type Error: From<Error>;

    /// Obtain named [`Key`].
    fn get_key(&mut self, name: &str) -> Option<Key>;

    /// Read data from [`URef`].
    fn read<T: FromBytes + CLTyped>(&mut self, uref: URef) -> Result<Option<T>, Self::Error>;

    /// Write data to [`URef].
    fn write<T: ToBytes + CLTyped>(&mut self, uref: URef, value: T) -> Result<(), Self::Error>;
}

/// Provides functionality of a system module.
pub trait SystemProvider {
    /// Error representation for system errors.
    type Error: From<Error>;

    /// Creates new purse.
    fn create_purse(&mut self) -> URef;

    /// Gets purse balance.
    fn get_balance(&mut self, purse: URef) -> Result<Option<U512>, Self::Error>;

    /// Transfers specified `amount` of tokens from `source` purse into a `target` purse.
    fn transfer_from_purse_to_purse(
        &mut self,
        source: URef,
        target: URef,
        amount: U512,
    ) -> Result<(), Self::Error>;
}
