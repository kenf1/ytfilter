.PHONY: reset_tags

reset_tags: #Reset GH tags
	git tag -l | xargs git tag -d