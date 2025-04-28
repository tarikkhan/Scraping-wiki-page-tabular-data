# Scraping Wikipedia page using Rust to gather tabular data and saving the data in CSV and JSON format

The wikipedia page url is: https://en.wikipedia.org/wiki/List_of_largest_companies_in_the_United_States_by_revenue

### Rust crates used:
- reqwest
- tokio
- scraper
- serde
- serde_json
- csv

The script extracts data from the first table in that wiki page and saves the data in CSV and JSON format. For json format, the data is organized in two different ways.
