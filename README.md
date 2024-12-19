# Atlas CLI Rust Plugin Example
This repository is a template for an example plugin in rust for the [Atlas CLI](https://github.com/Atlas-CLI/Atlas-CLI).

For more information on how Atlas CLI plugins work, please see the [Atlas CLI plugin ocumentation](https://github.com/mongodb/atlas-cli-plugin-example/blob/master/README.md).

**Important note**: This template needs Atlas CLI 1.35.0+ to work (which is not yet released as of today 2024-12-19, this commit is needed: https://github.com/mongodb/mongodb-atlas-cli/commit/0c757933bcb6580141ab7b5f6f06d58a741cb5c1)

## Use the template
This repository is a template repository.

1. Press "Use this template" button to create a new repository from this template.
2. Update `Cargo.toml` with your plugin information.
    - `name`: The name of your plugin.
    - `description`: A short description of your plugin.
    - `repository`: The URL of your GitHub repository.
    - `version`: The version of your plugin.
3. Update `manifest.template.yml` with your plugin information.
    - `name`: The name of your plugin.
    - `description`: A short description of your plugin.
    - `commands`: A list of commands for your plugin. Update this after updating the source code.
4. Update the source code with your plugin logic.

## Create a new release
Once you have updated the source code, you can create a new release.

This template has github actions setup to create a new release when a new tag is created.

This example will describe how to create a new release with version number `1.0.0`.

1. Bump the version number in `Cargo.toml`.
    - update the version number to `1.0.0`
    - commit the change
2. Create a tag which matches the version number.
    - `git tag v1.0.0`
    - `git push --tags`
3. Go to the actions tab of your repository and follow the action which creates a new release.
    - https://github.com/[your-username]/[your-plugin-repository]/actions
4. After a few minutes, you should see a new release in the releases tab.

## Run the plugin
You can run your plugin with the following steps:

1. Install the Atlas CLI plugin.
    - `atlas plugin install [your-username]/[your-plugin-repository]`
2. Verify the plugin is installed.
    - `atlas plugin list`
3. Run the plugin.
    - `atlas [your-plugin-command]`
