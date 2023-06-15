# count-loc-action
![Lines of Code badge](https://api.badgestore.dev/badge/6f30aef3a19c197c/local) <br>
Counts lines of code using [tokei](https://github.com/XAMPPRocky/tokei)

Generated based on [dbanty/rust-github-action-template](https://github.com/dbanty/rust-github-action-template)

## Inputs
| Name       | Description                                                  | Default  |
|------------|--------------------------------------------------------------|----------|
| `paths`    | The paths inside the github repository you are interested in | `.`      |
| `excluded` | Unix style glob patterns to exclude from the count           | `<none>` |

## Outputs
A list of languages with the format:
```yaml
<language_name>_code # eg 1200000
<language_name>_code_pretty # eg 1 200 000
<language_name>_code_abbreviated # eg 1.2M
<language_name>_blanks # eg 5000
<language_name>_blanks_pretty # eg 5 000
<language_name>_blanks_abbreviated # eg 5.0K
<language_name>_comments # eg 100000
<language_name>_comments_pretty # eg 100 000
<language_name>_comments_abbreviated # eg 100K
```
Language_name can be any of the languages supported by tokei.
Eg `Rust` or `Python`
There is also an entry for `Total` using the same format.
Take a look at the languages [here](https://github.com/XAMPPRocky/tokei/blob/v13.0.0-alpha.0/README.md#supported-languages)

## Example usage
```yaml
name: Count lines of code for the project, and upload to the badge store

on:
  push:
    branches:
      - 'master'

jobs:
  count-loc-and-upload:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3
      - id: loc
        name: Count lines of code
        uses: Sh1nku/count-loc-action@v1
        with:
          excluded: "*.json,*.svg,*.md"
      - uses: Sh1nku/badgestore-update-badge-action@v1
        name: Update badge
        id: badge
        with:
          right-label: ${{ steps.loc.outputs.Total_code_abbreviated }} # <-- Update the badge with the total lines of code
          read-write-key: ${{ secrets.LOC_COUNT_BADGE_RW_KEY }}
      - uses: koki-develop/hub-purge-action@v1 # <-- This only works if your target repo is public, if not github will cache for 24 hours
```

## Limitations
- Only meant for runners on Unix based systems
- All paths must be valid utf-8 strings