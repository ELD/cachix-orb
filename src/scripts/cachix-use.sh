cachix_use() {
    echo "${PARAM_NAME}"
    echo "${PARAM_EXTRA_NAMES}"
}

ORB_TEST_ENV="bats-core"
if [ "${0#*$ORB_TEST_ENV}" == "$0" ]; then
    cachix_use
fi
