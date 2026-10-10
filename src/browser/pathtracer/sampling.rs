//! three-gpu-pathtracer's CPU randomness: the StratifiedSamplesTexture the
//! RANDOM_TYPE 2 shader reads ( StratifiedSamplerCombined over 8-strata, four
//! dimensional samplers ) and the BlueNoiseTexture of its per-pixel offsets
//! ( BlueNoiseGenerator's void-and-cluster ranks ). Both draw from one
//! seeded sequence: the fixture's, which replaces the original's Math.random.

/// The fixture's LCG: Math.imul( seed, 1664525 ) + 1013904223 from 186.
pub(crate) struct Random(u32);
impl Random {
    pub fn new() -> Self {
        Self(186)
    }
    pub fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.
    }
}

/// The host's exponentiation ( `**` ).
fn pow(a: f64, b: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::pow(a, b);
    #[cfg(not(target_arch = "wasm32"))]
    a.powf(b)
}

/// JavaScript's ToInt32 ( `~~x` ).
fn to_int32(x: f64) -> i64 {
    if !x.is_finite() {
        return 0;
    }
    let m = x.trunc().rem_euclid(4294967296.);
    (if m >= 2147483648. { m - 4294967296. } else { m }) as i64
}

/// One StratifiedSampler: its shuffled strata and samples' slot in the
/// combined array.
struct Stratified {
    strata: Vec<u16>,
    index: usize,
    dimensions: usize,
    offset: usize,
}

const STRATA: usize = 8;

/// StratifiedSamplesTexture: `count` rows of `depth` four-dimensional samples.
pub(crate) struct StratifiedSamples {
    pub count: usize,
    pub depth: usize,
    samplers: Vec<Stratified>,
    pub samples: Vec<f32>,
}
impl StratifiedSamples {
    /// The constructor's init( 1, 1, 8 ).
    pub fn new(random: &mut Random) -> Self {
        let mut s = Self {
            count: 0,
            depth: 0,
            samplers: vec![],
            samples: vec![],
        };
        s.init(1, 1, random);
        s
    }
    /// init( count, depth ): new samplers when the size changes, then next().
    pub fn init(&mut self, count: usize, depth: usize, random: &mut Random) {
        if self.depth == depth && self.count == count && !self.samplers.is_empty() {
            return;
        }
        let dims = count * depth;
        let l = STRATA.pow(4);
        self.samplers = (0..dims)
            .map(|i| Stratified {
                strata: (0..l as u16).collect(),
                index: l,
                dimensions: 4,
                offset: i * 4,
            })
            .collect();
        self.samples = vec![0.; dims * 4];
        self.count = count;
        self.depth = depth;
        self.next(random);
    }
    /// sampler.next() for each sampler in order.
    pub fn next(&mut self, random: &mut Random) {
        for s in &mut self.samplers {
            if s.index >= s.strata.len() {
                // shuffle( strata, random )
                for i in (1..s.strata.len()).rev() {
                    let j = (random.next() * (i + 1) as f64).floor() as usize;
                    s.strata.swap(i, j);
                }
                s.index = 0;
            }
            let mut stratum = s.strata[s.index] as usize;
            s.index += 1;
            for i in 0..s.dimensions {
                self.samples[s.offset + i] =
                    (((stratum % STRATA) as f64 + random.next()) / STRATA as f64) as f32;
                stratum /= STRATA;
            }
        }
    }
}

