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


## Person types (0.38.0)

`domain::person::PersonType` replaces the Rust `String` in `Person.kind`.
Canonical variants are `Actor`, `Director`, `Writer`, `Producer`, `Creator`,
`Author`, `Family`, `Friends`, and `Singer`. `Custom(String)` supports any other
value. The type is also readable/writable as SQLite TEXT with the `rusqlite` feature.

The JSON field stays a plain string, for example `{"type":"Actor"}` or
`{"type":"custom name"}` inside a person object. Missing types remain omitted,
and `null` remains accepted. Existing string payloads continue to deserialize.

Plugins must map their provider's labels to the appropriate enum variants:
TMDB `Acting` should become `PersonType::Actor`, `Directing` should become
`PersonType::Director`, and book authors should use `PersonType::Author`.
There is no shared alias normalization: `Acting`, `acteur`, and `actor` remain
custom strings unless the plugin maps them. Custom values preserve case and
whitespace. A custom string equal to a canonical spelling decodes as that
canonical variant because their wire representations are identical.

For Rust callers, replace `kind: Some("Actor".to_string())` with
`kind: Some(PersonType::Actor)`. Use `PersonType::Custom(value)` for a custom
label, or `PersonType::from(value)` to recognize canonical spellings and retain
anything else. This is a Rust API breaking change, hence the 0.38.0 version bump;
it does not require changing existing JSON or database string formats.

### Plugin credits

`Relations.peopleDetails` is an optional array of `PersonWithRoles` objects. Each
object flattens the person profile and its optional relationship fields:

```json
{"peopleDetails":[{"id":"tmdb:287","name":"Brad Pitt","modified":0,"added":0,"generated":true,"roles":["Actor"],"characters":["Tyler Durden"],"rank":1}]}
```

`roles`, `characters`, and `rank` describe this title/person relationship, not the
person's global profile. Lower ranks come first; zero is valid. Unknown values
are omitted. Missing fields preserve saved values; empty role/name arrays clear
them. Old person-only array entries still deserialize with absent credit fields.
Rust producers can convert a `Person` with `.into()` for an unadorned credit.

`people_credits()` reads this array and fills missing fields from legacy
`peopleRoles`/`peopleCharacters`/`peopleRanks` maps. Inline values take precedence,
including explicit empty arrays and rank zero. Local `people` references are
included once when they have no detail object. New plugins should emit only the
inline array. Separate maps remain supported for compact title snapshots and
older plugins; they are not required when adding fields to plugin credit objects.
