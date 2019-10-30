pub mod proto {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/p4.v1.rs"));
    }

    pub mod config {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/p4.config.v1.rs"));
        }
    }

    pub mod gnmi {
        include!(concat!(env!("OUT_DIR"), "/gnmi.rs"));
    }

    pub mod gnmi_ext {
        include!(concat!(env!("OUT_DIR"), "/gnmi_ext.rs"));
    }
}

pub mod google {
    pub mod rpc {
        use crate::proto::v1::*;
        include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
    }
}