use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const SEED: u64 = 1_234_567_890;

const WORKLOAD_10K_LEN: usize = 4096;
const WORKLOAD_500_LEN: usize = 2048;
const SPAWN_LEN: usize = 1024;
const MOVEMENT_MOD_LEN: usize = 4096;
const MOVEMENT_KIND_LEN: usize = 1024;

const _: () = assert!(
    WORKLOAD_10K_LEN.is_power_of_two()
        && WORKLOAD_500_LEN.is_power_of_two()
        && SPAWN_LEN.is_power_of_two()
        && MOVEMENT_MOD_LEN.is_power_of_two()
        && MOVEMENT_KIND_LEN.is_power_of_two()
);

pub struct RandomTable {
    workload_10k: Box<[i32; WORKLOAD_10K_LEN]>,
    workload_500: Box<[i32; WORKLOAD_500_LEN]>,
    spawn_x: Box<[f32; SPAWN_LEN]>,
    spawn_y: Box<[f32; SPAWN_LEN]>,
    spawn_top: Box<[bool; SPAWN_LEN]>,
    heavy_tick: Box<[bool; SPAWN_LEN]>,
    movement_mod: Box<[f32; MOVEMENT_MOD_LEN]>,
    movement_kind: Box<[u8; MOVEMENT_KIND_LEN]>,
}

impl RandomTable {
    #[inline]
    pub fn workload_10k(&self, index: usize) -> i32 {
        self.workload_10k[index & (WORKLOAD_10K_LEN - 1)]
    }

    #[inline]
    pub fn workload_500(&self, index: usize) -> i32 {
        self.workload_500[index & (WORKLOAD_500_LEN - 1)]
    }

    #[inline]
    pub fn spawn_x(&self, index: usize) -> f32 {
        self.spawn_x[index & (SPAWN_LEN - 1)]
    }

    #[inline]
    pub fn spawn_y(&self, index: usize) -> f32 {
        self.spawn_y[index & (SPAWN_LEN - 1)]
    }

    #[inline]
    pub fn spawn_top(&self, index: usize) -> bool {
        self.spawn_top[index & (SPAWN_LEN - 1)]
    }

    #[inline]
    pub fn heavy_tick(&self, index: usize) -> bool {
        self.heavy_tick[index & (SPAWN_LEN - 1)]
    }

    #[inline]
    pub fn movement_mod(&self, index: usize) -> f32 {
        self.movement_mod[index & (MOVEMENT_MOD_LEN - 1)]
    }

    #[inline]
    pub fn movement_kind(&self, index: usize) -> u8 {
        self.movement_kind[index & (MOVEMENT_KIND_LEN - 1)]
    }
}

static TABLE: OnceLock<RandomTable> = OnceLock::new();

static SPAWN_INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn table() -> &'static RandomTable {
    TABLE.get_or_init(build)
}

pub fn warm_up() {
    let _ = table();
}

pub fn next_spawn_index() -> usize {
    SPAWN_INDEX.fetch_add(1, Ordering::Relaxed)
}

fn build() -> RandomTable {
    let mut rng = GodotRng::new(SEED);

    RandomTable {
        workload_10k: build_table(|| rng.randi_range(1, 9_999)),
        workload_500: build_table(|| rng.randi_range(1, 499)),
        spawn_x: build_table(|| rng.randf_range(-1000.0, 1000.0)),
        spawn_y: build_table(|| rng.randf_range(-800.0, 800.0)),
        spawn_top: build_table(|| rng.randf() < 0.5),
        heavy_tick: build_table(|| rng.randf() < 0.1),
        movement_mod: build_table(|| rng.randf_range(0.2, 2.0)),
        movement_kind: build_table(|| rng.randi_range(0, 4) as u8),
    }
}

fn build_table<T, const N: usize>(mut next: impl FnMut() -> T) -> Box<[T; N]> {
    let mut values = Vec::with_capacity(N);
    values.resize_with(N, &mut next);
    values
        .into_boxed_slice()
        .try_into()
        .unwrap_or_else(|_| unreachable!("the vector is filled with exactly N elements"))
}

struct GodotRng {
    state: u64,
    inc: u64,
}

impl GodotRng {
    const DEFAULT_INC: u64 = 1_442_695_040_888_963_407;

    fn new(seed: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (Self::DEFAULT_INC << 1) | 1,
        };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u32();
        rng
    }

    fn next_u32(&mut self) -> u32 {
        let old = self.state;
        self.state = old
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(self.inc | 1);
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        xorshifted.rotate_right((old >> 59) as u32)
    }

    fn bounded(&mut self, bound: u32) -> u32 {
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let r = self.next_u32();
            if r >= threshold {
                return r % bound;
            }
        }
    }

    fn randi_range(&mut self, from: i32, to: i32) -> i32 {
        let (min, max) = (i64::from(from.min(to)), i64::from(from.max(to)));
        (i64::from(self.bounded((max - min) as u32 + 1)) + min) as i32
    }

    fn randf(&mut self) -> f32 {
        let proto_exp_offset = self.next_u32();
        if proto_exp_offset == 0 {
            return 0.0;
        }
        let significand = (self.next_u32() | 0x8000_0001) as f32;
        significand * 2f32.powi(-32 - proto_exp_offset.leading_zeros() as i32)
    }

    fn randf_range(&mut self, from: f32, to: f32) -> f32 {
        self.randf() * (to - from) + from
    }
}
