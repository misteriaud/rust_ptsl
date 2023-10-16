use test_grpc::PTSL::{
    self,
    SDK::{
        CommandId, CreateNewTracksRequestBody, CreateNewTracksResponseBody, TrackFormat,
        TrackTimebase, TrackType,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PTSL::Client::connect(
        String::from("http://[::1]:31416"),
        String::from("CuetesCuetos.com"),
        String::from("kikikiki.com"),
    )
    .await
    .unwrap();

    println!(
        "{:?}",
        client
            .request::<CreateNewTracksResponseBody>(
                CommandId::CreateNewTracks,
                CreateNewTracksRequestBody {
                    number_of_tracks: 1,
                    track_name: String::from("test"),
                    track_format: TrackFormat::TfStereo.into(),
                    track_type: TrackType::TtAudio.into(),
                    track_timebase: TrackTimebase::TtbSamples.into(),
                },
            )
            .await
    );

    // client.request(command, payload)

    Ok(())
}
