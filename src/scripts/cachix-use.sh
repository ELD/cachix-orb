cachix_use() {
    echo "Using cache: ${PARAM_CACHE}"
}

ORB_TEST_ENV="bats-core"
if [ "${0#*$ORB_TEST_ENV}" == "$0" ]; then
    cachix_use
fi

