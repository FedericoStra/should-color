.PHONY: all doc test test-lib test-tests test

all: doc test

doc:
	cargo +nightly rustdoc -- --cfg docsrs

test:
	@cargo hack --feature-powerset test -- --test-threads=1 -q

test-lib:
	@cargo hack --feature-powerset test --lib -- --test-threads=1 -q

test-tests:
	@cargo hack --feature-powerset test --tests -- --test-threads=1 -q

test-doc:
	@cargo hack --feature-powerset test --doc -- --test-threads=1 -q
