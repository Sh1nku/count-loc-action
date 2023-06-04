# count-loc-action

Counts lines of code using [tokei](https://github.com/XAMPPRocky/tokei)

Generated based on [dbanty/rust-github-action-template](https://github.com/dbanty/rust-github-action-template)

## Inputs
| Name       | Description                                                  | Default  |
|------------|--------------------------------------------------------------|----------|
| `paths`    | The paths inside the github repository you are interested in | `.`      |
| `excludes` | Unix style glob patterns to exclude from the count           | `<none>` |
Examples:
- `paths`:
  - `frontend,backend,"Directory with spaces"`
- `excludes`
  - `*.json,node_modules,"Directory with spaces"`

## Outputs
A list of languages with the format:
```
<language_name>_code
<language_name>_blanks
<language_name>_comments
# Eg
Rust_code
Rust_blanks
Rust_comments
```
There is also an entry for `Total` using the same format.

Take a look at the languages [here](https://github.com/XAMPPRocky/tokei/blob/v12.1.2/README.md#supported-languages)

## Example usage
```yaml
name: Count loc
on: [push]
jobs:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    - id: loc
      uses: Sh1nku/count-loc-action@v1
      with:
        excluded: "*.json,*.yaml"
    - name: Verify Total set
      if: |
        steps.loc.outputs.Total_code != null &&
        steps.loc.outputs.Total_blanks != null &&
        steps.loc.outputs.Total_comments != null
      run: |
        echo "Total loc: ${{ steps.loc.outputs.Total_code }}"
    - name: Verify Rust set
      if: |
        steps.loc.outputs.Rust_code != null &&
        steps.loc.outputs.Rust_blanks != null &&
        steps.loc.outputs.Rust_comments != null
      run: |
        echo "Rust was counted: ${{ steps.loc.outputs.Rust_code }}"
```

## Limitations
- Only meant for runners on Unix based systems
- All paths must be valid utf-8 strings