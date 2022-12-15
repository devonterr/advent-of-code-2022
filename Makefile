results: problems data
	rm results
	find problems -name 'day-*' -type d | sed "s/problems\///g" | xargs -I _ sh -c 'echo "=================================" >> results && echo _ >> results && cargo run -p _ >> results'

