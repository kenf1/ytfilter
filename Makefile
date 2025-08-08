.PHONY: reset_tags fmt

reset_tags: #Reset GH tags
	git tag -l | xargs git tag -d

fmt: #Format all c# files
	dotnet format