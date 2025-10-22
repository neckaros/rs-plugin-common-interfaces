# Goal

If you are building a RedSeat Plugin with Rust Include this cargo package in your repository to get all the expected type including json serialization (and optionnally Rusqlite)

# Plugin expected functions and return types

## Video Transcoding
Get capabilities:
`get_convert_capabilities()` => `RsVideoCapabilities`

Get remaining credits (optional):
`get_credits()` => `RsRemainingCredits`

Start conversion:
`convert(job: RsVideoTranscodeJob)` => `RsVideoTranscodeJob`

Get conversion status:
`convert(jobId: String)` => `RsVideoTranscodeJob`

Cancel job:
`convert(jobId: String)` => `RsVideoTranscodeCancelResponse`