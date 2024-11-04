use vault;
fn main() {
    let data = include_bytes!("../replays/vsvsvsvsvsv.rec");
    let replay = vault::Replay::from_bytes(data);
    assert!(replay.is_ok())
}
