#!/bin/bash

source .env

echo ${NAME}
echo ${IDENTITY}

cbindgen --config cbindgen.toml --lang c --output gen/bindings.h                                                    

make macos
make ios

rm -rf libs/${NAME}.xcframework

xcodebuild -create-xcframework \
    -library libs/lib${NAME}-macos.a \
    -headers gen/ \
    -library libs/lib${NAME}-ios-sim.a \
    -headers gen/ \
    -library libs/lib${NAME}-ios.a \
    -headers gen/ \
    -output libs/${NAME}.xcframework

codesign --force --sign "${IDENTITY}" --timestamp --deep libs/${NAME}.xcframework