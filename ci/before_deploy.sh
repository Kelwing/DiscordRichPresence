# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    cross rustc --bin discord_rp --target $TARGET --release -- -C lto

    # TODO Update this to package the right artifacts
    cp config.yaml $stage/
    if [ -e target/$TARGET/release/discord_rp ]
    then
        cp target/$TARGET/release/discord_rp $stage/
    fi
    if [ -e target/$TARGET/release/discord_rp.exe ]
    then
        cp target/$TARGET/release/discord_rp.exe $stage/
    fi

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main