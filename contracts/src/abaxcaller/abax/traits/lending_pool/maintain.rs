/// Trait containing messages that are used to maintain inetrest accumulation. Used by **maintainers**.
#[ink::trait_definition]
pub trait LendingPoolMaintain {
    /// is used by anyone to accumulate deposit and variable rate interests
    ///
    ///  * `asset` - AccountId (aka address) of asset of which interests should be accumulated
    ///
    /// # Errors
    /// * `AccumulatedAlready` returned if the interest of `asset` was already accumulated in this block.
    #[ink(message)]
    fn accumulate_interest(&mut self, asset: AccountId) -> Result<(), LendingPoolError>;
}
