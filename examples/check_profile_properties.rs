//! Example showing how to check the properties of a profile.

extern crate vk_profiles_rs;

use ash::vk;
use vk_profiles_rs::{VulkanProfiles, vp};

fn main() {
    let vk_profiles = vk_profiles_rs::VulkanProfiles::linked();

    let profiles = unsafe { vk_profiles.get_profiles().expect("Failed to enumerate supported profiles") };
    for profile in &profiles {
        print_profile_property_support(&vk_profiles, profile);
    }
}

fn print_profile_property_support(vk_profiles: &VulkanProfiles, profile: &vp::ProfileProperties) {
    if let Some(min) = get_max_descriptor_set_update_after_bind_input_attachments(&vk_profiles, &profile).unwrap() {
        println!("Profile {:?} guarantees maxDescriptorSetUpdateAfterBindInputAttachments is at least {:?}", profile, min)
    } else {
        println!("Profile {:?} does not include VK_EXT_descriptor_indexing or vulkan 1.2", profile);
    }
}

fn get_max_descriptor_set_update_after_bind_input_attachments(vk_profiles: &VulkanProfiles, profile: &vp::ProfileProperties) -> Result<Option<u32>, vk::Result> {
    // The maxDescriptorSetUpdateAfterBindInputAttachments propety can either be defined in the extension struct or the vulkan 1.2 properties struct so we need to check which one the profile supports.
    let property_types = unsafe { vk_profiles.get_profile_property_structure_types(profile)? };

    if property_types.contains(&vk::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES) {
        // Can use the extension structure
        let mut properties = vk::PhysicalDeviceDescriptorIndexingProperties::builder();

        // We need to wrap our properties in a [`vk::PhysicalDeviceProperties2`] instance.
        let mut wrapped = vk::PhysicalDeviceProperties2::builder().push_next(&mut properties);
        unsafe { vk_profiles.get_profile_properties(profile, &mut wrapped) };

        Ok(Some(properties.max_descriptor_set_update_after_bind_input_attachments))
    } else if property_types.contains(&vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES) {
        // Need to use vulkan 1.2 properties
        let mut properties = vk::PhysicalDeviceVulkan12Properties::builder();

        // We need to wrap our properties in a [`vk::PhysicalDeviceProperties2`] instance.
        let mut wrapped = vk::PhysicalDeviceProperties2::builder().push_next(&mut properties);
        unsafe { vk_profiles.get_profile_properties(profile, &mut wrapped) };

        Ok(Some(properties.max_descriptor_set_update_after_bind_input_attachments))
    } else {
        Ok(None)
    }
}