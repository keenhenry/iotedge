#Copyright (c) Microsoft. All rights reserved.
#Licensed under the MIT license. See LICENSE file in the project root for full license information.

include(../deps/c-shared/configs/azure_iot_build_rules.cmake)
usePermissiveRulesForSamplesAndTests()

set(save_ut ${run_unittests})
set(run_unittests OFF CACHE BOOL "unittests" FORCE)
add_subdirectory(../deps/c-shared/testtools/ctest c-shared/testtools/ctest)
add_subdirectory(../deps/c-shared/testtools/testrunner c-shared/testtools/testrunner)
add_subdirectory(../deps/c-shared/testtools/umock-c c-shared/testtools/umock-c)
if (save_ut)
    set(run_unittests ${save_ut} CACHE BOOL "unittests" FORCE)
endif(save_ut)

set(SHARED_UTIL_REAL_TEST_FOLDER ${SHARED_UTIL_SRC_FOLDER}/../tests/real_test_files CACHE INTERNAL "this is what needs to be included when doing test sources" FORCE)

add_subdirectory(hsm_certificate_props_ut)
add_subdirectory(hsm_tpm_select_ut)
add_subdirectory(certificate_info_ut)
add_subdirectory(edge_hsm_client_x509_int)
add_subdirectory(edge_hsm_tpm_ut)
add_subdirectory(edge_hsm_key_intf_sas_ut)
add_subdirectory(edge_hsm_sas_auth_int)
add_subdirectory(edge_hsm_util_int)
add_subdirectory(edge_hsm_crypto_ut)
add_subdirectory(edge_hsm_crypto_int)
add_subdirectory(edge_openssl_common_ut)
add_subdirectory(edge_openssl_int)
add_subdirectory(edge_openssl_pki_ut)
add_subdirectory(edge_hsm_store_int)
add_subdirectory(hsm_client_tpm_ut)
add_subdirectory(edge_openssl_enc_ut)
add_subdirectory(edge_openssl_enc_int)
# todo enable test for windows after resolving link error
if(NOT WIN32)
    add_subdirectory(edge_hsm_x509_ut)
endif()
