.PHONY: clean

clean:
	find . -type d -name target | xargs rm -rf
