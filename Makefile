.PHONY: dotnet_fmt test

dotnet_fmt: #Format all c# files
	dotnet format

test:
	cargo update && cargo test