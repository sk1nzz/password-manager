use uuid::Uuid;

struct TotpKey {
    pub id: Uuid,
    pub site_name: String,
    pub secret: Vec<u8>,
}