/// BlueNoiseSamples: the binary pattern and its Gaussian energy.
#[derive(Clone)]
struct Samples {
    count: i64,
    size: usize,
    radius: i64,
    lookup: Vec<f32>,
    score: Vec<f32>,
    pattern: Vec<u8>,
}
impl Samples {
    fn new(size: usize) -> Self {
        let sigma: f64 = 1.5;
        let radius = to_int32((10. * 2. * sigma * sigma).sqrt() + 1.);
        let width = 2 * radius + 1;
        let mut lookup = vec![0f32; (width * width) as usize];
        let sigma2 = sigma * sigma;
        for x in -radius..=radius {
            for y in -radius..=radius {
                let index = (radius + y) * width + x + radius;
                let dist2 = (x * x + y * y) as f64;
                lookup[index as usize] = pow(std::f64::consts::E, -dist2 / (2. * sigma2)) as f32;
            }
        }
        Self {
            count: 0,
            size,
            radius,
            lookup,
            score: vec![0.; size * size],
            pattern: vec![0; size * size],
        }
    }
    fn find_void(&self) -> usize {
        let (mut value, mut index) = (f32::INFINITY, usize::MAX);
        for i in 0..self.pattern.len() {
            if self.pattern[i] != 0 {
                continue;
            }
            if self.score[i] < value {
                value = self.score[i];
                index = i;
            }
        }
        index
    }
    fn find_cluster(&self) -> usize {
        let (mut value, mut index) = (f32::NEG_INFINITY, usize::MAX);
        for i in 0..self.pattern.len() {
            if self.pattern[i] != 1 {
                continue;
            }
            if self.score[i] > value {
                value = self.score[i];
                index = i;
            }
        }
        index
    }
    fn update_score(&mut self, x: i64, y: i64, multiplier: f64) {
        let size = self.size as i64;
        let radius = self.radius;
        let width = 2 * radius + 1;
        for px in -radius..=radius {
            for py in -radius..=radius {
                let value = self.lookup[((radius + py) * width + px + radius) as usize] as f64;
                let mut sx = x + px;
                sx = if sx < 0 { size + sx } else { sx % size };
                let mut sy = y + py;
                sy = if sy < 0 { size + sy } else { sy % size };
                let s = (sy * size + sx) as usize;
                self.score[s] = (self.score[s] as f64 + multiplier * value) as f32;
            }
        }
    }
    fn add(&mut self, index: usize) {
        self.pattern[index] = 1;
        let size = self.size;
        let (x, y) = ((index % size) as i64, (index / size) as i64);
        self.update_score(x, y, 1.);
        self.count += 1;
    }
    fn remove(&mut self, index: usize) {
        self.pattern[index] = 0;
        let size = self.size;
        let (x, y) = ((index % size) as i64, (index / size) as i64);
        self.update_score(x, y, -1.);
        self.count -= 1;
    }
    fn invert(&mut self) {
        self.score.fill(0.);
        let size = self.size;
        for i in 0..self.pattern.len() {
            if self.pattern[i] == 0 {
                let (x, y) = ((i % size) as i64, (i / size) as i64);
                self.update_score(x, y, 1.);
                self.pattern[i] = 1;
            } else {
                self.pattern[i] = 0;
            }
        }
    }
}

/// BlueNoiseTexture( 64, 1 ): one channel of BlueNoiseGenerator.generate's
/// ranks over the texel count.
pub(crate) fn blue_noise(size: usize, random: &mut Random) -> Vec<f32> {
    let mut samples = Samples::new(size);
    let points = (size as f64 * size as f64 * 0.1).floor() as usize;
    // fillWithOnes, then shuffleArray.
    for i in 0..points {
        samples.pattern[i] = 1;
    }
    for i in (1..samples.pattern.len()).rev() {
        let j = to_int32((random.next() - 1e-6) * i as f64);
        // A negative index writes a property, not an element, in the original.
        let j = j.max(0) as usize;
        samples.pattern.swap(i, j);
    }
    let initial = samples.pattern.clone();
    samples.pattern.fill(0);
    for (i, &v) in initial.iter().enumerate() {
        if v == 1 {
            samples.add(i);
        }
    }
    loop {
        let cluster = samples.find_cluster();
        samples.remove(cluster);
        let void = samples.find_void();
        if cluster == void {
            samples.add(cluster);
            break;
        }
        samples.add(void);
    }
    let mut dither = vec![0u32; size * size];
    let mut saved = samples.clone();
    let mut rank = samples.count - 1;
    while rank >= 0 {
        let cluster = samples.find_cluster();
        samples.remove(cluster);
        dither[cluster] = rank as u32;
        rank -= 1;
    }
    let total = (size * size) as i64;
    rank = saved.count;
    while (rank as f64) < total as f64 / 2. {
        let void = saved.find_void();
        saved.add(void);
        dither[void] = rank as u32;
        rank += 1;
    }
    saved.invert();
    while rank < total {
        let cluster = saved.find_cluster();
        saved.remove(cluster);
        dither[cluster] = rank as u32;
        rank += 1;
    }
    dither
        .iter()
        .map(|&v| (v as f64 / total as f64) as f32)
        .collect()
}
