
#[derive(Serialize, Deserialize)]
pub enum ClientRequestHeader {
    ReadFromSpout { name : String },  
    WriteToSink { name : String },
    CreatePipePair { name : String },
    DestroyPipePair { name : String },
}

#[derive(Serialize, Deserialize)]
pub enum DaemonResponse {
    Confirm,
    Deny
}

