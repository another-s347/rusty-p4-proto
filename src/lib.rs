pub mod proto {
    pub mod v1 {
        tonic::include_proto!("p4.v1");
    }

    pub mod config {
        pub mod v1 {
            tonic::include_proto!("p4.config.v1");
        }
    }
}

pub mod google {
    pub mod rpc {
        use crate::proto::v1::*;
        tonic::include_proto!("google.rpc");
    }
}
