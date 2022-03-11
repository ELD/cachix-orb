setup() {
    source ./src/scripts/cachix-use.sh
}

@test '1: Set up the cache' {
    export PARAM_CACHE="ELD"
    result=$(cachix_use)
    [ "$result" == "Using cache: ELD" ]
}
