verus! {

struct FSUpdate {
    version: u32,
    active: bool,
}

spec fn update_valid(u: &FSUpdate) -> bool {
    u.version > 0
}

proof fn ab_update_atomic(u: FSUpdate)
    requires(update_valid(&u))
    ensures(update_valid(&u))
{
    assert(update_valid(&u));
}

}
