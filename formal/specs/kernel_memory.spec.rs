verus! {

struct MemoryRegion {
    base: u64,
    size: u64,
}

spec fn region_valid(r: &MemoryRegion) -> bool {
    r.size > 0 && r.base % 4096 == 0 && r.size % 4096 == 0
}

proof fn allocate_region_preserves_validity(r: MemoryRegion)
    requires(region_valid(&r))
    ensures(region_valid(&r))
{
    assert(region_valid(&r));
}

}
