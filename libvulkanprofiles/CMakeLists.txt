project( vkprofiles )
cmake_minimum_required( VERSION 3.13 )
include(FetchContent)

find_package(Vulkan REQUIRED)

FetchContent_Declare(
    VkProfiles
    GIT_REPOSITORY https://github.com/KhronosGroup/Vulkan-Profiles.git
    GIT_TAG "v1.3.208"
)
FetchContent_GetProperties(VkProfiles)
FetchContent_Populate(VkProfiles)

add_library(${PROJECT_NAME} STATIC ${vkprofiles_SOURCE_DIR}/library/source/vulkan_profiles.cpp)

target_include_directories(${PROJECT_NAME} PRIVATE ${vkprofiles_SOURCE_DIR}/library/include)
target_include_directories(${PROJECT_NAME} PRIVATE Vulkan_INCLUDE_DIRS)

install(TARGETS ${PROJECT_NAME} DESTINATION .)