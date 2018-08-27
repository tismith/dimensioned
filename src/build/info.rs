use super::*;

pub fn new() -> System {
    System {
        name: "Information", module: "info",
        doc_prelude: "Information units",
        base: base_units!(
            BIT: Bit, b, Information;
            S: Second, s, Time;
        ),
        derived: derived_units!(
            BPS: BitsPerSecond = Bit / Second, RateOfInformation;
        ),
        constants: constants!(
            BYTE: Bit = 8.0 * BIT.value_unsafe, "Byte";
            B: Bit = BYTE.value_unsafe, "Byte";
            MB: Bit = MEGA * BYTE.value_unsafe, "Megabyte";
            MIB: Bit = 1.024 * MB.value_unsafe, "Mebibyte";

            MIN: Second = 60.0 * S.value_unsafe, "Minute";
            HR: Second = 60.0 * MIN.value_unsafe, "Hour";
            DAY: Second = 24.0 * HR.value_unsafe, "Day";
        ),
        fmt: true,
        from: Vec::new(),
        refl_blacklist: Vec::new(),
    }
}
