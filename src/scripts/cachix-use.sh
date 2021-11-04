cachix_use() {
    echo "${PARAM_CACHE}"
    for extra_cache_name in ${EXTRA_CACHES}; do
        echo "${extra_cache_name}"
    done
}

ORB_TEST_ENV="bats-core"
if [ "${0#*$ORB_TEST_ENV}" == "$0" ]; then
    cachix_use
fi
