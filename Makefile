include .env

macos:
	@cargo build --release --lib --target aarch64-apple-darwin
	@cargo build --release --lib --target x86_64-apple-darwin
	@$(RM) -rf libs/lib${NAME}-macos.a
	@lipo -create -output libs/lib${NAME}-macos.a \
		 target/aarch64-apple-darwin/release/lib${NAME}.a \
		 target/x86_64-apple-darwin/release/lib${NAME}.a
	@codesign --force --sign ${IDENTITY} --timestamp --deep libs/lib${NAME}-macos.a
ios:
	@cargo build --release --lib --target aarch64-apple-ios
	@cargo build --release --lib --target aarch64-apple-ios-sim
	@cargo build --release --lib --target x86_64-apple-ios
	@$(RM) -rf libs/lib${NAME}-ios.a
	@$(RM) -rf libs/lib${NAME}-ios-sim.a
	@cp target/aarch64-apple-ios/release/lib${NAME}.a libs/lib${NAME}-ios.a
	@lipo -create -output libs/lib${NAME}-ios-sim.a \
		 target/aarch64-apple-ios-sim/release/lib${NAME}.a \
		 target/x86_64-apple-ios/release/lib${NAME}.a
	@codesign --force --sign ${IDENTITY} --timestamp --deep libs/lib${NAME}-ios-sim.a