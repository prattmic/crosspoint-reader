cmake_minimum_required(VERSION 3.16)
project(crosspoint_reader_tests C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

include(FetchContent)

FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG v1.17.0
)

set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
set(INSTALL_GTEST OFF CACHE BOOL "" FORCE)
set(BUILD_GMOCK OFF CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

set(REPO_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/..")

add_library(crosspoint_test_common INTERFACE)
target_include_directories(crosspoint_test_common INTERFACE
  ${REPO_ROOT}
  ${REPO_ROOT}/lib
)
target_compile_options(crosspoint_test_common INTERFACE
  -Wall
  -Wextra
  -pedantic
)

find_package(Python3 REQUIRED COMPONENTS Interpreter)

# Build the lib/rust workspace for the host and link it into every test,
# mirroring the firmware's x4pro-rust env (scripts/build_rust_lib.py), so code
# guarded by CROSSPOINT_RUST_LIB is tested against the Rust implementation.
option(CROSSPOINT_RUST_LIB "Build and link the lib/rust workspace (requires cargo)" OFF)
if(CROSSPOINT_RUST_LIB)
  find_program(CARGO_EXECUTABLE cargo REQUIRED)
  find_package(Threads REQUIRED)

  set(RUST_LIB_WORKSPACE_DIR "${REPO_ROOT}/lib/rust")
  set(RUST_LIB_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/rust-lib")
  set(RUST_LIB_INCLUDE_DIR "${RUST_LIB_TARGET_DIR}/include")
  set(RUST_LIB_ARCHIVE "${RUST_LIB_TARGET_DIR}/release/${CMAKE_STATIC_LIBRARY_PREFIX}rust_lib${CMAKE_STATIC_LIBRARY_SUFFIX}")

  # Always run: cargo tracks the workspace's own dependencies and is a fast no-op
  # when nothing changed.
  add_custom_target(rust_lib_build
    COMMAND ${CMAKE_COMMAND} -E env CARGO=${CARGO_EXECUTABLE}
      ${Python3_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/build_rust_lib.py
      ${RUST_LIB_WORKSPACE_DIR} ${RUST_LIB_TARGET_DIR} release ${RUST_LIB_INCLUDE_DIR}
    BYPRODUCTS ${RUST_LIB_ARCHIVE}
    COMMENT "Building lib/rust with cargo"
    USES_TERMINAL
  )

  # Must exist at configure time for INTERFACE_INCLUDE_DIRECTORIES.
  file(MAKE_DIRECTORY ${RUST_LIB_INCLUDE_DIR})

  add_library(rust_lib STATIC IMPORTED GLOBAL)
  set_target_properties(rust_lib PROPERTIES
    IMPORTED_LOCATION ${RUST_LIB_ARCHIVE}
    INTERFACE_INCLUDE_DIRECTORIES ${RUST_LIB_INCLUDE_DIR}
    INTERFACE_COMPILE_DEFINITIONS CROSSPOINT_RUST_LIB=1
    # Native libraries Rust's std needs on the host.
    INTERFACE_LINK_LIBRARIES "Threads::Threads;${CMAKE_DL_LIBS}"
  )
  add_dependencies(rust_lib rust_lib_build)

  target_link_libraries(crosspoint_test_common INTERFACE rust_lib)
endif()

enable_testing()
include(GoogleTest)

add_subdirectory(streaming_json_parser)
add_subdirectory(release_json_parser)
add_subdirectory(differential_rounding)
add_subdirectory(hyphenation_eval)
add_subdirectory(utf8_compose)
add_subdirectory(opds_filename)
add_subdirectory(html_to_plain_text)
add_subdirectory(library_text)
add_subdirectory(library_format)
add_subdirectory(library_index_file)
add_subdirectory(library_builder)
add_subdirectory(minibidi_arabic)
add_subdirectory(combining_marks)
add_subdirectory(credential_integrity)
add_subdirectory(xml_parser_utils)
add_subdirectory(token_boundary)
add_subdirectory(koreader_xpath_resolver)
add_subdirectory(progress_comparison)
add_subdirectory(font_cache_manager)
add_subdirectory(css_parser)
add_subdirectory(chapter_html_slim_parser)
add_subdirectory(korean_line_breaking)
add_subdirectory(ligature_guard)
add_subdirectory(page_link)
add_subdirectory(content_opf_parser)
add_subdirectory(chapter_position)
add_subdirectory(home_button)
add_subdirectory(button_press_navigation)

add_subdirectory(inflate_stream)
add_subdirectory(fs_helpers)
add_subdirectory(txt_to_html)

add_subdirectory(absolute_grayscale)

add_subdirectory(sd_card_font)
add_subdirectory(glyph_bitmap)

add_subdirectory(reader_session)
add_subdirectory(plugin_manifest_classification)
add_subdirectory(protected_paths)

add_test(NAME TrustedTimeTimezone COMMAND ${Python3_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/test_trusted_time.py)
