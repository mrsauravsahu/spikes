# notion-integration for expenses

This tool fetches transactions to be parsed from a Notion Database, parses them using Regex and augments the original record back into the Database.
The tool can be run with the following loop to parse all transactions.

```bash
while true; do cargo run; if [ $? -ne 0 ]; then break; fi; done
```