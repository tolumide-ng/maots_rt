fn main() {}
use sqlx::{Pool, Postgres};

#[sqlx::test(migrations = false)]
#[maots_rt::human]
pub async fn do_animals_talk(pool: Pool<Postgres>) {
    // fn eleniyan_lawa() {
    //     println!("thank you Jesus!")
    // }

    let name = "tolumide";
    assert_eq!(name, "michael");
}
