RELEASE_TYPE: patch

Fix hashset and hashmap generation for non-basic element types (e.g. generators using `flat_map`). The non-basic fallback path now uses the Collection protocol (`new_collection`/`collection_more`/`collection_reject`) instead of a manual retry loop, fixing both generation correctness and shrinking quality.
