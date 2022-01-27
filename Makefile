

tree:
	tree --dirsfirst --noreport -I "target|node_modules|DIR.md|*.json|*.lock|*.toml" | sed 's/^/    /' > DIR.md
