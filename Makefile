.PHONY: all doc test test-lib test-tests test

all: doc test

doc:
	cargo +nightly rustdoc --all-features -- --cfg docsrs

test:
	@cargo hack --optional-deps --feature-powerset test -- --test-threads=1 -q

test-lib:
	@cargo hack --optional-deps --feature-powerset test --lib -- --test-threads=1 -q

test-tests:
	@cargo hack --optional-deps --feature-powerset test --tests -- --test-threads=1 -q

test-doc:
	@cargo hack --optional-deps --feature-powerset test --doc -- --test-threads=1 -q
