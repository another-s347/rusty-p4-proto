pub mod proto {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/p4.v1.rs"));
    }

    pub mod config {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/p4.config.v1.rs"));
        }
    }
}

pub mod google {
    pub mod rpc {
        include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
    }
}
