use clap::Args;

#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub chunk_type: String,
    pub msg: String,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub chunk_type: String,
}
