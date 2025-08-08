.PHONY: reset_tags dotnet_fmt test

reset_tags: #Reset GH tags
	git tag -l | xargs git tag -d

dotnet_fmt: #Format all c# files
	dotnet format

test:
	cargo test