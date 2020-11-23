use uuid::Uuid;

pub fn gen_rev_id() -> String {
    let u = Uuid::new_v4();
    u.to_simple_ref().to_string()
}

