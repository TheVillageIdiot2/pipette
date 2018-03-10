
#[derive(Serialize, Deserialize)]
pub enum ClientRequestHeader {
    ReadFromSpout(String),  
    WriteToSink(String),
    CreatePipePair(String),
    DestroyPipePair(String),
}

#[derive(Serialize, Deserialize)]
pub enum DaemonResponse {
    Confirm,
    Deny(String),
}

