# Goal

If you are building a RedSeat Plugin with Rust Include this cargo package in your repository to get all the expected type including json serialization (and optionnally Rusqlite)

# Plugin expected functions and return types

## Video Transcoding
Get capabilities:
`get_convert_capabilities(PluginCredential)` => `RsVideoCapabilities`

`RsVideoCapabilities.maxConcurrentJobs` controls host-side submission concurrency for this plugin:
- omitted/null: RedSeat uses its default limit (currently 1)
- `0`: unlimited concurrent jobs
- `n > 0`: RedSeat submits at most `n` active jobs for this installed plugin

Get remaining credits (optional):
`get_credits()` => `RsRemainingCredits`

Start conversion:
`convert(job: RsVideoTranscodeJobPluginRequest)` => `RsVideoTranscodeJob`

Get conversion status:
`convert_status(jobId: RsVideoTranscodeJobPluginAction)` => `RsVideoTranscodeJob`


Get download request:
`convert_link(jobId: RsVideoTranscodeJobPluginAction)` => `RsRequest`


Cancel job:
`convert_cancel(jobId: RsVideoTranscodeJobPluginAction)` => `RsVideoTranscodeCancelResponse`

## Publishing

Merging a change into `main` runs `.github/workflows/publish.yml`. The workflow tests and packages the crate, checks whether the version in `Cargo.toml` already exists on crates.io, and publishes only a new version.

Publishing uses crates.io trusted publishing with a short-lived GitHub OIDC token. Configure the crate once in its crates.io Trusted Publishers settings with:

- GitHub owner: `neckaros`
- Repository: `rs-plugin-common-interfaces`
- Workflow: `publish.yml`
- Environment: leave empty

The crate must already have an initial manually published version before trusted publishing can be configured. No `CARGO_REGISTRY_TOKEN` GitHub secret is required.
