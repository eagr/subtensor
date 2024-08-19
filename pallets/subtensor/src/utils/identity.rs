use super::*;
use frame_support::ensure;
use frame_system::ensure_signed;
use sp_std::vec::Vec;

impl<T: Config> Pallet<T> {
    /// Sets the identity for a coldkey.
    ///
    /// This function allows a user to set or update their identity information associated with their coldkey.
    /// It checks if the caller has at least one registered hotkey, validates the provided identity information,
    /// and then stores it in the blockchain state.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin of the call, which should be a signed extrinsic.
    /// * `name` - The name to be associated with the identity.
    /// * `url` - A URL associated with the identity.
    /// * `image` - An image URL or identifier for the identity.
    /// * `discord` - Discord information for the identity.
    /// * `description` - A description of the identity.
    /// * `additional` - Any additional information for the identity.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the identity is successfully set, otherwise returns an error.
    pub fn do_set_identity(
        origin: T::RuntimeOrigin,
        name: Vec<u8>,
        url: Vec<u8>,
        image: Vec<u8>,
        discord: Vec<u8>,
        description: Vec<u8>,
        additional: Vec<u8>,
    ) -> dispatch::DispatchResult {
        // Ensure the call is signed and get the signer's (coldkey) account
        let coldkey = ensure_signed(origin)?;

        // Retrieve all hotkeys associated with this coldkey
        let hotkeys: Vec<T::AccountId> = OwnedHotkeys::<T>::get(coldkey.clone());

        // Ensure that at least one of the associated hotkeys is registered on any network
        ensure!(
            hotkeys
                .iter()
                .any(|hotkey| Self::is_hotkey_registered_on_any_network(hotkey)),
            Error::<T>::HotKeyNotRegisteredInNetwork
        );

        // Create the identity struct with the provided information
        let identity = ChainIdentityOf {
            name,
            url,
            image,
            discord,
            description,
            additional,
        };

        // Validate the created identity
        ensure!(
            Self::is_valid_identity(&identity),
            Error::<T>::InvalidIdentity
        );

        // Store the validated identity in the blockchain state
        Identities::<T>::insert(coldkey.clone(), identity.clone());

        // Log the identity set event
        log::debug!("ChainIdentitySet( coldkey:{:?} ) ", coldkey.clone());

        // Emit an event to notify that an identity has been set
        Self::deposit_event(Event::ChainIdentitySet(coldkey.clone()));

        // Return Ok to indicate successful execution
        Ok(())
    }

    /// Validates the given ChainIdentityOf struct.
    ///
    /// This function checks if the total length of all fields in the ChainIdentityOf struct
    /// is less than or equal to 512 bytes, and if each individual field is also
    /// less than or equal to 512 bytes.
    ///
    /// # Arguments
    ///
    /// * `identity` - A reference to the ChainIdentityOf struct to be validated.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns true if the Identity is valid, false otherwise.
    pub fn is_valid_identity(identity: &ChainIdentityOf) -> bool {
        let total_length = identity
            .name
            .len()
            .saturating_add(identity.url.len())
            .saturating_add(identity.image.len())
            .saturating_add(identity.discord.len())
            .saturating_add(identity.description.len())
            .saturating_add(identity.additional.len());

        total_length <= 256 + 256 + 1024 + 256 + 1024 + 1024
            && identity.name.len() <= 256
            && identity.url.len() <= 256
            && identity.image.len() <= 1024
            && identity.discord.len() <= 256
            && identity.description.len() <= 1024
            && identity.additional.len() <= 1024
    }
}