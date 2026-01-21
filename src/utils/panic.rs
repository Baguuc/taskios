pub struct UtilPanics;

impl UtilPanics {
    pub fn server_not_authios() {
        panic!("AUTH SERVER ERROR: auth server returns invalid responses");
    }

    pub fn authios_unavailable() {
        panic!("AUTH SERVER ERROR: auth server shut down");
    }

    pub fn authios_not_inited() {
        panic!(
            "AUTH SERVER ERROR: auth server wasn't inited - it's lacking crucial permissions to run this software"
        );
    }
}
