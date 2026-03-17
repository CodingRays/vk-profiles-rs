cmake_minimum_required( VERSION 3.13 )
project( vkprofiles )

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


find_package(VulkanHeaders REQUIRED CONFIG QUIET)

add_library(${PROJECT_NAME} STATIC ${VK_PROFILES_SRC_DIR}/library/source/vulkan_profiles.cpp)

target_include_directories(${PROJECT_NAME} PRIVATE ${VK_PROFILES_SRC_DIR}/library/include)
target_link_libraries(${PROJECT_NAME} PUBLIC Vulkan::Headers)

install(TARGETS ${PROJECT_NAME} DESTINATION .)
