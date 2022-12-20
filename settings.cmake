list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/tools/cmake-tool/helpers")
include(application_settings)

correct_platform_strings()

ApplyCommonSimulationSettings("${KernelSel4Arch}")
ApplyCommonReleaseVerificationSettings(${RELEASE} ${VERIFICATION})
